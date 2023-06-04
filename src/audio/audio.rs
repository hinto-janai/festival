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
	Append,
};
use crossbeam::channel::{Sender,Receiver};
use std::io::BufReader;
use std::fs::File;
use symphonia::core::{
	probe::{ProbeResult, Hint},
	meta::MetadataOptions,
	io::MediaSourceStream,
	formats::{FormatReader, FormatOptions, Track},
	codecs::{Decoder, DecoderOptions},
	audio::{Signal,AudioBuffer,AsAudioBufferRef},
};
use std::time::Duration;
use crate::audio::output::{
	AudioOutput,Output,
};

//---------------------------------------------------------------------------------------------------- Constants
// If the audio device is not connected, how many seconds
// should we wait before trying to connect again?
const RETRY_SECONDS: u64 = 5;

// In the audio packet demux/decode loop, how much time
// should we spare to listen for `Kernel` messages?
//
// If too high, we won't process the audio
// fast enough and it will end up choppy.
//
// If too low, we will be spinning the CPU more.
const RECV_TIMEOUT: Duration = Duration::from_millis(10);

// If there are multiple messages in the queue, how
// many additional ones should we process before continuing on?
//
// This shouldn't be too high, otherwise we'll be
// delaying the audio demux/decode code and audio
// will be choppy.
//
// These messages don't wait additional time with `RECV_TIMEOUT`,
// they either are there or we break and continue with audio.
const MSG_PROCESS_LIMIT: u8 = 6;

//---------------------------------------------------------------------------------------------------- Audio Init
pub(crate) struct Audio {
	// A handle to the audio output device.
	output: AudioOutput,

	// The current song.
	current: Option<AudioReader>,

	// A local copy of `AUDIO_STATE`.
	// This exists so we don't have to lock
	// in the loop every time we want to read values.
	//
	// It will get updated when we receive
	// a message, e.g, to change the volume.
	state: AudioState,

	collection:  Arc<Collection>,         // Pointer to `Collection`
	to_kernel:   Sender<AudioToKernel>,   // Channel TO `Kernel`
	from_kernel: Receiver<KernelToAudio>, // Channel FROM `Kernel`
}

// This is a simple container for some
// metadata needed to be held while
// playing back a song.
pub(crate) struct AudioReader {
	// The current song.
	reader: Box<dyn FormatReader>,
	// The current song's decoder.
	decoder: Box<dyn Decoder>,
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

		// Loop until we can connect to an audio device.
		let output = loop {
			 match AudioOutput::dummy() {
				Ok(o) => { debug!("Audio - Output device"); break o; },
				Err(e) => {
					warn!("Audio - Output device error: {e:?} ... retrying in {RETRY_SECONDS} seconds");
				},
			}
			sleep!(RETRY_SECONDS);
		};

		// Re-write global `AudioState`.
		let queue_len = state.queue.len();
		let audio_state = state.clone();
		*AUDIO_STATE.write() = audio_state;

		// Init data.
		let audio = Self {
			output,
			current: None,
			state,
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
	//-------------------------------------------------- The "main" loop.
	#[inline(always)]
	fn main(mut self) {
		loop {
			//------ Kernel message.
			// If we're playing something, only listen for `Kernel` messages for a few millis.
			if self.state.playing {
				if let Ok(msg) = self.from_kernel.recv_timeout(RECV_TIMEOUT) {
					self.kernel_msg(msg);

					// If there's more messages, process them before continuing on.
					for _ in 0..MSG_PROCESS_LIMIT {
						if let Ok(msg) = self.from_kernel.try_recv() {
							self.kernel_msg(msg);
						} else {
							break;
						}
					}
				}
			// Else, sleep on `Kernel`.
			} else {
				self.kernel_msg(recv!(self.from_kernel));
			}

			// If the above message didn't set `playing` state, that
			// means we don't need to continue below to decode audio.
			if !self.state.playing {
				continue;
			}

			//------ Audio decoding & demuxing.
			if let Some(audio_reader) = &mut self.current {
				let reader  = &mut audio_reader.reader;
				let decoder = &mut audio_reader.decoder;

				// Decode and play the packets belonging to the selected track.
				// Get the next packet from the format reader.
				let packet = match reader.next_packet() {
					Ok(packet) => packet,
//					Err(err) => break Err(err),
					Err(err) => todo!(),
				};

				// Decode the packet into audio samples.
				match decoder.decode(&packet) {
					Ok(decoded) => {
						// Get the audio buffer specification. This is a description of the decoded
						// audio buffer's sample format and sample rate.
						let spec = *decoded.spec();

						// Get the capacity of the decoded buffer. Note that this is capacity, not
						// length! The capacity of the decoded buffer is constant for the life of the
						// decoder, but the length is not.
						let duration = decoded.capacity() as u64;

						if spec != self.output.spec || duration != self.output.duration {
							// TODO
							self.output = AudioOutput::try_open(spec, duration).unwrap();
						}

						// Write the decoded audio samples to the audio output if the presentation timestamp
						// for the packet is >= the seeked position (0 if not seeking).
						// TODO
//						if packet.ts() >= play_opts.seek_ts {
//						if packet.ts() >= 0 {

							// Convert the buffer to `f32` and multiply
							// it by `0.0..1.0` to set volume levels.
							let mut buf = AudioBuffer::<f32>::new(duration, spec);
							decoded.convert(&mut buf);
							buf.transform(|f| f * self.state.volume.f32());

							// Write to audio output device.
							self.output.write(buf.as_audio_buffer_ref()).unwrap()
//						}
					}
					Err(symphonia::core::errors::Error::DecodeError(err)) => {
						// Decode errors are not fatal. Print the error message and try to decode the next
						// packet as usual.
						warn!("decode error: {}", err);
					}
//					Err(err) => break Err(err),

					// We're done playing audio.
					// This "end of stream" error is currently the only way
					// a FormatReader can indicate the media is complete.
					Err(symphonia::core::errors::Error::IoError(err)) => {
						break;
					},
					Err(err) => todo!(),
				}

			// If `playing == true`, but our state doesn't
			// have a Some(song), then something went wrong.
			} else {
				todo!();
			}

		//------ End of `loop {}`.
		}
	}

	//-------------------------------------------------- Kernel message.
	#[inline(always)]
	fn kernel_msg(&mut self, msg: KernelToAudio) {
		use KernelToAudio::*;
		match msg {
			// Audio playback.
			Toggle    => self.msg_toggle(),
			Play      => self.msg_play(),
			Pause     => self.msg_pause(),
			Next      => self.msg_next(),
			Previous  => self.msg_previous(),
//
//			// Audio settings.
			Shuffle(s) => self.msg_shuffle(s),
			Repeat(r)  => self.msg_repeat(r),
			Volume(v)  => self.msg_volume(v),
//			Seek(f)   => self.msg_seek(f),
//
//			// Queue.
//			AddQueueSongFront((s_key, clear))     => self.msg_add_queue_song(s_key, clear,      Append::Front),
//			AddQueueSongBack((s_key, clear))      => self.msg_add_queue_song(s_key, clear,      Append::Back),
			AddQueueAlbum((al_key, append, clear))   => self.msg_add_queue_album(al_key, append, clear),
//			AddQueueAlbumBack((al_key, clear))    => self.msg_add_queue_album(al_key, clear,    Append::Back),
//			AddQueueArtistFront((ar_key, clear))  => self.msg_add_queue_artist(ar_key, clear,   Append::Front),
//			AddQueueArtistBack((ar_key, clear))   => self.msg_add_queue_artist(ar_key, clear,   Append::Back),
			Skip(num) => self.msg_skip(num),
//
//			// Queue Index.
//			PlayQueueIndex(idx)   => self.msg_play_queue_index(idx),
//			RemoveQueueIndex(idx) => self.msg_remove_queue_index(idx),
//
//			// Audio State.
			RestoreAudioState => self.msg_restore_audio_state(),

			// Collection.
			DropCollection     => self.drop_collection(),
			NewCollection(arc) => self.collection = arc,

			_ => todo!(),
		}
	}

	//-------------------------------------------------- Non-msg functions.
	// Error handling gets handled in these functions
	// rather than the caller (it gets called everywhere).

	#[inline]
	// Convert a `SongKey` to a playable object.
	fn to_reader(&self, key: SongKey) -> Option<Box<dyn FormatReader>> {
		// Get `Song` PATH.
		let path = &self.collection.songs[key].path;

		// Try to the extension hint.
		let mut hint = Hint::new();
		if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
			hint.with_extension(ext);
		}

		// Some misc `symphonia` options.
		let format_opts = FormatOptions {
			enable_gapless: true,
			..Default::default()
		};
		let metadata_opts: MetadataOptions = Default::default();

		// Open file.
		let file = match File::open(path) {
			Ok(f)  => f,
			Err(e) => {
				fail!("Audio - path error: {e}");
				send!(self.to_kernel, AudioToKernel::PathError((key, anyhow!(e))));
				return None;
			},
		};

		// Attempt probe.
		let mss = MediaSourceStream::new(Box::new(file), Default::default());
		match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
			Ok(o)  => Some(o.format),
			Err(e) => {
				fail!("Audio - probe error: {e}");
				send!(self.to_kernel, AudioToKernel::PathError((key, anyhow!(e))));
				None
			},
		}
	}

	// 1. Takes in a playable song from the above function
	// 2. Creates a decoder for the reader
	// 3. Sets our current state with it
	//
	// This does nothing on error.
	fn set_current(&mut self, reader: Box<dyn FormatReader>) {
		// Select the first track with a known codec.
		let track = match reader
			.tracks()
			.iter()
			.find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
		{
			Some(t) => t,
			None => {
				todo!();
			},
		};

		// Create a decoder for the track.
		let decoder_opts = DecoderOptions { verify: false, ..Default::default() };
		let decoder = match symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts) {
			Ok(d) => d,
			Err(e) => {
				todo!();
			},
		};

		self.current = Some(AudioReader {
			reader,
			decoder,
		});
	}

	// Convenience function that combines the above 2 functions.
	// Does nothing on error.
	fn set(&mut self, song: SongKey) {
		if let Some(reader) = self.to_reader(song) {
			self.set_current(reader);
		}
	}

	#[inline]
	// Clears the `Queue`.
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
	fn clear(
		&mut self,
		keep_playing: bool,
		state: &mut std::sync::RwLockWriteGuard<'_, AudioState>,
	) {
		trace!("Audio - clear_queue_sink({keep_playing})");

		state.queue.clear();
		state.queue_idx = None;
		state.playing   = keep_playing;
		state.song      = None;

		self.state.playing = keep_playing;
	}

	//-------------------------------------------------- Audio playback.
	#[inline(always)]
	fn msg_toggle(&mut self) {
		trace!("Audio - Toggle");
		if self.current.is_some() {
			flip!(self.state.playing);
			flip!(AUDIO_STATE.write().playing);
		}
	}

	#[inline(always)]
	fn msg_play(&mut self) {
		trace!("Audio - Play");
		if self.current.is_some() {
			self.state.playing = true;
			AUDIO_STATE.write().playing = true;
		}
	}

	#[inline(always)]
	fn msg_pause(&mut self) {
		trace!("Audio - Pause");
		if self.current.is_some() {
			self.state.playing = false;
			AUDIO_STATE.write().playing = false;
		}
	}

	#[inline(always)]
	fn msg_next(&mut self) {
		trace!("Audio - Next");

		// Lock state.
		let mut state = AUDIO_STATE.write();

		if !state.queue.is_empty() {
			// If we're at the end of the queue, clear.
			if state.at_last_queue_idx() {
				self.clear(false, &mut state);
				return;
			}

			let key = state.next();
			self.set(key);
		}
	}

	#[inline(always)]
	fn msg_previous(&mut self) {
		trace!("Audio - Previous");
		// Lock state.
		let mut state = AUDIO_STATE.write();

		if !state.queue.is_empty() {
			// If we're at the end of the queue, clear.
			let key = state.prev();
			self.set(key);
		}
	}

	#[inline(always)]
	fn msg_skip(&mut self, num: usize) {
		trace!("Audio - msg_skip({num})");

		// Lock state.
		let mut state = AUDIO_STATE.write();

		if !state.queue.is_empty() {
			if let Some(index) = state.queue_idx {
				let new_index = index + num;

				if let Some(key) = state.queue.get(new_index) {
					self.set(*key);
					state.song      = Some(*key);
					state.queue_idx = Some(new_index);
				}
			}
		}
	}

	//-------------------------------------------------- Audio settings.
	#[inline(always)]
	fn msg_shuffle(&mut self, shuffle: crate::audio::shuffle::Shuffle) {
		trace!("Audio - Shuffle");
		todo!();
	}

	#[inline(always)]
	fn msg_repeat(&mut self, repeat: crate::audio::repeat::Repeat) {
		trace!("Audio - Repeat");
		todo!();
	}

	#[inline(always)]
	fn msg_volume(&mut self, volume: Volume) {
		trace!("Audio - {volume:?}");
		self.state.volume = volume;
		AUDIO_STATE.write().volume = volume;
	}

//	#[inline(always)]
//	fn msg_seek(&mut self, seek: u32) {
//		trace!("Audio - Seek");
//		let state = AUDIO_STATE.read();
//		if let Some(idx) = state.queue_idx {
//			// Re-create current `Source ` and seek forward to `seek`.
//			let (source, key) = match self.to_source(state.queue[idx]) {
//				Some((s, k)) => (s.skip_duration(std::time::Duration::from_secs(seek.into())), k),
//				None    => return,
//			};
//
//			// Re-add current song to front.
//			self.sink.append(source, Some(Append::Front));
//
//			// Remove the old previous song.
//			if let Err(e) = self.sink.remove(1) {
//				debug_panic!("self.sink.remove(1) fail in msg_seek()");
//			}
//		}
//	}
//
//	//-------------------------------------------------- Queue.
//	#[inline(always)]
//	fn msg_add_queue_song(&mut self, song: SongKey, clear: bool, append: Append) {
//		trace!("Audio - msg_add_queue_song({song:?}) - {append:?}");
//
//		let mut state = AUDIO_STATE.write();
//
//		if clear {
//			self.clear_queue_sink(true, &mut state)
//		}
//
//		if let Some((song, key)) = self.to_source(song) {
//			self.sink.append(song, Some(append));
//
//			state.queue.push_front(key);
//			state.queue_idx = Some(0);
//			state.song = Some(key);
//		}
//	}
//

	#[inline(always)]
	fn msg_add_queue_album(&mut self, album: AlbumKey, append: Append, clear: bool) {
		trace!("Audio - msg_add_queue_album({album:?}, {append:?}, {clear:?})");

		let mut state = AUDIO_STATE.write();

		if clear {
			self.clear(true, &mut state)
		}

		// INVARIANT:
		// `Collection` only creates `Album`'s that
		// have a minimum of 1 `Song`, so this should
		// never panic.
		self.set(self.collection.albums[album].songs[0]);

		// FIXME:
		// These have to be below because the above takes `&mut self`
		// and below create a `&` in the same scope.
		let album = &self.collection.albums[album];
		let first_song  = album.songs[0];
		state.song      = Some(first_song);
		state.queue_idx = Some(0);

		let keys = album.songs.iter();
		match append {
			Append::Front => keys.rev().for_each(|k| state.queue.push_front(*k)),
			Append::Back  => keys.for_each(|k| state.queue.push_back(*k)),
			Append::Index(mut i) => {
				keys.for_each(|k| {
					state.queue.insert(i, *k);
					i += 1;
				});
			}
		}
	}

//	#[inline(always)]
//	fn msg_add_queue_artist(&mut self, artist: ArtistKey, clear: bool, append: Append) {
//		trace!("Audio - msg_add_queue_artist({artist:?}) - {append:?}");
//
//		let mut state = AUDIO_STATE.write();
//
//		if clear {
//			self.clear_queue_sink(true, &mut state)
//		}
//
//		let (songs, keys) = self.to_source_artist(artist);
//
//		if !songs.is_empty() {
//			self.sink.append_bulk(songs, Some(append));
//		}
//	}
//
//	#[inline(always)]
//	fn msg_play_queue_index(&mut self, index: usize) {
//		trace!("Audio - msg_play_queue_index({index})");
//		todo!();
//	}
//
//	#[inline(always)]
//	fn msg_remove_queue_index(&mut self, index: usize) {
//		trace!("Audio - msg_remove_queue_index({index})");
//		todo!();
//	}
//
//	//-------------------------------------------------- Restore Audio State.
	#[inline(always)]
	fn msg_restore_audio_state(&mut self) {
		trace!("Audio - msg_restore_audio_state()");

		// INVARIANT:
		// `Kernel` validates `AUDIO_STATE` before handing
		// it off to `Audio` so we should be safe to assume
		// the state holds proper indices into the `Collection`.
		let key = {
			let state = AUDIO_STATE.read();
			state.shallow_copy(&mut self.state);

			if let Some(key) = state.queue_idx {
				Some(state.queue[key])
			} else {
				None
			}
		};

		trace!("Audio - Restore: {:#?}", self.state);
		if let Some(key) = key {
			debug!("Audio - Restore ... setting {key:?}");
			self.set(key);
		}
	}

	//-------------------------------------------------- Collection.
	#[inline(always)]
	fn drop_collection(&mut self) {
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

////---------------------------------------------------------------------------------------------------- TESTS
////#[cfg(test)]
////mod tests {
////  #[test]
////  fn __TEST__() {
////  }
////}
