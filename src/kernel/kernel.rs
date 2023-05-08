//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use disk::prelude::*;
//use disk::{};
use crate::constants::{
	COLLECTION_VERSION,
	STATE_VERSION,
};
use std::sync::{Arc,RwLock};
use super::state::{
	KernelState,
	DUMMY_KERNEL_STATE,
};
use super::DUMMY_RESET_STATE;
use super::volume::Volume;
use super::reset::ResetState;
use super::phase::Phase;
use rolock::RoLock;
use benri::{
	time::*,
	ops::*,
	sync::*,
	log::*,
	mass_panic,
};
use disk::Bincode2;
use disk::Plain;
use super::{KernelToFrontend, FrontendToKernel};
use crate::{
	ccd::{KernelToCcd, CcdToKernel, Ccd},
	search::{KernelToSearch, SearchToKernel, Search},
	audio::{KernelToAudio, AudioToKernel, Audio},
	watch::{WatchToKernel, Watch},
	collection::{Collection,DUMMY_COLLECTION},
};
use crossbeam_channel::{Sender,Receiver};
use std::path::PathBuf;
use readable::Percent;

//---------------------------------------------------------------------------------------------------- Kernel
/// The [`Kernel`] of `Festival`
///
/// [`Kernel`], the messenger and coordinator.
///
/// [`Kernel`] handles all of `Festival`'s internals and acts
/// as a small & simple interface to all the frontends.
///
/// It is highly recommended to read [`festival-gui`](https://github.com/hinto-janai/festival/festival-gui)'s
/// code and [`Festival`'s internal documentation](https://github.com/hinto-janai/festival/src)
/// if you're creating your own frontend for `Festival`.
pub struct Kernel {
	// Frontend (GUI) Channels.
	to_frontend: Sender<KernelToFrontend>,
	from_frontend: Receiver<FrontendToKernel>,

	// Search Channels.
	to_search: Sender<KernelToSearch>,
	from_search: Receiver<SearchToKernel>,

	// Audio Channels.
	to_audio: Sender<KernelToAudio>,
	from_audio: Receiver<AudioToKernel>,

	// Watch Channel.
	from_watch: Receiver<WatchToKernel>,

	// Data.
	collection: Arc<Collection>,
	state: Arc<RwLock<KernelState>>,
	reset: Arc<RwLock<ResetState>>,
	ctx: egui::Context,
}

// `Kernel` boot process:
//
//`bios()` ---> `boot_loader()` ---> `kernel()` ---> `init()` ---> `userspace()`
//         |                                          |
//         |--- (bios error occured, skip to init) ---|
//
impl Kernel {
	//-------------------------------------------------- bios()
	#[inline(always)]
	/// [`Kernel`] is started with this.
	///
	/// For more info, see [here.](https://github.com/hinto-janai/festival/src/kernel)
	///
	/// You must provide [`Kernel`] with a `crossbeam_channel` between it and your frontend.
	///
	/// This channel _should never_ be closed.
	///
	/// This function itself spawns a new thread for [`Kernel`] and returns the `Result`.
	/// ```rust,ignore
	/// // Don't do this.
	/// std::thread::spawn(|| Kernel::spawn());
	///
	/// // Do this.
	/// Kernel::spawn();
	/// ```
	pub fn spawn(
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		ctx:           egui::Context,
	) -> Result<std::thread::JoinHandle<()>, std::io::Error> {
		std::thread::Builder::new()
			.name("Kernel".to_string())
			.stack_size(16_000_000) // 16MB stack.
			.spawn(move || Self::bios(to_frontend, from_frontend, ctx))
	}

	fn bios(
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		ctx:           egui::Context,
	) {
		// Set `Kernel`'s init time.
		let beginning = now!();

		#[cfg(feature = "panic")]
		// Set panic hook.
		//
		// If `Kernel` or anyone else `panic!()`'s,
		// we want _everyone_ to exit.
		crate::panic::set_panic_hook();

		debug!("Kernel [1/12] ... entering bios()");

		// Initialize lazy statics.
		lazy_static::initialize(&DUMMY_COLLECTION);
		lazy_static::initialize(&DUMMY_KERNEL_STATE);
		lazy_static::initialize(&DUMMY_RESET_STATE);
		lazy_static::initialize(&crate::logger::INIT_INSTANT);

		// Create `ResetState`, send to `Frontend`.
		let reset = ResetState::from_dummy();
		lockw!(reset).disk();
		send!(to_frontend, KernelToFrontend::NewResetState(RoLock::new(&reset)));

		// Attempt to load `Collection` from file.
		debug!("Kernel - Reading Collection{COLLECTION_VERSION} from disk...");
		let now = now!();
		// SAFETY:
		// `Collection` is `memmap`'ed from disk.
		//
		// We (`Kernel`) are the only "entity" that should
		// be touching `collection.bin` at this point.
		//
		// `CCD` saves to `collection.bin`, but that function can
		// only be called after `Kernel` initially loads this one.
		// (we aren't in `userland()` yet, `Kernel` won't respond
		//  to `FrontendToKernel::NewCollection` messages yet)
		//
		// I can't prevent other programs from touching this file
		// although they shouldn't be messing around in other program's
		// data directories anyway.
		match unsafe { Collection::from_file_memmap() } {
			// If success, continue to `boot_loader` to convert
			// bytes to actual usable `egui` images.
			Ok(collection) => {
				ok_debug!("Kernel - Collection{COLLECTION_VERSION} deserialization ... Took {} seconds", secs_f32!(now));
				Self::boot_loader(collection, to_frontend, from_frontend, reset, ctx, beginning);
			},
			// Else, straight to `init` with default flag set.
			Err(e) => {
				warn!("Kernel - Collection{COLLECTION_VERSION} from file error: {}", e);
				Self::init(None, None, to_frontend, from_frontend, reset, ctx, beginning);
			},
		}
	}

	//-------------------------------------------------- boot_loader()
	#[inline(always)]
	fn boot_loader(
		collection:    Collection,
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		reset:         Arc<RwLock<ResetState>>,
		ctx:           egui::Context,
		beginning:     std::time::Instant,
	) {
		debug!("Kernel [2/12] ... entering boot_loader()");

		// We successfully loaded `Collection`.
		// Create `CCD` channel + thread and make it convert images.
		debug!("Kernel [3/12] ... spawning CCD");
		let (ccd_send, from_ccd) = crossbeam_channel::unbounded::<CcdToKernel>();
		let ctx_clone = ctx.clone();
		std::thread::Builder::new()
			.name("CCD".to_string())
			.spawn(move || Ccd::convert_art(ccd_send, collection, ctx_clone));

		// Before hanging on `CCD`, read `KernelState` file.
		// Note: This is a `Result`.
		debug!("Kernel [4/12] ... reading KernelState");
		let state = KernelState::from_file();

		// Set `ResetState` to `Start` + `Art` phase.
		lockw!(reset).start();
		lockw!(reset).phase = Phase::Art;

		// Wait for `Collection` to be returned by `CCD`.
		debug!("Kernel [5/12] ... waiting on CCD");
		let collection = loop {
			use CcdToKernel::*;
			match recv!(from_ccd) {
				// We received an incremental update.
				// Update the current `KernelState.ResetState` values to match.
				UpdateIncrement((increment, specific)) => lockw!(reset).new_increment(increment, specific),

				// We're onto the next phase in `Collection` creation process.
				// Update the current `ResetState` values to match.
				UpdatePhase((percent, phase)) => lockw!(reset).new_phase(percent, phase),

				// `CCD` was successful. We got the new `Collection`.
				NewCollection(collection) => break Some(collection),

				// `CCD` failed, tell `GUI` and give the
				// old `Collection` pointer to everyone
				// and return out of this function.
				Failed(anyhow) => {
					error!("Kernel - Collection failed: {}", anyhow.to_string());
					break None;
				},
			}
		};

		// We're done with `CCD`.
		lockw!(reset).done();

		// If everything went ok, continue to `kernel` to verify data.
		if let Some(collection) = collection {
			Self::kernel(collection, state, to_frontend, from_frontend, reset, ctx, beginning);
		// Else, skip to `init()`.
		} else {
			Self::init(None, None, to_frontend, from_frontend, reset, ctx, beginning);
		}
	}

	//-------------------------------------------------- kernel()
	#[inline(always)]
	fn kernel(
		collection:    Arc<Collection>,
		state:         Result<KernelState, anyhow::Error>,
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		reset:         Arc<RwLock<ResetState>>,
		ctx:           egui::Context,
		beginning:     std::time::Instant,
	) {
		/* TODO: initialize and sanitize collection & misc data */
		debug!("Kernel [6/12] ... entering kernel()");
		let state = match state {
			Ok(state) => {
				ok_debug!("Kernel - State{STATE_VERSION} deserialization");
				state
			},
			Err(e) => {
				warn!("Kernel - State{STATE_VERSION} from file error: {}", e);
				KernelState::new()
			},
		};

		Self::init(Some(collection), Some(state), to_frontend, from_frontend, reset, ctx, beginning);
	}

	//-------------------------------------------------- init()
	#[inline(always)]
	fn init(
		collection:    Option<Arc<Collection>>,
		state:         Option<KernelState>,
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		reset:         Arc<RwLock<ResetState>>,
		ctx:           egui::Context,
		beginning:     std::time::Instant,
	) {
		debug!("Kernel [7/12] ... entering init()");

		// Handle potentially missing `Collection`.
		let collection = match collection {
			Some(c) => { debug!("Kernel [8/12] ... Collection found"); c },
			None    => { debug!("Kernel [8/12] ... Collection NOT found, returning default"); Arc::new(Collection::new()) },
		};

		// Handle potentially missing `State`.
		let state = match state {
			Some(s) => { debug!("Kernel [9/12] ... KernelState found"); Arc::new(RwLock::new(s)) },
			None    => { debug!("Kernel [9/12] ... KernelState NOT found, returning default"); Arc::new(RwLock::new(KernelState::new())) },
		};

		// Send `Collection/State` to `Frontend`.
		send!(to_frontend, KernelToFrontend::NewCollection(Arc::clone(&collection)));
		send!(to_frontend, KernelToFrontend::NewKernelState(RoLock::new(&state)));
		// TODO: Only with `egui` feature flag.
		ctx.request_repaint();

		// Create `To` channels.
		let (to_search, search_recv) = crossbeam_channel::unbounded::<KernelToSearch>();
		let (to_audio,  audio_recv)  = crossbeam_channel::unbounded::<KernelToAudio>();

		// Create `From` channels.
		let (search_send, from_search) = crossbeam_channel::unbounded::<SearchToKernel>();
		let (audio_send,  from_audio)  = crossbeam_channel::unbounded::<AudioToKernel>();
		let (watch_send,  from_watch)  = crossbeam_channel::unbounded::<WatchToKernel>();

		// Create `Kernel`.
		let kernel = Self {
			// Channels.
			to_frontend, from_frontend,
			to_search, from_search,
			to_audio, from_audio,
			from_watch,

			// Data.
			collection,
			state,
			reset,
			ctx,
		};

		// Spawn `Search`.
		debug!("Kernel [10/12] ... spawning Search");
		let collection = Arc::clone(&kernel.collection);
		std::thread::Builder::new()
			.name("Search".to_string())
			.spawn(move || Search::init(collection, search_send, search_recv));

		// Spawn `Audio`.
		debug!("Kernel [11/12] ... spawning Audio");
		let collection = Arc::clone(&kernel.collection);
		let state      = RoLock::new(&kernel.state);
		std::thread::Builder::new()
			.name("Audio".to_string())
			.spawn(move || Audio::init(collection, state, audio_send, audio_recv));

		// Spawn `Watch`.
		debug!("Kernel [12/12] ... spawning Watch");
		std::thread::Builder::new()
			.name("Watch".to_string())
			.spawn(move || Watch::init(watch_send));

		// We're done, enter main `userspace` loop.
		ok_debug!("Kernel - Entering 'userspace()' ... Took {} seconds total", secs_f32!(beginning));
		Self::userspace(kernel);
	}

}

//---------------------------------------------------------------------------------------------------- Main Kernel loop (userspace)
impl Kernel {
	#[inline(always)]
	fn userspace(mut self) {
		ok_debug!("Kernel");

		// Array of our channels we can `select` from.
		let mut select = crossbeam_channel::Select::new();
		// FIXME:
		// These channels need to be cloned first because
		// `select.recv()` requires a `&`, but we need a
		// `&mut` version of `self` later, so instead,
		// we give `select.recv()` a cloned `&`.
		let (frontend, search, audio, watch) = (
			self.from_frontend.clone(),
			self.from_search.clone(),
			self.from_audio.clone(),
			self.from_watch.clone(),
		);
		let (frontend, search, audio, watch) = (
			select.recv(&frontend),
			select.recv(&search),
			select.recv(&audio),
			select.recv(&watch),
		);

		// 1) Hang until message is ready.
		// 2) Receive the message and pass to appropriate function.
		// 3) Loop.
		loop {
			match select.ready() {
				i if i == frontend => self.msg_frontend(recv!(self.from_frontend)),
				i if i == search   => self.msg_search(recv!(self.from_search)),
				i if i == audio    => self.msg_audio(recv!(self.from_audio)),
				i if i == watch    => self.msg_watch(recv!(self.from_watch)),
				_ => error!("Kernel - Received an unknown message"),
			}
		}
	}

	//-------------------------------------------------- Message handling.
	#[inline(always)]
	// We got a message from `GUI`.
	fn msg_frontend(&mut self, msg: FrontendToKernel) {
		use super::FrontendToKernel::*;
		match msg {
			// Audio playback.
			Toggle               => send!(self.to_audio, KernelToAudio::Toggle),
			Play                 => send!(self.to_audio, KernelToAudio::Play),
			Stop                 => send!(self.to_audio, KernelToAudio::Stop),
			Next                 => send!(self.to_audio, KernelToAudio::Next),
			Last                 => send!(self.to_audio, KernelToAudio::Last),
			Seek(float)          => self.seek(float),
			PlayQueueKey(key)    => send!(self.to_audio, KernelToAudio::PlayQueueKey(key)),
			Volume(volume)       => send!(self.to_audio, KernelToAudio::Volume(volume.inner())),
			// Audio settings.
			Shuffle              => flip!(lockw!(self.state).audio.shuffle),
			Repeat               => flip!(lockw!(self.state).audio.repeat),
			// Collection.
			NewCollection(paths) => self.ccd_mode(paths),
			SearchSim(string)    => send!(self.to_search, KernelToSearch::SearchSim(string)),
			// Exit.
			Exit                 => self.exit(),
		}
	}

	#[inline(always)]
	// We got a message from `Search`.
	fn msg_search(&self, msg: SearchToKernel) {
		use crate::search::SearchToKernel::*;
		match msg {
			SearchSim(keychain) => send!(self.to_frontend, KernelToFrontend::SearchSim(keychain)),
		}
	}

	#[inline(always)]
	// We got a message from `Audio`.
	fn msg_audio(&self, msg: AudioToKernel) {
		use crate::audio::AudioToKernel::*;
		match msg {
			TimestampUpdate(float) => lockw!(self.state).audio.current_runtime = float,
			PathError(string)      => send!(self.to_frontend, KernelToFrontend::PathError(string)),
		}
	}

	#[inline(always)]
	// We got a message from `Watch`.
	fn msg_watch(&self, msg: WatchToKernel) {
		use crate::watch::WatchToKernel::*;
		match msg {
			Toggle  => send!(self.to_audio, KernelToAudio::Toggle),
			Play    => send!(self.to_audio, KernelToAudio::Play),
			Stop    => send!(self.to_audio, KernelToAudio::Stop),
			Next    => send!(self.to_audio, KernelToAudio::Next),
			Last    => send!(self.to_audio, KernelToAudio::Last),
			Shuffle => flip!(lockw!(self.state).audio.shuffle),
			Repeat  => flip!(lockw!(self.state).audio.repeat),
		}
	}

	//-------------------------------------------------- Misc message handling.
	#[inline(always)]
	// Verify the `seek` is valid before sending to `Audio`.
	fn seek(&self, float: f64) {
		if !lockr!(self.state).audio.playing {
			return
		}

		if float <= lockr!(self.state).audio.current_runtime {
			send!(self.to_audio, KernelToAudio::Play);
		}
	}

	#[inline(always)]
	// The `Frontend` is exiting, save everything.
	fn exit(&mut self) -> ! {
		// Save `KernelState`.
		match lockr!(self.state).save() {
			Ok(o)  => {
				debug!("Kernel - State save: {o}");
				send!(self.to_frontend, KernelToFrontend::Exit(Ok(())));
			},
			Err(e) => send!(self.to_frontend, KernelToFrontend::Exit(Err(e.to_string()))),
		}

		// Hang forever.
		debug!("Kernel - Entering exit() loop - Total uptime: {}", readable::Time::from(crate::init_instant()));
		loop {
			std::thread::park();
		}
	}

	//-------------------------------------------------- `CCD` Mode.
	#[inline(always)]
	// `GUI` wants a new `Collection`:
	//
	// 1. Enter `CCD` mode
	// 2. Only listen to it
	// 3. (but send updates to `GUI`)
	// 4. Tell everyone to drop the old `Collection` pointer
	// 5. Wait until `CCD` gives the new `Collection`
	// 6. Tell `CCD` to... `Die`
	// 7. Give new `Arc<Collection>` to everyone
	fn ccd_mode(&mut self, paths: Vec<PathBuf>) {
		// Set our `ResetState`.
		lockw!(self.reset).start();

		// INVARIANT:
		// `GUI` is expected to drop its pointer by itself
		// after requesting the new `Collection`.
		//
		// Drop your pointers.
		send!(self.to_search, KernelToSearch::DropCollection);
		send!(self.to_audio,  KernelToAudio::DropCollection);

		// Create `CCD` channels.
		let (to_ccd,   ccd_recv) = crossbeam_channel::unbounded::<KernelToCcd>();
		let (ccd_send, from_ccd) = crossbeam_channel::unbounded::<CcdToKernel>();

		// Get `KernelState` pointer.
		let kernel_state = Arc::clone(&self.state);

		// Get old `Collection` pointer.
		let old_collection = Arc::clone(&self.collection);

		// Get `egui::Context` pointer.
		let ctx = self.ctx.clone();

		// Set `ResetState` to `Start` phase.
		lockw!(self.reset).start();

		// Spawn `CCD`.
		std::thread::Builder::new()
			.name("CCD".to_string())
			.stack_size(16_000_000) // 16MB stack.
			.spawn(move || Ccd::new_collection(ccd_send, ccd_recv, kernel_state, old_collection, paths, ctx));

		// Listen to `CCD`.
		self.collection = loop {
			use crate::ccd::CcdToKernel::*;

			// What message did `CCD` send?
			match recv!(from_ccd) {
				// We received an incremental update.
				// Update the current `KernelState.ResetState` values to match.
				UpdateIncrement((increment, specific)) => lockw!(self.reset).new_increment(increment, specific),

				// We're onto the next phase in `Collection` creation process.
				// Update the current `ResetState` values to match.
				UpdatePhase((percent, phase)) => lockw!(self.reset).new_phase(percent, phase),

				// `CCD` was successful. We got the new `Collection`.
				NewCollection(collection) => break collection,

				// `CCD` failed, tell `GUI` and give the
				// old `Collection` pointer to everyone
				// and return out of this function.
				Failed(anyhow) => {
					send!(self.to_search,   KernelToSearch::NewCollection(Arc::clone(&self.collection)));
					send!(self.to_audio,    KernelToAudio::NewCollection(Arc::clone(&self.collection)));
					send!(self.to_frontend, KernelToFrontend::Failed((Arc::clone(&self.collection), anyhow.to_string())));
					// TODO: Only with `egui` feature flag.
					self.ctx.request_repaint();
					return;
				},
			}
		};

		// We have the `Collection`, tell `CCD` to die.
		send!(to_ccd, KernelToCcd::Die);

		// `CCD` succeeded, send new pointers to everyone.
		send!(self.to_search,   KernelToSearch::NewCollection(Arc::clone(&self.collection)));
		send!(self.to_audio,    KernelToAudio::NewCollection(Arc::clone(&self.collection)));
		send!(self.to_frontend, KernelToFrontend::NewCollection(Arc::clone(&self.collection)));
		// TODO: Only with `egui` feature flag.
		self.ctx.request_repaint();

		// Set our `ResetState`, we're done.
		lockw!(self.reset).done();
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
