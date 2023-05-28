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
				Toggle      => self.msg_toggle(),
				Play        => self.msg_play(),
				Pause       => self.msg_pause(),
				Next        => self.msg_next(),
				Last        => self.msg_last(),

				// Audio settings.
				Shuffle     => self.msg_shuffle(),
				Repeat      => self.msg_repeat(),
				Volume(v)   => self.msg_volume(v),
				Seek(f)     => self.msg_seek(f),

				// Queue.
				AddQueueSongFront(s_key)    => self.msg_add_queue_song_front(s_key),
				AddQueueSongBack(s_key)     => self.msg_add_queue_song_back(s_key),
				AddQueueAlbumFront(al_key)  => self.msg_add_queue_album_front(al_key),
				AddQueueAlbumBack(al_key)   => self.msg_add_queue_album_back(al_key),
				AddQueueArtistFront(ar_key) => self.msg_add_queue_artist_front(ar_key),
				AddQueueArtistBack(ar_key)  => self.msg_add_queue_artist_back(ar_key),

				// Queue Index.
				PlayQueueIndex(idx)   => self.msg_play_queue_index(idx),
				RemoveQueueIndex(idx) => self.msg_remove_queue_index(idx),

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
	fn to_source(&self, key: SongKey) -> Option<rodio::Decoder<BufReader<File>>> {
		let path = &self.collection.songs[key].path;
		let file = match File::open(path) {
			Ok(f)  => BufReader::new(f),
			Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((key, anyhow!(e)))); return None; },
		};

		match rodio::Decoder::new(file) {
			Ok(d)  => Some(d),
			Err(e) => { send!(self.to_kernel, AudioToKernel::PathError((key, anyhow!(e)))); None },
		}
	}

	#[inline]
	// Convert an `AlbumKey` to `Vec<Decode>` of its `Song`'s.
	fn to_source_album(&self, key: AlbumKey) -> Option<Vec<rodio::Decoder<BufReader<File>>>> {
		let songs = &self.collection.albums[key].songs;
		let mut vec = Vec::with_capacity(songs.len());

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
		}

		if vec.is_empty() {
			None
		} else {
			Some(vec)
		}
	}

	#[inline]
	// Convert an `ArtistKey` to `Vec<Decode>` of ALL their `Song`'s.
	fn to_source_artist(&self, key: ArtistKey) -> Option<Vec<rodio::Decoder<BufReader<File>>>> {
		let songs = &self.collection.artists[key].songs;
		let mut vec = Vec::with_capacity(songs.len());

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
		}

		if vec.is_empty() {
			None
		} else {
			Some(vec)
		}
	}

	#[inline]
	fn clear_queue_sink(&mut self) {
		let mut state = AUDIO_STATE.write();
		state.queue.clear();
		state.queue_idx = None;
		state.playing   = false;
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
				self.clear_queue_sink();
				return;
			}

			self.sink.skip_one();
			state.increment_queue_idx();
		}
	}

	#[inline(always)]
	fn msg_last(&mut self) {
		trace!("Audio - Last");

		if !self.sink.empty() {
			// Lock state.
			let mut state = AUDIO_STATE.write();

			// Push the previous key back onto the `Sink`.
			if let Some(x) = state.queue_idx {
				// If we're at the beginning of the `Queue`, we have to remove it
				// from the `Sink` and add it again to "reset" the audio track.
				if x == 0 {
					let source = match self.to_source(state.queue[0]) {
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
					let source = match self.to_source(state.queue[x - 1]) {
						Some(s) => s,
						None    => return,
					};
					self.sink.append(source, Some(rodio::Append::Front));
					state.decrement_queue_idx();
				}
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
		trace!("Audio - Volume");
		self.sink.set_volume(volume.f32())
	}

	#[inline(always)]
	fn msg_seek(&mut self, seek: u32) {
		trace!("Audio - Seek");
		let state = AUDIO_STATE.read();
		if let Some(idx) = state.queue_idx {
			// Re-create current `Source ` and seek forward to `seek`.
			let source = match self.to_source(state.queue[idx]) {
				Some(s) => s,
				None    => return,
			}.skip_duration(std::time::Duration::from_secs(seek.into()));

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
	fn msg_add_queue_song_front(&mut self, song: SongKey) {
		trace!("Audio - msg_add_queue_song_front({song:?})");
		if let Some(song) = self.to_source(song) {
			self.sink.append(song, Some(rodio::Append::Front));
		}
	}

	#[inline(always)]
	fn msg_add_queue_song_back(&mut self, song: SongKey) {
		trace!("Audio - msg_add_queue_song_back({song:?})");
		if let Some(song) = self.to_source(song) {
			self.sink.append(song, Some(rodio::Append::Back));
		}
	}

	#[inline(always)]
	fn msg_add_queue_album_front(&mut self, album: AlbumKey) {
		trace!("Audio - msg_add_queue_album_front({album:?})");
		if let Some(songs) = self.to_source_album(album) {
			self.sink.append_bulk(songs, Some(rodio::Append::Front));
		}
	}

	#[inline(always)]
	fn msg_add_queue_album_back(&mut self, album: AlbumKey) {
		trace!("Audio - msg_add_queue_album_back({album:?})");
		if let Some(songs) = self.to_source_album(album) {
			self.sink.append_bulk(songs, Some(rodio::Append::Back));
		}
	}

	#[inline(always)]
	fn msg_add_queue_artist_front(&mut self, artist: ArtistKey) {
		trace!("Audio - msg_add_queue_artist_front({artist:?})");
		if let Some(songs) = self.to_source_artist(artist) {
			self.sink.append_bulk(songs, Some(rodio::Append::Front));
		}
	}

	#[inline(always)]
	fn msg_add_queue_artist_back(&mut self, artist: ArtistKey) {
		trace!("Audio - msg_add_queue_artist_back({artist:?})");
		if let Some(songs) = self.to_source_artist(artist) {
			self.sink.append_bulk(songs, Some(rodio::Append::Back));
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

	//-------------------------------------------------- Drop.
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
