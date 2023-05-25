//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
//use disk::prelude::*;
//use disk::{};
use crate::constants::{
	COLLECTION_VERSION,
	AUDIO_VERSION,
};
use std::sync::{Arc,RwLock};
use crate::collection::Key;
use crate::kernel::{
	RESET_STATE,
	reset::ResetState,
	phase::Phase,
};
use crate::audio::{
	AUDIO_STATE,
	AudioState,
	Volume,
};
use rolock::RoLock;
use benri::{
	debug_panic,
	time::*,
	ops::*,
	sync::*,
	log::*,
};
use disk::{Bincode2,Json,Plain};
use super::{KernelToFrontend, FrontendToKernel};
use crate::{
	ccd::{KernelToCcd, CcdToKernel, Ccd},
	search::{KernelToSearch, SearchToKernel, Search},
	audio::{KernelToAudio, AudioToKernel, Audio},
	watch::{WatchToKernel, Watch},
	collection::{Collection,DUMMY_COLLECTION},
};
use crossbeam::channel::{Sender,Receiver};
use std::path::PathBuf;
use readable::Percent;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicBool;

//---------------------------------------------------------------------------------------------------- Saving.
/// This [`bool`] represents if a [`Collection`] that was
/// recently created is still being written to the disk.
///
/// For performance reasons, when the `Frontend` asks [`Kernel`]
/// for a new [`Collection`], [`Kernel`] will return immediately upon
/// having an in-memory [`Collection`]. However, `shukusai` will
/// (in the background) be saving it disk.
///
/// If your `Frontend` exits around this time, it should probably hang
/// (for a reasonable amount of time) if this is set to `true`, waiting
/// for the [`Collection`] to be saved to disk.
///
/// **This should not be mutated by the `Frontend`.**
pub static SAVING: AtomicBool = AtomicBool::new(false);

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
	ctx: egui::Context,
}

// `Kernel` boot process:
//
//`bios()` ---> `boot_loader()` ---> `kernel()` ---> `init()` ---> `userspace()`
//         |                                           |
//         |--- (bios error occurred, skip to init) ---|
//
impl Kernel {
	//-------------------------------------------------- bios()
	/// [`Kernel`] is started with this.
	///
	/// For more info, see [here.](https://github.com/hinto-janai/festival/src/kernel)
	///
	/// [`Kernel`] will return `crossbeam::channel`'s for communication between it and your frontend.
	///
	/// These channels _should never_ be closed.
	///
	/// This function itself spawns a new thread for [`Kernel`].
	/// ```rust,ignore
	/// // Don't do this.
	/// std::thread::spawn(|| Kernel::spawn());
	///
	/// // Do this.
	/// Kernel::spawn();
	/// ```
	pub fn spawn(ctx: egui::Context) -> Result<(Sender<FrontendToKernel>, Receiver<KernelToFrontend>), std::io::Error> {
		// Create `Kernel` <-> `Frontend` channels.
		let (to_frontend, from_kernel) = crossbeam::channel::unbounded::<KernelToFrontend>();
		let (to_kernel, from_frontend) = crossbeam::channel::unbounded::<FrontendToKernel>();

		// Spawn Kernel.
		std::thread::Builder::new()
			.name("Kernel".to_string())
			.stack_size(16_000_000) // 16MB stack.
			.spawn(move || Self::bios(to_frontend, from_frontend, ctx))?;

		// Return channels.
		Ok((to_kernel, from_kernel))
	}

	fn bios(
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		ctx:           egui::Context,
	) {
		// Initialize lazy statics.
		let _         = Lazy::force(&DUMMY_COLLECTION);
		let beginning = Lazy::force(&crate::logger::INIT_INSTANT);

		#[cfg(feature = "panic")]
		// Set panic hook.
		//
		// If `Kernel` or anyone else `panic!()`'s,
		// we want _everyone_ to exit.
		crate::panic::set_panic_hook();

		debug!("Kernel [1/12] ... entering bios()");

		// Create `ResetState`, send to `Frontend`.
		RESET_STATE.write().disk();

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
				Self::boot_loader(collection, to_frontend, from_frontend, ctx, *beginning);
			},
			// Else, straight to `init` with default flag set.
			Err(e) => {
				warn!("Kernel - Collection{COLLECTION_VERSION} from file error: {}", e);
				Self::init(None, None, to_frontend, from_frontend, ctx, *beginning);
			},
		}
	}

	//-------------------------------------------------- boot_loader()
	fn boot_loader(
		collection:    Collection,
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		ctx:           egui::Context,
		beginning:     std::time::Instant,
	) {
		debug!("Kernel [2/12] ... entering boot_loader()");

		// We successfully loaded `Collection`.
		// Create `CCD` channel + thread and make it convert images.
		debug!("Kernel [3/12] ... spawning CCD");
		let (ccd_send, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();
		let ctx_clone = ctx.clone();
		if let Err(e) = std::thread::Builder::new()
			.name("CCD".to_string())
			.spawn(move || Ccd::convert_art(ccd_send, collection, ctx_clone))
		{
			panic!("Kernel - failed to spawn CCD: {e}");
		}

		// Before hanging on `CCD`, read `AudioState` file.
		// Note: This is a `Result`.
		debug!("Kernel [4/12] ... reading AudioState");
		let state = AudioState::from_file();

		// Set `ResetState` to `Start` + `Art` phase.
		RESET_STATE.write().start();
		RESET_STATE.write().phase = Phase::Art;

		// Wait for `Collection` to be returned by `CCD`.
		debug!("Kernel [5/12] ... waiting on CCD");
		let collection = loop {
			use CcdToKernel::*;
			match recv!(from_ccd) {
				// We received an incremental update.
				// Update the current `ResetState` values to match.
				UpdateIncrement((increment, specific)) => RESET_STATE.write().new_increment(increment, specific),

				// We're onto the next phase in `Collection` creation process.
				// Update the current `ResetState` values to match.
				UpdatePhase((percent, phase)) => RESET_STATE.write().new_phase(percent, phase),

				// `CCD` was successful. We got the new `Collection`.
				NewCollection(collection) => break Some(collection),

				// `CCD` failed, tell `GUI` and give the
				// old `Collection` pointer to everyone
				// and return out of this function.
				Failed(anyhow) => {
					debug_panic!("{anyhow}");

					error!("Kernel - Collection failed: {anyhow}");
					break None;
				},
			}
		};

		// We're done with `CCD`.
		RESET_STATE.write().done();

		// If everything went ok, continue to `kernel` to verify data.
		if let Some(collection) = collection {
			Self::kernel(collection, state, to_frontend, from_frontend, ctx, beginning);
		// Else, skip to `init()`.
		} else {
			Self::init(None, None, to_frontend, from_frontend, ctx, beginning);
		}
	}

	//-------------------------------------------------- kernel()
	fn kernel(
		collection:    Arc<Collection>,
		audio:         Result<AudioState, anyhow::Error>,
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		ctx:           egui::Context,
		beginning:     std::time::Instant,
	) {
		debug!("Kernel [6/12] ... entering kernel()");
		let audio = match audio {
			Ok(audio) => {
				ok_debug!("Kernel - AudioState{AUDIO_VERSION} deserialization");
				audio
			},
			Err(e) => {
				warn!("Kernel - AudioState{AUDIO_VERSION} from file error: {}", e);
				AudioState::new()
			},
		};

		use crate::validate;

		let audio = if validate::key(&collection, audio.key.unwrap_or(Key::zero())) {
			ok!("Kernel - AudioState{AUDIO_VERSION} validation");
			audio
		} else {
			fail!("Kernel - AudioState{AUDIO_VERSION} validation");
			AudioState::new()
		};

		Self::init(Some(collection), Some(audio), to_frontend, from_frontend, ctx, beginning);
	}

	//-------------------------------------------------- init()
	fn init(
		collection:    Option<Arc<Collection>>,
		audio:         Option<AudioState>,
		to_frontend:   Sender<KernelToFrontend>,
		from_frontend: Receiver<FrontendToKernel>,
		ctx:           egui::Context,
		beginning:     std::time::Instant,
	) {
		debug!("Kernel [7/12] ... entering init()");

		// Handle potentially missing `Collection`.
		let collection = match collection {
			Some(c) => { debug!("Kernel [8/12] ... Collection found"); c },
			None    => { debug!("Kernel [8/12] ... Collection NOT found, returning default"); Arc::new(Collection::new()) },
		};

		// Handle potentially missing `AudioState`.
		let audio = match audio {
			Some(a) => { debug!("Kernel [9/12] ... AudioState found"); a }
			None => { debug!("Kernel [9/12] ... AudioState NOT found, returning default"); AudioState::new() },
		};

		// Send `Collection/State` to `Frontend`.
		send!(to_frontend, KernelToFrontend::NewCollection(Arc::clone(&collection)));
		// TODO: Only with `egui` feature flag.
		ctx.request_repaint();

		// Create `To` channels.
		let (to_search, search_recv) = crossbeam::channel::unbounded::<KernelToSearch>();
		let (to_audio,  audio_recv)  = crossbeam::channel::unbounded::<KernelToAudio>();

		// Create `From` channels.
		let (search_send, from_search) = crossbeam::channel::unbounded::<SearchToKernel>();
		let (audio_send,  from_audio)  = crossbeam::channel::unbounded::<AudioToKernel>();
		let (watch_send,  from_watch)  = crossbeam::channel::unbounded::<WatchToKernel>();

		// Create `Kernel`.
		let kernel = Self {
			// Channels.
			to_frontend, from_frontend,
			to_search, from_search,
			to_audio, from_audio,
			from_watch,

			// Data.
			collection,
			ctx,
		};

		// Spawn `Audio`.
		debug!("Kernel [10/12] ... spawning Audio");
		let collection = Arc::clone(&kernel.collection);
		if let Err(e) = std::thread::Builder::new()
			.name("Audio".to_string())
			.spawn(move || Audio::init(collection, audio, audio_send, audio_recv))
		{
			panic!("Kernel - failed to spawn Audio: {e}");
		}

		// Spawn `Search`.
		debug!("Kernel [11/12] ... spawning Search");
		let collection = Arc::clone(&kernel.collection);
		if let Err(e) = std::thread::Builder::new()
			.name("Search".to_string())
			.spawn(move || Search::init(collection, search_send, search_recv))
		{
			panic!("Kernel - failed to spawn Search: {e}");
		}

		// Spawn `Watch`.
		debug!("Kernel [12/12] ... spawning Watch");
		if let Err(e) = std::thread::Builder::new()
			.name("Watch".to_string())
			.spawn(move || Watch::init(watch_send))
		{
			panic!("Kernel - failed to spawn Audio: {e}");
		}

		// We're done, enter main `userspace` loop.
		ok_debug!("Kernel - Entering 'userspace()' ... Took {} seconds total", secs_f32!(beginning));
		Self::userspace(kernel);
	}

}

//---------------------------------------------------------------------------------------------------- Main Kernel loop (userspace)
impl Kernel {
	fn userspace(mut self) {
		ok_debug!("Kernel");

		// Array of our channels we can `select` from.
		let mut select = crossbeam::channel::Select::new();
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
			// Audio settings.
			Shuffle              => send!(self.to_audio, KernelToAudio::Shuffle),
			Repeat               => send!(self.to_audio, KernelToAudio::Repeat),
			Volume(volume)       => send!(self.to_audio, KernelToAudio::Volume(volume)),
			Seek(second)         => send!(self.to_audio, KernelToAudio::Seek(second)),

			// Queue.
			AddQueueSongFront(s_key)    => send!(self.to_audio, KernelToAudio::AddQueueSongFront(s_key)),
			AddQueueSongBack(s_key)     => send!(self.to_audio, KernelToAudio::AddQueueSongBack(s_key)),
			AddQueueAlbumFront(al_key)  => send!(self.to_audio, KernelToAudio::AddQueueAlbumFront(al_key)),
			AddQueueAlbumBack(al_key)   => send!(self.to_audio, KernelToAudio::AddQueueAlbumBack(al_key)),
			AddQueueArtistFront(ar_key) => send!(self.to_audio, KernelToAudio::AddQueueArtistFront(ar_key)),
			AddQueueArtistBack(ar_key)  => send!(self.to_audio, KernelToAudio::AddQueueArtistBack(ar_key)),

		    // Queue Index.
			PlayQueueIndex(q_key)   => send!(self.to_audio, KernelToAudio::PlayQueueIndex(q_key)),
		    RemoveQueueIndex(q_key) => send!(self.to_audio, KernelToAudio::RemoveQueueIndex(q_key)),

			// Collection.
			NewCollection(paths) => self.ccd_mode(paths),
			Search(string)       => send!(self.to_search, KernelToSearch::Search(string)),

			// Exit.
			Exit                 => self.exit(),
		}
	}

	#[inline(always)]
	// We got a message from `Search`.
	fn msg_search(&self, msg: SearchToKernel) {
		use crate::search::SearchToKernel::*;
		match msg {
			Resp(keychain) => send!(self.to_frontend, KernelToFrontend::SearchResp(keychain)),
		}
	}

	#[inline(always)]
	// We got a message from `Audio`.
	fn msg_audio(&self, msg: AudioToKernel) {
		// TODO
//		use crate::audio::AudioToKernel::*;
//		match msg {
//			TimestampUpdate(float) => lockw!(self.state).audio.current_runtime = float,
//			PathError(string)      => send!(self.to_frontend, KernelToFrontend::PathError(string)),
//		}
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
			Shuffle => send!(self.to_audio, KernelToAudio::Shuffle),
			Repeat  => send!(self.to_audio, KernelToAudio::Repeat),
		}
	}

	//-------------------------------------------------- Misc message handling.
	#[inline(always)]
	// Verify the `seek` is valid before sending to `Audio`.
	fn seek(&self, float: f64) {
		// TODO
//		if !lockr!(self.state).audio.playing {
//			return
//		}
//
//		if float <= lockr!(self.state).audio.current_runtime {
//			send!(self.to_audio, KernelToAudio::Play);
//		}
	}

	#[inline(always)]
	// The `Frontend` is exiting, save everything.
	fn exit(&mut self) -> ! {
		// Save `AudioState`.
		match AUDIO_STATE.read().save() {
			Ok(o)  => {
				debug!("Kernel - State save: {o}");
				send!(self.to_frontend, KernelToFrontend::Exit(Ok(())));
			},
			Err(e) => {
				debug_panic!("{e}");
				send!(self.to_frontend, KernelToFrontend::Exit(Err(e.to_string())));
			},
		}

		// Hang forever.
		debug!("Kernel - Entering exit() loop - Total uptime: {}", readable::Time::from(*crate::INIT_INSTANT));
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
		RESET_STATE.write().start();

		// INVARIANT:
		// `GUI` is expected to drop its pointer by itself
		// after requesting the new `Collection`.
		//
		// Drop your pointers.
		send!(self.to_search, KernelToSearch::DropCollection);
		send!(self.to_audio,  KernelToAudio::DropCollection);

		// Create `CCD` channels.
		let (to_ccd,   ccd_recv) = crossbeam::channel::unbounded::<KernelToCcd>();
		let (ccd_send, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();

		// Get old `Collection` pointer.
		let old_collection = Arc::clone(&self.collection);

		// Get `egui::Context` pointer.
		let ctx = self.ctx.clone();

		// Set `ResetState` to `Start` phase.
		RESET_STATE.write().start();

		// Spawn `CCD`.
		if let Err(e) = std::thread::Builder::new()
			.name("CCD".to_string())
			.stack_size(16_000_000) // 16MB stack.
			.spawn(move || Ccd::new_collection(ccd_send, ccd_recv, old_collection, paths, ctx))
		{
			panic!("Kernel - failed to spawn CCD: {e}");
		}

		// Listen to `CCD`.
		self.collection = loop {
			use crate::ccd::CcdToKernel::*;

			// What message did `CCD` send?
			match recv!(from_ccd) {
				// We received an incremental update.
				// Update the current `KernelState.ResetState` values to match.
				UpdateIncrement((increment, specific)) => RESET_STATE.write().new_increment(increment, specific),

				// We're onto the next phase in `Collection` creation process.
				// Update the current `ResetState` values to match.
				UpdatePhase((percent, phase)) => RESET_STATE.write().new_phase(percent, phase),

				// `CCD` was successful. We got the new `Collection`.
				NewCollection(collection) => break collection,

				// `CCD` failed, tell `GUI` and give the
				// old `Collection` pointer to everyone
				// and return out of this function.
				Failed(anyhow) => {
					debug_panic!("{anyhow}");

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
		RESET_STATE.write().done();
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
