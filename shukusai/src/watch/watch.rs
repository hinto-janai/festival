//---------------------------------------------------------------------------------------------------- Use
use log::{error,debug,trace};
use benri::{
	log::*,
	sync::*,
};
use disk::{Empty, Plain};
use crossbeam::channel::{
	Sender,Receiver,
};
use super::msg::WatchToKernel;
use notify::{
	Watcher,
	RecommendedWatcher,
	RecursiveMode,
	Config,
	Event,
};
use crate::signal::*;

//---------------------------------------------------------------------------------------------------- Watch
#[derive(Debug)]
pub(crate) struct Watch {
	// Channel to `Kernel`.
	to_kernel: Sender<WatchToKernel>,
	// Channel from `notify`.
	from_notify: Receiver<Result<Event, notify::Error>>,
}

impl Watch {
	// Kernel starts `Audio` with this.
	pub(crate) fn init(to_kernel: Sender<WatchToKernel>) {
		Self::clean();

		// Get PATH.
		let path = match Play::base_path() {
			Ok(p) => {
				trace!("Watch - Watching PATH: {}", p.display());
				p
			},
			Err(e) => {
				error!("Watch - Failed to get PATH. Signals will be ignored: {e}");
				return;
			},
		};

		// Set up watcher.
		let (tx, from_notify) = crossbeam::channel::unbounded();
		let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
			Ok(w) => w,
			Err(e) => {
				error!("Watch - Failed to create watcher. Signals will be ignored: {e}");
				return;
			},
		};

		// Add PATH to watcher.
		if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
			error!("Watch - Failed to watch. Signals will be ignored: {e}");
			return;
		}

		// Create self.
		let watch = Self {
			to_kernel,
			from_notify,
		};

		Self::main(watch)
	}

	#[inline(always)]
	// Remove all files (if they exist), log errors.
	// Make sure the directory exists.
	fn clean() {
		// Create base directory.
		if let Err(e) = Pause::mkdir() { error!("Watch - Could not create signal folder {e}"); }

		// Clean files.
		if let Err(e) = Toggle::rm()        { error!("Watch - Toggle: {e}"); }
		if let Err(e) = Pause::rm()         { error!("Watch - Pause: {e}"); }
		if let Err(e) = Play::rm()          { error!("Watch - Play: {e}"); }
		if let Err(e) = Next::rm()          { error!("Watch - Next: {e}"); }
		if let Err(e) = Previous::rm()      { error!("Watch - Previous: {e}"); }
		if let Err(e) = Stop::rm()          { error!("Watch - Stop: {e}"); }
		if let Err(e) = Shuffle::rm()       { error!("Watch - Shuffle: {e}"); }
		if let Err(e) = RepeatSong::rm()    { error!("Watch - RepeatSong: {e}"); }
		if let Err(e) = RepeatQueue::rm()   { error!("Watch - RepeatQueue: {e}"); }
		if let Err(e) = RepeatOff::rm()     { error!("Watch - RepeatOff: {e}"); }

		// Content files.
		if let Err(e) = Volume::rm()       { error!("Watch - Volume: {e}"); }
		if let Err(e) = Seek::rm()         { error!("Watch - Seek: {e}"); }
		if let Err(e) = SeekForward::rm()  { error!("Watch - SeekForward: {e}"); }
		if let Err(e) = SeekBackward::rm() { error!("Watch - SeekBackward: {e}"); }
		if let Err(e) = Index::rm()        { error!("Watch - Index: {e}"); }
		if let Err(e) = Clear::rm()        { error!("Watch - Clear: {e}"); }
		if let Err(e) = Skip::rm()         { error!("Watch - Skip: {e}"); }
		if let Err(e) = Back::rm()         { error!("Watch - Back: {e}"); }
//		if let Err(e) = ArtistKey::rm()    { error!("Watch - ArtistKey: {e}"); }
//		if let Err(e) = AlbumKey::rm()     { error!("Watch - AlbumKey: {e}"); }
//		if let Err(e) = SongKey::rm()      { error!("Watch - SongKey: {e}"); }
//		if let Err(e) = Artist::rm()       { error!("Watch - Artist: {e}"); }
//		if let Err(e) = Album::rm()        { error!("Watch - Album: {e}"); }
//		if let Err(e) = Song::rm()         { error!("Watch - Song: {e}"); }
	}

	#[inline(always)]
	fn send(&self, msg: WatchToKernel) {
		debug!("Watch - {msg:?}");
		send!(self.to_kernel, msg);
	}

	fn main(self) {
		ok_debug!("Watch");

		use notify::event::{EventKind,CreateKind};

		loop {
			// Wait for a change in the filesystem.
			// We only care if it was a file creation.
			loop {
				if let Ok(Ok(event)) = self.from_notify.recv() {
					match event.kind {
						// UNIX sends `File`, Windows sends `Any`.
						EventKind::Create(CreateKind::File|CreateKind::Any) => break,
						_ => trace!("Watch - ignoring: {event:?}"),
					}
				}
			}

			// Toggle.
			if Toggle::exists().is_ok() {
				self.send(WatchToKernel::Toggle);
			}

			// Stop/Pause/Play.
			//
			// Priority is `Stop` > `Pause` > `Play`.
			if Stop::exists().is_ok() {
				self.send(WatchToKernel::Stop);
			} else if Pause::exists().is_ok() {
				self.send(WatchToKernel::Pause);
			} else if Play::exists().is_ok() {
				self.send(WatchToKernel::Play);
			}

			// Next/Prev.
			//
			// These two will cancel each-other
			// out if they both exist.
			let next = Next::exists().is_ok();
			let prev = Previous::exists().is_ok();
			if next && prev {
				debug!("Watch - Next & Previous existed, doing nothing");
			} else if next {
				self.send(WatchToKernel::Next);
			} else if prev {
				debug!("Watch - Previous");
				self.send(WatchToKernel::Previous);
			}

			// Shuffle/Repeat.
			if Shuffle::exists().is_ok()     { self.send(WatchToKernel::Shuffle); }
			if RepeatSong::exists().is_ok()  { self.send(WatchToKernel::RepeatSong); }
			if RepeatQueue::exists().is_ok() { self.send(WatchToKernel::RepeatQueue); }
			if RepeatOff::exists().is_ok()   { self.send(WatchToKernel::RepeatOff); }

			// Content signals.
			if let Ok(v) = Volume::from_file()       { self.send(WatchToKernel::Volume(v.0.check())); }
			if let Ok(s) = Skip::from_file()         { self.send(WatchToKernel::Skip(s.0)); }
			if let Ok(s) = Index::from_file()        { self.send(WatchToKernel::Index(s.0.saturating_sub(1))); }
			if let Ok(s) = Clear::from_file()        { self.send(WatchToKernel::Clear(s.0)); }
			if let Ok(s) = Seek::from_file()         { self.send(WatchToKernel::Seek(s.0)); }
			if let Ok(s) = SeekForward::from_file()  { self.send(WatchToKernel::SeekForward(s.0)); }
			if let Ok(s) = SeekBackward::from_file() { self.send(WatchToKernel::SeekBackward(s.0)); }
			if let Ok(s) = Back::from_file()         { self.send(WatchToKernel::Back(s.0)); }

			// Clean folder.
			Self::clean();
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// Tests if all files being created
	// correspond to the correct signals.
	fn signals() {
		if std::env::var("CI").is_err() {
			println!("skipping signals, only for CI");
			return;
		}

		// Logger.
		crate::logger::init_logger(log::LevelFilter::Debug);

		// Set-up fake `Kernel`.
		let (to_kernel, from_watch) = crossbeam::channel::unbounded::<WatchToKernel>();

		// Spawn `Watch`
		std::thread::spawn(move || Watch::init(to_kernel));

		// Wait a bit.
		use benri::sleep;

		// GitHub CI is so insanely slow that signals need more
		// time so `Watch` can delete the old file and reset.
		const S: u64 = 8;
		const T: std::time::Duration = std::time::Duration::from_secs(S);
		sleep!(S);

		// Regular signals.
		Toggle::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Toggle);
		sleep!(S);

		Pause::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Pause);
		sleep!(S);

		Play::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Play);
		sleep!(S);

		Next::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Next);
		sleep!(S);

		Previous::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Previous);
		sleep!(S);

		Stop::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Stop);
		sleep!(S);

		Shuffle::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Shuffle);
		sleep!(S);

		RepeatSong::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::RepeatSong);
		sleep!(S);

		RepeatQueue::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::RepeatQueue);
		sleep!(S);

		RepeatOff::touch().unwrap();
		assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::RepeatOff);
		sleep!(S);

		// Content signals.
		// Should be 0..=100
		for i in [0, 50, 100, 101, u8::MAX] {
			let v = unsafe { crate::audio::Volume::new_unchecked(i) };
			Volume(v).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Volume(crate::audio::Volume::new(i)));
			sleep!(S);
		}

		for i in [0, 5, usize::MAX] {
			Skip(i).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Skip(i));
			sleep!(S);

			Back(i).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Back(i));
			sleep!(S);
		}

		// Should saturate at 0.
		for i in [0, 1, 5, usize::MAX] {
			Index(i).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Index(i.saturating_sub(1)));
			sleep!(S);
		}

		for i in [true, false] {
			Clear(i).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Clear(i));
			sleep!(S);
		}

		for i in [0, 10, u64::MAX] {
			Seek(i).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::Seek(i));
			sleep!(S);

			SeekForward(i).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::SeekForward(i));
			sleep!(S);

			SeekBackward(i).save().unwrap();
			assert_eq!(from_watch.recv_timeout(T).unwrap(), WatchToKernel::SeekBackward(i));
			sleep!(S);
		}
	}
}
