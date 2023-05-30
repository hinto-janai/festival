//---------------------------------------------------------------------------------------------------- Use
use anyhow::{anyhow,bail,ensure};
use log::{error,warn,info,debug,trace};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use benri::{
	sleep,
	flip,
	debug_panic,
	log::*,
	sync::*,
};
use crate::collection::{
	Collection,
	Keychain,
	ArtistKey,
	AlbumKey,
	SongKey,
	QueueKey,
};
use std::sync::{
	Arc,RwLock,
};
use crate::audio::{
	AUDIO_STATE,
	AudioToKernel,
	KernelToAudio,
	AudioState,
	Volume,
};
use crossbeam::channel::{Sender,Receiver};
use rodio::{Sink,OutputStream,OutputStreamHandle,Source};
use std::io::BufReader;
use std::fs::File;

//---------------------------------------------------------------------------------------------------- Audio Init
pub(crate) struct Audio {
	// `rodio`'s audio device type abstractions.
	// This must stay alive for audio to be played.
	stream: OutputStream,
	handle: OutputStreamHandle,

	sink:        Sink,                    // Audio sink, holder and controller of all `Source`'s
	collection:  Arc<Collection>,         // Pointer to `Collection`
	to_kernel:   Sender<AudioToKernel>,   // Channel TO `Kernel`
	from_kernel: Receiver<KernelToAudio>, // Channel FROM `Kernel`
}

impl Audio {
	#[inline(always)]
	// Kernel starts `Audio` with this.
	pub(crate) fn init(
		collection:  Arc<Collection>,
		state:       AudioState,
		to_kernel:   Sender<AudioToKernel>,
		from_kernel: Receiver<KernelToAudio>,
	) {
		trace!("Audio - State:\n{state:#?}");

		const RETRY_SECONDS: u64 = 5;

		// Loop until we can connect to an audio device.
		let (stream, handle) = loop {
			 match OutputStream::try_default() {
				Ok((s, h)) => { debug!("Audio [1/2] - Output device"); break (s, h); },
				Err(e) => {
					warn!("Audio - Output device error: {e} ... retrying in {RETRY_SECONDS} seconds");
				},
			}
			sleep!(RETRY_SECONDS);
		};

		let sink = loop {
			match Sink::try_new(&handle) {
				Ok(s)  => { debug!("Audio [2/2] - Sink"); break s; },
				Err(e) => warn!("Audio - Sink error: {e} ... retrying in {RETRY_SECONDS} seconds"),
			}
			sleep!(RETRY_SECONDS);
		};

		// Re-write global `AudioState`.
		*AUDIO_STATE.write() = state;

		// Restore previous state.

		// Init data.
		let audio = Self {
			stream,
			handle,
			sink,
			collection,
			to_kernel,
			from_kernel,
		};

		// Start `main()`.
		ok_debug!("Audio");
		Self::main(audio);
	}
}

//---------------------------------------------------------------------------------------------------- Main Audio loop.
impl Audio {
	#[inline(always)]
	fn main(mut self) {
		loop {
			// Listen for message.
			let msg = recv!(self.from_kernel);

			use KernelToAudio::*;
			match msg {
				// TODO: Implement.
				// Audio playback.
				Toggle    => self.msg_toggle(),
				Play      => self.msg_play(),
				Pause     => self.msg_pause(),
				Next      => self.msg_next(),
				Previous  => self.msg_previous(),

				// Audio settings.
				Shuffle   => self.msg_shuffle(),
				Repeat    => self.msg_repeat(),
				Volume(v) => self.msg_volume(v),
				Seek(f)   => self.msg_seek(f),

				// Queue.
				AddQueueSongFront((s_key, clear))     => self.msg_add_queue_song(s_key, clear,      rodio::Append::Front),
				AddQueueSongBack((s_key, clear))      => self.msg_add_queue_song(s_key, clear,      rodio::Append::Back),
				AddQueueSongTailFront((s_key, clear)) => self.msg_add_queue_song_tail(s_key, clear, rodio::Append::Front),
				AddQueueSongTailBack((s_key, clear))  => self.msg_add_queue_song_tail(s_key, clear, rodio::Append::Back),
				AddQueueAlbumFront((al_key, clear))   => self.msg_add_queue_album(al_key, clear,    rodio::Append::Front),
				AddQueueAlbumBack((al_key, clear))    => self.msg_add_queue_album(al_key, clear,    rodio::Append::Back),
				AddQueueArtistFront((ar_key, clear))  => self.msg_add_queue_artist(ar_key, clear,   rodio::Append::Front),
				AddQueueArtistBack((ar_key, clear))   => self.msg_add_queue_artist(ar_key, clear,   rodio::Append::Back),

				// Queue Index.
				PlayQueueIndex(idx)   => self.msg_play_queue_index(idx),
				RemoveQueueIndex(idx) => self.msg_remove_queue_index(idx),

				// Audio State.
				RestoreAudioState => self.msg_restore_audio_state(),

				// Collection.
				DropCollection     => self.msg_drop(),
				NewCollection(arc) => self.collection = arc,
			}
		}
	}

	//-------------------------------------------------- Non-msg functions.
	// Error handling gets handled in the `to_source()` functions
	// rather than the `msg_*()` handling functions.

	#[inline]
	// Convert a `SongKey` to `Decode` which implements `Source`.
	fn to_source(&self, key: SongKey) -> Option<(rodio::Decoder<BufReader<File>>, SongKey)> {
		let path = &self.collection.songs[key].path;
		let file = match File::open(path) {
			Ok(f)  => BufReader::new(f),
			Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((key, anyhow!(e)))); return None; },
		};

		match rodio::Decoder::new(file) {
			Ok(d)  => Some((d, key)),
			Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((key, anyhow!(e)))); None },
		}
	}

	#[inline]
	// Convert many `SongKey`'s to `Decode`.
	fn to_source_bulk(&self, iter: std::slice::Iter<'_, SongKey>) -> (Vec<rodio::Decoder<BufReader<File>>>, Vec<SongKey>) {
		let mut vec  = Vec::with_capacity(16);
		let mut keys = Vec::with_capacity(16);

		for key in iter {
			let path = &self.collection.songs[key].path;

			let file = match File::open(path) {
				Ok(f)  => BufReader::new(f),
				Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((*key, anyhow!(e)))); continue; },
			};

			match rodio::Decoder::new(file) {
				Ok(d)  => {
					vec.push(d);
					keys.push(*key);
				},
				Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((*key, anyhow!(e)))); continue },
			}
		}

		(vec, keys)
	}

	#[inline]
	// Convert a `SongKey` to `Vec<Decode>` with `Collection::song_tail()`.
	fn to_source_song_tail(&self, key: SongKey) -> (Vec<rodio::Decoder<BufReader<File>>>, Vec<SongKey>) {
		let mut vec  = Vec::with_capacity(16);
		let mut keys = Vec::with_capacity(16);

		self.collection
			.song_tail(key)
			.filter_map(|k| self.to_source(*k))
			.for_each(|(s, k)| {
				vec.push(s);
				keys.push(k);
			});

		(vec, keys)
	}

	#[inline]
	// Convert an `AlbumKey` to `Vec<Decode>` of its `Song`'s.
	fn to_source_album(&self, key: AlbumKey) -> (Vec<rodio::Decoder<BufReader<File>>>, Vec<SongKey>) {
		let songs    = &self.collection.albums[key].songs;
		let mut vec  = Vec::with_capacity(songs.len());
		let mut keys = Vec::with_capacity(songs.len());

		for song_key in songs {
			let path = &self.collection.songs[song_key].path;
			let file = match File::open(path) {
				Ok(f)  => BufReader::new(f),
				Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((*song_key, anyhow!(e)))); continue },
			};
			let decoder = match rodio::Decoder::new(file) {
				Ok(d)  => d,
				Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((*song_key, anyhow!(e)))); continue },
			};
			vec.push(decoder);
			keys.push(*song_key);
		}

		(vec, keys)
	}

	#[inline]
	// Convert an `ArtistKey` to `Vec<Decode>` of ALL their `Song`'s.
	fn to_source_artist(&self, key: ArtistKey) -> (Vec<rodio::Decoder<BufReader<File>>>, Vec<SongKey>) {
		let songs    = &self.collection.artists[key].songs;
		let mut vec  = Vec::with_capacity(songs.len());
		let mut keys = Vec::with_capacity(songs.len());

		for song_key in songs.iter() {
			let path = &self.collection.songs[song_key].path;
			let file = match File::open(path) {
				Ok(f)  => BufReader::new(f),
				Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((*song_key, anyhow!(e)))); continue },
			};
			let decoder = match rodio::Decoder::new(file) {
				Ok(d)  => d,
				Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((*song_key, anyhow!(e)))); continue },
			};
			vec.push(decoder);
			keys.push(*song_key);
		}

		(vec, keys)
	}

	#[inline]
	// Clear both the `Sink` and `Queue`.
	//
	// The `bool` represents if we should
	// set `playing` to `false` in `AUDIO_STATE`.
	//
	// `false` is used when we want to clear
	// and then append new stuff and want to
	// prevent the API from flickering values.
	//
	// This takes in a lock so we can continue
	// more operations after this without flickering.
	fn clear_queue_sink(
		&mut self,
		keep_playing: bool,
		state: &mut std::sync::RwLockWriteGuard<'_, AudioState>,
	) {
		trace!("Audio - clear_queue_sink({keep_playing})");

		state.queue.clear();
		state.queue_idx = None;
		state.playing   = keep_playing;
		state.song      = None;
		self.sink.clear();
	}

	//-------------------------------------------------- Audio playback.
	#[inline(always)]
	fn msg_toggle(&mut self) {
		trace!("Audio - Toggle");
		if !self.sink.empty() {
			self.sink.toggle();
			flip!(AUDIO_STATE.write().playing);
		}
	}

	#[inline(always)]
	fn msg_play(&mut self) {
		trace!("Audio - Play");
		if !self.sink.empty() {
			self.sink.play();
			AUDIO_STATE.write().playing = true;
		}
	}

	#[inline(always)]
	fn msg_pause(&mut self) {
		trace!("Audio - Pause");
		if !self.sink.empty() {
			self.sink.pause();
			AUDIO_STATE.write().playing = false;
		}
	}

	#[inline(always)]
	fn msg_next(&mut self) {
		trace!("Audio - Next");
		if !self.sink.empty() {
			// Lock state.
			let mut state = AUDIO_STATE.write();
			let queue_idx = state.queue_idx;

			// If we're at the end of the queue, clear.
			if state.at_last_queue_idx() {
				self.clear_queue_sink(false, &mut state);
				return;
			}

			self.sink.skip_one();
			state.increment_queue_idx();
		}
	}

	#[inline(always)]
	fn msg_previous(&mut self) {
		trace!("Audio - Previous");

		if !self.sink.empty() {
			// Lock state.
			let mut state = AUDIO_STATE.write();

			// Push the previous key back onto the `Sink`.
			if let Some(x) = state.queue_idx {
				// If we're at the beginning of the `Queue`, we have to remove it
				// from the `Sink` and add it again to "reset" the audio track.
				if x == 0 {
					let (source, key) = match self.to_source(state.queue[0]) {
						Some(s) => s,
						None    => return,
					};
					// Append it first.
					self.sink.append(source, Some(rodio::Append::Front));
					// Remove the previous version.
					if let Err(e) = self.sink.remove(1) {
						debug_panic!("invalid sink.remove()");
					}
				} else {
					let (source, key) = match self.to_source(state.queue[x - 1]) {
						Some(s) => s,
						None    => return,
					};
					self.sink.append(source, Some(rodio::Append::Front));
					state.decrement_queue_idx();
				}

				self.sink.skip_one();
			}
		}
	}

	//-------------------------------------------------- Audio settings.
	#[inline(always)]
	fn msg_shuffle(&mut self) {
		trace!("Audio - Shuffle");
		todo!();
	}

	#[inline(always)]
	fn msg_repeat(&mut self) {
		trace!("Audio - Repeat");
		todo!();
	}

	#[inline(always)]
	fn msg_volume(&mut self, volume: Volume) {
		trace!("Audio - {volume:?}");
		self.sink.set_volume(volume.f32());
		AUDIO_STATE.write().volume = volume;
	}

	#[inline(always)]
	fn msg_seek(&mut self, seek: u32) {
		trace!("Audio - Seek");
		let state = AUDIO_STATE.read();
		if let Some(idx) = state.queue_idx {
			// Re-create current `Source ` and seek forward to `seek`.
			let (source, key) = match self.to_source(state.queue[idx]) {
				Some((s, k)) => (s.skip_duration(std::time::Duration::from_secs(seek.into())), k),
				None    => return,
			};

			// Re-add current song to front.
			self.sink.append(source, Some(rodio::Append::Front));

			// Remove the old previous song.
			if let Err(e) = self.sink.remove(1) {
				debug_panic!("self.sink.remove(1) fail in msg_seek()");
			}
		}
	}

	//-------------------------------------------------- Queue.
	#[inline(always)]
	fn msg_add_queue_song(&mut self, song: SongKey, clear: bool, append: rodio::Append) {
		trace!("Audio - msg_add_queue_song({song:?}) - {append:?}");

		let mut state = AUDIO_STATE.write();

		if clear {
			self.clear_queue_sink(true, &mut state)
		}

		if let Some((song, key)) = self.to_source(song) {
			self.sink.append(song, Some(append));

			state.queue.push_front(key);
			state.queue_idx = Some(0);
			state.song = Some(key);
		}
	}

	#[inline(always)]
	fn msg_add_queue_song_tail(&mut self, song: SongKey, clear: bool, append: rodio::Append) {
		trace!("Audio - msg_add_queue_song_tail({song:?}) - {append:?}");

		let (song_vec, keys) = self.to_source_song_tail(song);

		let mut state = AUDIO_STATE.write();

		if clear {
			self.clear_queue_sink(true, &mut state)
		}

		if song_vec.len() > 0 {
			self.sink.append_bulk(song_vec, Some(append));


			for k in keys {
				state.queue.push_back(k);
			}

			state.queue_idx = Some(0);
			state.song = Some(song);
		}
	}

	#[inline(always)]
	fn msg_add_queue_album(&mut self, album: AlbumKey, clear: bool, append: rodio::Append) {
		trace!("Audio - msg_add_queue_album({album:?}) - {append:?}");

		let mut state = AUDIO_STATE.write();

		if clear {
			self.clear_queue_sink(true, &mut state)
		}

		let (songs, keys) = self.to_source_album(album);

		if !songs.is_empty() {
			self.sink.append_bulk(songs, Some(append));
		}
	}

	#[inline(always)]
	fn msg_add_queue_artist(&mut self, artist: ArtistKey, clear: bool, append: rodio::Append) {
		trace!("Audio - msg_add_queue_artist({artist:?}) - {append:?}");

		let mut state = AUDIO_STATE.write();

		if clear {
			self.clear_queue_sink(true, &mut state)
		}

		let (songs, keys) = self.to_source_artist(artist);

		if !songs.is_empty() {
			self.sink.append_bulk(songs, Some(append));
		}
	}

	#[inline(always)]
	fn msg_play_queue_index(&mut self, index: usize) {
		trace!("Audio - msg_play_queue_index({index})");
		todo!();
	}

	#[inline(always)]
	fn msg_remove_queue_index(&mut self, index: usize) {
		trace!("Audio - msg_remove_queue_index({index})");
		todo!();
	}

	//-------------------------------------------------- Restore Audio State.
	#[inline(always)]
	fn msg_restore_audio_state(&mut self) {
		trace!("Audio - msg_restore_audio_state()");
		let mut state = AUDIO_STATE.write();

		// INVARIANT:
		// `Kernel` validates `AUDIO_STATE` before handing
		// it off to `Audio` so we should be safe to assume
		// the state holds proper indices into the `Collection`.

		// Volume
		debug!("Audio - Restore ... {:?}", state.volume);
		self.sink.set_volume(state.volume.f32());

		let len     = state.queue.len();
		debug!("Audio - Restore ... queue.len(): {len}");

		if len != 0 {
			if let Some(key) = state.queue_idx {
				let mut vec  = Vec::with_capacity(len);
				let mut keys = Vec::with_capacity(len);

				for (de, key) in state.queue
					.make_contiguous()[key..]
					.iter()
					.filter_map(|key| self.to_source(*key))
				{
					vec.push(de);
					keys.push(key);
				}

				if !vec.is_empty() {
					debug!("Audio - Restore ... appending {} songs to queue", vec.len());
					self.sink.append_bulk(vec, Some(rodio::Append::Front));
				}
			}
		}

		debug!("Audio - Restore ... playing: {}", state.playing);
		if state.playing {
			self.sink.play();
		} else {
			self.sink.pause();
		}
	}

	//-------------------------------------------------- Collection.
	#[inline(always)]
	fn msg_drop(&mut self) {
		// Drop pointer.
		self.collection = Collection::dummy();

		// Hang until we get the new one.
		debug!("Audio - Dropped Collection, waiting...");

		// Ignore messages until it's a pointer.
		loop {
			match recv!(self.from_kernel) {
				KernelToAudio::NewCollection(arc) => {
					ok_debug!("Audio - New Collection received");
					self.collection = arc;
					return;
				},
				_ => {
					debug_panic!("Audio - Incorrect message received");
					error!("Audio - Incorrect message received");
				},
			}
		}
	}
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
