//---------------------------------------------------------------------------------------------------- Use
use log::{info,error,warn,debug,trace};
use crate::constants::{
	COLLECTION_VERSION,
	AUDIO_VERSION,
};
use std::sync::{Arc};
use crate::logger::INIT_INSTANT;
use crate::collection::{
	UNKNOWN_ALBUM,
	UNKNOWN_ALBUM_ID,
	SongKey,
};
use crate::state::{
	Phase,
	RESET_STATE,
	AUDIO_STATE,
	AudioState,
	AudioState0,
	RESETTING,
};
use crate::audio::Volume;
use benri::{
	debug_panic,
	time::*,
	sync::*,
	log::*,
};
use disk::{Bincode2,Json};
use super::{KernelToFrontend, FrontendToKernel};
use crate::{
	ccd::{CcdToKernel, Ccd},
	search::{KernelToSearch, SearchToKernel, Search},
	audio::{KernelToAudio, AudioToKernel, Audio},
	watch::{WatchToKernel, Watch},
	collection::{Collection,DUMMY_COLLECTION},
};
use crossbeam::channel::{Sender,Receiver};
use std::path::PathBuf;

use once_cell::sync::Lazy;


#[cfg(feature = "gui")]
use crate::frontend::egui::{
	gui_context,
	gui_request_update,
};

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
}

// See `GUI`'s Drop impl for reasoning on why this exists.
impl Drop for Kernel {
	fn drop(&mut self) {
		self.exit();
	}
}

// `Kernel` boot process:
//
//`bios()` ---> `boot_loader()` ---> `kernel()` ---> `init()` ---> `userspace()`
//         |                                           |
//         |--- (bios error occurred, skip to init) ---|
//
impl Kernel {
	//-------------------------------------------------- spawn()
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
	///
	/// The `watch` [`bool`] indicates if `Kernel` should spawn a thread
	/// that watches over the `festival/signal` directory for filesystem-based
	/// [`crate::signal`]'s.
	///
	/// The `media_controls` [`bool`] indicates if `Kernel` should plug into
	/// the OS and allow communication via the OS-specific media controls.
	pub fn spawn(
		watch: bool,
		media_controls: bool,
	) -> Result<(Sender<FrontendToKernel>, Receiver<KernelToFrontend>), std::io::Error> {
		// Assert `OnceCell`'s were set.
		#[cfg(feature = "gui")]
		{
			let _ = Lazy::force(&UNKNOWN_ALBUM);

			assert!(crate::frontend::egui::GUI_CONTEXT.get().is_some());

			// INVARIANT:
			// `GUI` must not allocate any textures before this.
			//
			// This allocates unknown texture and makes sure it is index `1`.
			let id = UNKNOWN_ALBUM.texture_id(gui_context());
			if id != UNKNOWN_ALBUM_ID {
				panic!("UNKNOWN_ALBUM id: {id:?}, expected: {UNKNOWN_ALBUM_ID:?}");
			}
		}

		// Create `Kernel` <-> `Frontend` channels.
		let (to_frontend, from_kernel) = crossbeam::channel::unbounded::<KernelToFrontend>();
		let (to_kernel, from_frontend) = crossbeam::channel::unbounded::<FrontendToKernel>();

		// Spawn Kernel.
		std::thread::Builder::new()
			.name("Kernel".to_string())
			.stack_size(16_000_000) // 16MB stack.
			.spawn(move || Self::bios(to_frontend, from_frontend, watch, media_controls))?;

		// Return channels.
		Ok((to_kernel, from_kernel))
	}

	//-------------------------------------------------- bios()
	fn bios(
		to_frontend:    Sender<KernelToFrontend>,
		from_frontend:  Receiver<FrontendToKernel>,
		watch:          bool,
		media_controls: bool,
	) {
		debug!("Kernel Init [1/12] ... entering bios()");

		#[cfg(feature = "panic")]
		// Set panic hook.
		//
		// If `Kernel` or anyone else `panic!()`'s,
		// we want _everyone_ to exit.
		crate::panic::set_panic_hook();

		// Initialize lazy statics.
		let _         = Lazy::force(&DUMMY_COLLECTION);
		let beginning = Lazy::force(&INIT_INSTANT);

		// Create `ResetState`, send to `Frontend`.
		RESET_STATE.write().disk();

		// Attempt to load `Collection` from file.
		debug!("Kernel Init ... Reading Collection{COLLECTION_VERSION} from disk...");
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
				ok_debug!("Kernel Init ... Collection{COLLECTION_VERSION} deserialization ... Took {} seconds", secs_f32!(now));
				Self::boot_loader(collection, to_frontend, from_frontend, *beginning, watch, media_controls);
			},
			// Else, straight to `init` with default flag set.
			Err(e) => {
				warn!("Kernel Init ... Collection{COLLECTION_VERSION} from file error: {}", e);
				Self::init(None, None, to_frontend, from_frontend, *beginning, watch, media_controls);
			},
		}
	}

	//-------------------------------------------------- boot_loader()
	fn boot_loader(
		collection:     Collection,
		to_frontend:    Sender<KernelToFrontend>,
		from_frontend:  Receiver<FrontendToKernel>,
		beginning:      std::time::Instant,
		watch:          bool,
		media_controls: bool,
	) {
		debug!("Kernel Init [2/12] ... entering boot_loader()");

		// We successfully loaded `Collection`.
		// Create `CCD` channel + thread and make it convert images.
		debug!("Kernel Init [3/12] ... spawning CCD");
		let (ccd_send, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();
		if let Err(e) = std::thread::Builder::new()
			.name("CCD".to_string())
			.spawn(move || Ccd::convert_art(ccd_send, collection))
		{
			panic!("Kernel Init [3/12] ... failed to spawn CCD: {e}");
		}

		// Before hanging on `CCD`, read `AudioState` file.
		// Note: This is a `Result`.
		debug!("Kernel Init [4/12] ... reading AudioState");
		let state = AudioState::from_versions(&[
			// SAFETY: memmap is used.
			(AUDIO_VERSION, || unsafe { AudioState::from_file_memmap() }),
			(0,             AudioState0::disk_into),
		]);

		// Set `ResetState` to `Start` + `Art` phase.
		{
			let mut lock = RESET_STATE.write();
			lock.start();
			lock.phase = Phase::Art;
		}

		// Wait for `Collection` to be returned by `CCD`.
		debug!("Kernel Init [5/12] ... waiting on CCD");
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

					error!("Kernel Init ... Collection{COLLECTION_VERSION} failed: {anyhow}");
					break None;
				},
			}
		};

		// We're done with `CCD`.
		RESET_STATE.write().done();

		// If everything went ok, continue to `kernel` to verify data.
		if let Some(collection) = collection {
			Self::kernel(collection, state, to_frontend, from_frontend, beginning, watch, media_controls);
		// Else, skip to `init()`.
		} else {
			Self::init(None, None, to_frontend, from_frontend, beginning, watch, media_controls);
		}
	}

	//-------------------------------------------------- kernel()
	fn kernel(
		collection:     Arc<Collection>,
		audio:          Result<(u8, AudioState), anyhow::Error>,
		to_frontend:    Sender<KernelToFrontend>,
		from_frontend:  Receiver<FrontendToKernel>,
		beginning:      std::time::Instant,
		watch:          bool,
		media_controls: bool,
	) {
		debug!("Kernel Init [6/12] ... entering kernel()");
		let mut audio = match audio {
			Ok((v, s)) if v == AUDIO_VERSION => { info!("Kernel Init ... AudioState{AUDIO_VERSION} from disk"); s },
			Ok((v, s)) => { info!("Kernel Init ... AudioState{v} from disk, converted to AudioState{AUDIO_VERSION}"); s },
			Err(e) => { warn!("Kernel Init ... AudioState failed from disk: {e}, returning default AudioState{AUDIO_VERSION}"); AudioState::new() },
		};

		// Check if `AUDIO_STATE`'s `SongKey` is valid.
		if !crate::validate::song(&collection, audio.song.unwrap_or_else(SongKey::zero)) {
			info!("AudioState ... SongKey invalid, resetting to None");
			audio.song = None;
		}

		// Check if `AUDIO_STATE` indices into itself are in-bounds.
		if let Some(idx) = audio.queue_idx {
			if audio.queue.get(idx).is_none() {
				info!("AudioState ... Queue index invalid, resetting to None");
				audio.queue_idx = None;
			}
		}

		// Check if all of `AUDIO_STATE`'s queue keys are valid.
		if !crate::validate::song(&collection, audio.queue.iter().max().unwrap_or(&SongKey::zero())) {
			info!("AudioState ... Queue contains SongKey that is out-of-bounds, clear()'ing");
			audio.queue.clear();
		}

		Self::init(Some(collection), Some(audio), to_frontend, from_frontend, beginning, watch, media_controls);
	}

	//-------------------------------------------------- init()
	fn init(
		collection:     Option<Arc<Collection>>,
		audio:          Option<AudioState>,
		to_frontend:    Sender<KernelToFrontend>,
		from_frontend:  Receiver<FrontendToKernel>,
		beginning:      std::time::Instant,
		watch:          bool,
		media_controls: bool,
	) {
		debug!("Kernel Init [7/12] ... entering init()");

		// Handle potentially missing `Collection`.
		let collection = match collection {
			Some(c) => { debug!("Kernel Init [8/12] ... Collection found"); c },
			None    => { debug!("Kernel Init [8/12] ... Collection NOT found, returning default"); Arc::new(Collection::new()) },
		};

		// Handle potentially missing `AudioState`.
		let audio = match audio {
			Some(a) => { debug!("Kernel Init [9/12] ... AudioState found"); a }
			None => { debug!("Kernel Init [9/12] ... AudioState NOT found, returning default"); AudioState::new() },
		};

		// Send `Collection/State` to `Frontend`.
		send!(to_frontend, KernelToFrontend::NewCollection(Arc::clone(&collection)));
		#[cfg(feature = "gui")]
		gui_request_update();

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
		};

		// Spawn `Audio`.
		let collection = Arc::clone(&kernel.collection);
		match std::thread::Builder::new()
			.name("Audio".to_string())
			.spawn(move || Audio::init(collection, audio, audio_send, audio_recv, media_controls))
		{
			Ok(_)  => debug!("Kernel Init [10/12] ... spawned Audio"),
			Err(e) => panic!("Kernel Init [10/12] ... failed to spawn Audio: {e}"),
		}

		// Spawn `Search`.
		let collection = Arc::clone(&kernel.collection);
		match std::thread::Builder::new()
			.name("Search".to_string())
			.spawn(move || Search::init(collection, search_send, search_recv))
		{
			Ok(_)  => debug!("Kernel Init [11/12] ... spawned Search"),
			Err(e) => panic!("Kernel Init [11/12] ... failed to spawn Search: {e}"),
		}

		// Spawn `Watch`.
		if watch {
			match std::thread::Builder::new()
				.name("Watch".to_string())
				.spawn(move || Watch::init(watch_send))
			{
				Ok(_)  => debug!("Kernel Init [12/12] ... spawned Watch"),
				Err(e) => fail!("Kernel Init [12/12] ... failed to spawn Watch: {e}"),
			}
		} else {
			debug!("Kernel Init [12/12] ... skipping Watch");
		}

		// We're done, enter main `userspace` loop.
		debug!("Kernel Init ... entering userspace(), took {} seconds", secs_f32!(beginning));
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
				_ => {
					error!("Kernel - Received an unknown message");
					debug_panic!("Kernel - Received an unknown message");
				},
			}
		}
	}

	//-------------------------------------------------- Message handling.
	#[inline(always)]
	// We got a message from `GUI`.
	fn msg_frontend(&mut self, msg: FrontendToKernel) {
		use crate::kernel::FrontendToKernel::*;
		match msg {
			// Audio playback.
			Toggle               => send!(self.to_audio, KernelToAudio::Toggle),
			Play                 => send!(self.to_audio, KernelToAudio::Play),
			Pause                => send!(self.to_audio, KernelToAudio::Pause),
			Next                 => send!(self.to_audio, KernelToAudio::Next),
			Previous(threshold)  => send!(self.to_audio, KernelToAudio::Previous(threshold)),
			Stop                 => send!(self.to_audio, KernelToAudio::Clear(false)),
			// Audio settings.
			Repeat(r)            => send!(self.to_audio, KernelToAudio::Repeat(r)),
			Volume(volume)       => send!(self.to_audio, KernelToAudio::Volume(volume)),
			Seek(tuple)          => send!(self.to_audio, KernelToAudio::Seek(tuple)),

			// Queue.
			AddQueueSong(tuple)     => send!(self.to_audio, KernelToAudio::AddQueueSong(tuple)),
			AddQueueAlbum(tuple)    => send!(self.to_audio, KernelToAudio::AddQueueAlbum(tuple)),
			AddQueueArtist(tuple)   => send!(self.to_audio, KernelToAudio::AddQueueArtist(tuple)),
			Shuffle                 => send!(self.to_audio, KernelToAudio::Shuffle),
			Clear(play)             => send!(self.to_audio, KernelToAudio::Clear(play)),
			Skip(num)               => send!(self.to_audio, KernelToAudio::Skip(num)),
			Back(num)               => send!(self.to_audio, KernelToAudio::Back(num)),

		    // Queue Index.
			SetQueueIndex(q_key)    => send!(self.to_audio, KernelToAudio::SetQueueIndex(q_key)),
		    RemoveQueueRange(tuple) => send!(self.to_audio, KernelToAudio::RemoveQueueRange(tuple)),

			// Audio State.
			RestoreAudioState => send!(self.to_audio, KernelToAudio::RestoreAudioState),

			// Collection.
			NewCollection(paths) => self.ccd_mode(paths),
			CachePath(paths)     => Self::cache_path(paths),
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
		use crate::audio::AudioToKernel::*;
		match msg {
			DeviceError(string)           => send!(self.to_frontend, KernelToFrontend::DeviceError(string.to_string())),
			PlayError(string)             => send!(self.to_frontend, KernelToFrontend::PlayError(string.to_string())),
			SeekError(string)             => send!(self.to_frontend, KernelToFrontend::SeekError(string.to_string())),
			PathError((song_key, string)) => send!(self.to_frontend, KernelToFrontend::PathError((song_key, string.to_string()))),
		}
	}

	#[inline(always)]
	// We got a message from `Watch`.
	fn msg_watch(&self, msg: WatchToKernel) {
		use crate::watch::WatchToKernel::*;
		use crate::audio::{Seek, Repeat};
		match msg {
			Toggle        => send!(self.to_audio, KernelToAudio::Toggle),
			Play          => send!(self.to_audio, KernelToAudio::Play),
			Pause         => send!(self.to_audio, KernelToAudio::Pause),
			Next          => send!(self.to_audio, KernelToAudio::Next),
			Previous      => send!(self.to_audio, KernelToAudio::Previous(None)),
			Stop          => send!(self.to_audio, KernelToAudio::Clear(false)),
			Shuffle       => send!(self.to_audio, KernelToAudio::Shuffle),
			RepeatSong    => send!(self.to_audio, KernelToAudio::Repeat(Repeat::Song)),
			RepeatQueue   => send!(self.to_audio, KernelToAudio::Repeat(Repeat::Queue)),
			RepeatOff     => send!(self.to_audio, KernelToAudio::Repeat(Repeat::Off)),

			// Content signals.
			Volume(v)       => send!(self.to_audio, KernelToAudio::Volume(v)),
			Clear(b)        => send!(self.to_audio, KernelToAudio::Clear(b)),
			Seek(s)         => send!(self.to_audio, KernelToAudio::Seek((Seek::Absolute, s))),
			SeekForward(s)  => send!(self.to_audio, KernelToAudio::Seek((Seek::Forward, s))),
			SeekBackward(s) => send!(self.to_audio, KernelToAudio::Seek((Seek::Backward, s))),
			Index(s)        => send!(self.to_audio, KernelToAudio::SetQueueIndex(s)),
			Skip(s)         => send!(self.to_audio, KernelToAudio::Skip(s)),
			Back(s)         => send!(self.to_audio, KernelToAudio::Back(s)),
//			ArtistKey(k)    => send!(self.to_audio, KernelToAudio::ArtistKey(k)),
//			AlbumKey(k)     => send!(self.to_audio, KernelToAudio::AlbumKey(k)),
//			SongKey(k)      => send!(self.to_audio, KernelToAudio::SongKey(k)),
//			Artist(s)       => send!(self.to_audio, KernelToAudio::Artist(k)),
//			Album(s)        => send!(self.to_audio, KernelToAudio::Album(k)),
//			Song(s)         => send!(self.to_audio, KernelToAudio::Song(k)),
		}
	}

	//-------------------------------------------------- Misc message handling.
	#[inline(always)]
	// The `Frontend` is exiting, save everything.
	fn exit(&mut self) -> ! {
		{
			// Set the saved state's volume
			// to the correct global.
			let volume    =  Volume::new(atomic_load!(crate::state::VOLUME));
			let mut state = AUDIO_STATE.write();
			state.volume  = volume;

			// Save `AudioState`.
			match state.save() {
				Ok(o)  => {
					ok!("Kernel - AudioState{AUDIO_VERSION} save: {o}");
					send!(self.to_frontend, KernelToFrontend::Exit(Ok(())));
				},
				Err(e) => {
					fail!("Kernel - AudioState{AUDIO_VERSION} save: {e}");
					send!(self.to_frontend, KernelToFrontend::Exit(Err(e.to_string())));
				},
			}
		}

		// Hang forever.
		info!("Kernel - Total uptime: {}", readable::Time::from(*crate::logger::INIT_INSTANT));
		loop {
			std::thread::park();
		}
	}

	//-------------------------------------------------- CachePath.
	// A separate thread is responsible for walking these
	// directories since `Kernel` really shouldn't be blocked
	// doing work at any given moment.
	#[inline(always)]
	fn cache_path(mut paths: Vec<PathBuf>) {
		let now = now!();
		debug!("Kernel - Starting CachePath...");

		let cache_path = std::thread::Builder::new().name("CachePath".to_string());

		if let Err(e) = cache_path.spawn(move || {
			paths.retain(|p| p.exists());
			paths.sort();
			paths.dedup();

			let iter = paths
				.into_iter()
				.flat_map(|p| walkdir::WalkDir::new(p).follow_links(true))
				.filter_map(Result::ok)
				.map(walkdir::DirEntry::into_path);

			for path in iter {
				// If we're resetting the `Collection`, we might be doing
				// more harm by thrashing the filesystem, so just exit.
				if atomic_load!(RESETTING) {
					debug!("CachePath - CCD detected, exiting early");
					break;
				}

				trace!("CachePath - {path:?}");
				Ccd::path_infer_audio(&path);
			}

			debug!("CachePath - took {} seconds, bye!", secs_f32!(now));
		}) {
			panic!("Kernel ... failed to spawn CachePath: {e}");
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
		atomic_store!(RESETTING, true);

		// Set our `ResetState`.
		RESET_STATE.write().start();

		// INVARIANT:
		// `GUI` is expected to drop its pointer by itself
		// after requesting the new `Collection`.
		//
		// Drop your pointers.
		send!(self.to_search, KernelToSearch::DropCollection);
		send!(self.to_audio,  KernelToAudio::DropCollection);

		// Create `CCD` channel.
		let (ccd_send, from_ccd) = crossbeam::channel::unbounded::<CcdToKernel>();

		// Give the last ownership of the
		// old `Collection` pointer to `CCD`.
		let old_collection = Arc::clone(&self.collection);
		self.collection    = Collection::dummy();

		// If there is another `CCD` still alive
		// saving, wait for it to finish.
		if crate::state::saving() {
			// Set `ResetState` to `Wait` phase.
			RESET_STATE.write().wait();

			while crate::state::saving() {
				info!("Kernel - Another CCD is still saving, waiting...");
				benri::sleep_millis!(500);
			}
		}

		// Set `ResetState` to `Start` phase.
		RESET_STATE.write().start();

		// Spawn `CCD`.
		if let Err(e) = std::thread::Builder::new()
			.name("CCD".to_string())
			.stack_size(16_000_000) // 16MB stack.
			.spawn(move || Ccd::new_collection(ccd_send, old_collection, paths))
		{
			panic!("Kernel - failed to spawn CCD: {e}");
		}

		// Listen to `CCD`.
		let (error, collection) = loop {
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
				NewCollection(collection) => break (None, collection),

				// `CCD` failed, tell `GUI` and give the
				// old `Collection` pointer to everyone
				// and return out of this function.
				Failed(anyhow) => break (Some(anyhow), Collection::dummy()),
			}
		};

		self.collection = collection;

		// Send new pointers to everyone.
		send!(self.to_audio,    KernelToAudio::NewCollection(Arc::clone(&self.collection)));
		send!(self.to_search,   KernelToSearch::NewCollection(Arc::clone(&self.collection)));
		if let Some(e) = error {
			send!(self.to_frontend, KernelToFrontend::Failed((Arc::clone(&self.collection), e.to_string())));
		} else {
			send!(self.to_frontend, KernelToFrontend::NewCollection(Arc::clone(&self.collection)));
		}

		#[cfg(feature = "gui")]
		gui_request_update();

		// Set our `ResetState`, we're done.
		RESET_STATE.write().done();
		atomic_store!(RESETTING, false);
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
