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
	units::{TimeBase,Time,TimeStamp},
};
use std::time::{Instant, Duration};
use crate::audio::output::{
	AudioOutput,Output,
};
use readable::Runtime;
use crate::frontend::egui::gui_request_update;

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
	// The existence of this field means we should
	// be seeking in the next loop iteration.
	seek: Option<symphonia::core::units::Time>,

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
	// Song's `TimeBase`
	timebase: TimeBase,
	// Elapsed `Time`
	time: Time,
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
			seek: None,
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
				// Flush output on pause.
				//
				// These prevents a few stutters from happening
				// when users switch songs (the leftover samples
				// get played then and sound terrible).
//				trace!("Audio - Pause [1/3]: flush()'ing leftover samples");
				// TODO:
				// This hangs for 30s, for some reason.
//				self.output.flush();

				trace!("Audio - Pause [2/3]: waiting on Kernel...");
				self.kernel_msg(recv!(self.from_kernel));

				trace!("Audio - Pause [3/3]: woke up from Kernel message...!");
			}

			// If the above message didn't set `playing` state, that
			// means we don't need to continue below to decode audio.
			if !self.state.playing {
				continue;
			}


			//------ Audio decoding & demuxing.
			if let Some(audio_reader) = &mut self.current {
				let AudioReader {
					reader,
					decoder,
					timebase,
					time,
				} = audio_reader;

				//------ Audio seeking.
				if let Some(seek) = self.seek.take() {
					if let Err(e) = reader.seek(
						symphonia::core::formats::SeekMode::Coarse,
						symphonia::core::formats::SeekTo::Time { time: seek, track_id: None }
					) {
						send!(self.to_kernel, AudioToKernel::SeekError(anyhow!(e)));
					} else {
						AUDIO_STATE.write().elapsed = Runtime::from(seek.seconds);
					}
				}

				// Decode and play the packets belonging to the selected track.
				// Get the next packet from the format reader.
				let packet = match reader.next_packet() {
					Ok(packet) => packet,
					// We're done playing audio.
					// This "end of stream" error is currently the only way
					// a FormatReader can indicate the media is complete.
					Err(symphonia::core::errors::Error::IoError(err)) => {
						self.next(&mut AUDIO_STATE.write());
						gui_request_update();
						continue;
					},
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

						// Convert the buffer to `f32` and multiply
						// it by `0.0..1.0` to set volume levels.
						let mut buf = AudioBuffer::<f32>::new(duration, spec);
						decoded.convert(&mut buf);
						buf.transform(|f| f * self.state.volume.f32());

						// Write to audio output device.
						self.output.write(buf.as_audio_buffer_ref()).unwrap();

						// Set runtime timestamp.
						let new_time = timebase.calc_time(packet.ts);
						if time.seconds != new_time.seconds {
							*time = new_time;

							// Set state.
							AUDIO_STATE.write().elapsed = Runtime::from(time.seconds);

							// Wake up the GUI thread.
							gui_request_update();
						}
					}
					Err(symphonia::core::errors::Error::DecodeError(err)) => {
						// Decode errors are not fatal. Print the error message and try to decode the next
						// packet as usual.
						warn!("decode error: {}", err);
					}
//					Err(err) => break Err(err),
					// We're done playing audio.
					Err(symphonia::core::errors::Error::IoError(err)) => {
						self.next(&mut AUDIO_STATE.write());
						gui_request_update();
						continue;
					},
					Err(err) => todo!(),
				}
			// If `playing == true`, but our state doesn't
			// have a Some(song), then something went wrong.
			} else {
				todo!();
			}

		} //------ End of `loop {}`.

		// Audio should never exit the above loop.
//		panic!("Audio - exited the `main()` loop");
	}

	//-------------------------------------------------- Kernel message.
	#[inline(always)]
	fn kernel_msg(&mut self, msg: KernelToAudio) {
		use KernelToAudio::*;
		match msg {
			// Audio playback.
			Toggle    => self.toggle(),
			Play      => self.play(),
			Pause     => self.pause(),
			Next      => self.next(&mut AUDIO_STATE.write()),
			Previous  => self.previous(),
//
//			// Audio settings.
			Shuffle(s) => self.shuffle(s),
			Repeat(r)  => self.repeat(r),
			Volume(v)  => self.volume(v),
//
//			// Queue.
//			AddQueueSongFront((s_key, clear))     => self.add_queue_song(s_key, clear,      Append::Front),
//			AddQueueSongBack((s_key, clear))      => self.add_queue_song(s_key, clear,      Append::Back),
			AddQueueAlbum((al_key, append, clear, offset))   => self.add_queue_album(al_key, append, clear, offset),
//			AddQueueAlbumBack((al_key, clear))    => self.add_queue_album(al_key, clear,    Append::Back),
//			AddQueueArtistFront((ar_key, clear))  => self.add_queue_artist(ar_key, clear,   Append::Front),
//			AddQueueArtistBack((ar_key, clear))   => self.add_queue_artist(ar_key, clear,   Append::Back),
			Seek(f)   => self.seek(f, &mut AUDIO_STATE.write()),
			Skip(num) => self.skip(num),
			Back(num) => self.back(num),
//
//			// Queue Index.
			SetQueueIndex(idx)      => self.set_queue_index(idx),
			RemoveQueueRange(range) => self.remove_queue_range(range),
//
//			// Audio State.
			RestoreAudioState => self.restore_audio_state(),

			// Collection.
			DropCollection     => self.drop_collection(),
			NewCollection(arc) => self.collection = arc,

			_ => todo!(),
		}
	}

	//-------------------------------------------------- Non-msg functions.
	// Error handling gets handled in these functions
	// rather than the caller (it gets called everywhere).

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
	fn set_current(&mut self, reader: Box<dyn FormatReader>, timebase: TimeBase) {
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
			timebase,
			time: Time::new(0, 0.0),
		});
	}

	// Convenience function that combines the above 2 functions.
	// Does nothing on error.
	fn set(
		&mut self,
		key: SongKey,
		state: &mut std::sync::RwLockWriteGuard<'_, AudioState>,
	) {
		if let Some(reader) = self.to_reader(key) {
			self.set_current(
				reader,
				TimeBase::new(1, self.collection.songs[key].sample_rate),
			);

			// Set song state.
			state.song    = Some(key);
			state.elapsed = Runtime::zero();
			state.runtime = self.collection.songs[key].runtime;

			gui_request_update();
		}
	}

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
		trace!("Audio - clear({keep_playing})");

		state.queue.clear();
		state.queue_idx = None;
		state.playing   = keep_playing;
		state.song      = None;

		self.state.playing = keep_playing;
	}

	//-------------------------------------------------- Audio playback.
	fn toggle(&mut self) {
		trace!("Audio - toggle()");
		if self.current.is_some() {
			flip!(self.state.playing);
			flip!(AUDIO_STATE.write().playing);
			gui_request_update();
		}
	}

	fn play(&mut self) {
		trace!("Audio - play()");
		if self.current.is_some() {
			self.state.playing = true;
			AUDIO_STATE.write().playing = true;
			gui_request_update();
		}
	}

	fn pause(&mut self) {
		trace!("Audio - pause()");
		if self.current.is_some() {
			self.state.playing = false;
			AUDIO_STATE.write().playing = false;
			gui_request_update();
		}
	}

	fn previous(&mut self) {
		trace!("Audio - previous()");
		// Lock state.
		let mut state = AUDIO_STATE.write();

		match state.prev() {
			Some(prev) => {
				trace!("Audio - prev song, setting: {prev:?}");
				self.set(prev, &mut state);
			},
			_ => trace!("Audio - no song for prev"),
		}
	}

	// If there is another element in the queue, play it,
	// else, assume we are done and set the relevant state.
	fn next(
		&mut self,
		state: &mut std::sync::RwLockWriteGuard<'_, AudioState>,
	) {
		// TODO:
		// account for shuffle and repeat
		match state.next() {
			Some(next) => {
				trace!("Audio - more songs left, setting: {next:?}");
				self.set(next, state);
			},
			_ => {
				trace!("Audio - no songs left, calling state.finish()");
				state.finish();
				self.state.finish();
				self.current = None;
				gui_request_update();
			},
		}
	}

	fn skip(&mut self, num: usize) {
		trace!("Audio - skip({num})");

		// Lock state.
		let mut state = AUDIO_STATE.write();

		if !state.queue.is_empty() {
			if let Some(index) = state.queue_idx {
				let new_index = index + num;

				// FIXME:
				// We must do this behavior due to
				// `&` and `&mut` rules. `.get()` makes a `&`
				// and the below `.set()` requires a `&mut`.
				//
				// The inner `Some(key)` is just a usize so I wish
				// Rust would just `Copy` it and move on but... whatever.
//				if let Some(key) = state.queue.get(new_index) {
				let len = state.queue.len();
				if len >= new_index {
					let key = state.queue[new_index];
					self.set(key, &mut state);
					state.song      = Some(key);
					state.queue_idx = Some(new_index);
				} else {
					trace!("Audio - skip({new_index}) > {len}, calling state.finish()");
					state.finish();
					self.state.finish();
					self.current = None;
				}

				gui_request_update();
			}
		}
	}

	fn back(&mut self, num: usize) {
		trace!("Audio - back({num})");

		// Lock state.
		let mut state = AUDIO_STATE.write();

		if !state.queue.is_empty() {
			// FIXME:
			// Same as `skip()`.
			if let Some(index) = state.queue_idx {
				// Back input was greater than our current index,
				// play the first song in our queue.
				if index < num {
					let key = state.queue[0];
					self.set(key, &mut state);
					state.song      = Some(key);
					state.queue_idx = Some(0);
				} else {
					let new_index = index - num;
					let key = state.queue[new_index];
					self.set(key, &mut state);
					state.song      = Some(key);
					state.queue_idx = Some(new_index);
				}

				gui_request_update();
			}
		}
	}

	fn seek(
		&mut self,
		seek: usize,
		state: &mut std::sync::RwLockWriteGuard<'_, AudioState>,
	) {
		trace!("Audio - seek({seek})");

		if self.state.playing {
			let runtime = state.runtime.usize();

			if seek > runtime {
				debug!("Audio - seek: {seek} > {runtime}, calling .next()");
				self.next(state);
			} else {
				self.seek = Some(symphonia::core::units::Time {
					seconds: seek as u64,
					frac: 0.0
				});
			}
		}

		gui_request_update();
	}

	//-------------------------------------------------- Audio settings.
	fn shuffle(&mut self, shuffle: crate::audio::shuffle::Shuffle) {
		trace!("Audio - {shuffle:?}");
		gui_request_update();
		todo!();
	}

	fn repeat(&mut self, repeat: crate::audio::repeat::Repeat) {
		trace!("Audio - {repeat:?}");
		gui_request_update();
		todo!();
	}

	fn volume(&mut self, volume: Volume) {
		trace!("Audio - {volume:?}");
		self.state.volume = volume;
		AUDIO_STATE.write().volume = volume;
	}

//	//-------------------------------------------------- Queue.
//	fn add_queue_song(&mut self, song: SongKey, clear: bool, append: Append) {
//		trace!("Audio - add_queue_song({song:?}) - {append:?}");
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
//			state.queue.push_front(key);
//			state.queue_idx = Some(0);
//			state.song = Some(key);
//		}
//	}
//

	fn add_queue_album(
		&mut self,
		key: AlbumKey,
		append: Append,
		clear: bool,
		offset: usize,
	) {
		trace!("Audio - add_queue_album({key:?}, {append:?}, {clear:?})");

		let mut state = AUDIO_STATE.write();

		if clear {
			self.clear(true, &mut state)
		}

		// Prevent bad offsets panicking.
		let offset = if offset >= self.collection.albums[key].songs.len() {
			0
		} else {
			offset
		};

		let album = &self.collection.albums[key];

		// INVARIANT:
		// `Collection` only creates `Album`'s that
		// have a minimum of 1 `Song`, so this should
		// never panic.
		let keys = album.songs.iter();
		match append {
			Append::Back  => {
				keys.for_each(|k| state.queue.push_back(*k));
				if self.current.is_none() {
					state.queue_idx = Some(offset);
					self.set(self.collection.albums[key].songs[offset], &mut state);
				}
			},
			Append::Front => {
				keys.rev().for_each(|k| state.queue.push_front(*k));
				state.queue_idx = Some(offset);
				self.set(self.collection.albums[key].songs[offset], &mut state);
			},
			Append::Index(mut i) => {
				keys.for_each(|k| {
					state.queue.insert(i, *k);
					i += 1;
				});
				if i == 0 {
					state.queue_idx = Some(0);
					self.set(self.collection.albums[key].songs[offset], &mut state);
				}
			}
		}
	}

//	fn add_queue_artist(&mut self, artist: ArtistKey, clear: bool, append: Append) {
//		trace!("Audio - add_queue_artist({artist:?}) - {append:?}");
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
	fn set_queue_index(&mut self, index: usize) {
		let mut state = AUDIO_STATE.write();

		// Prevent bad index panicking.
		if index >= state.queue.len() {
			warn!("Audio - index is invalid, skipping set_queue_index({index})");
			return;
		}

		trace!("Audio - set_queue_index({index})");
		state.queue_idx = Some(index);
		self.set(state.queue[index], &mut state);
	}

	fn remove_queue_range(
		&mut self,
		range: std::ops::Range<usize>,
//		next: bool,
	) {
		let mut state = AUDIO_STATE.write();

		let len = state.queue.len();

		// Prevent bad start/end panicking.
		if range.start >= len {
			warn!("Audio - start is invalid, skipping remove_queue_range({range:?})");
			return;
		} else if range.end > len {
			warn!("Audio - end is invalid, skipping remove_queue_range({range:?})");
			return;
		}

		trace!("Audio - remove_queue_range({range:?})");
		state.queue.drain(range);

		// TODO
//		// Skip to next song if we removed the current one.
//		if let Some(index) = state.song {
//			if range.contains(index) {
//				let len = state.queue.len();
//
//				if len == 0 {
//					return;
//				}
//
//				// Fallback to the previous if we removed the ahead.
//				if index < len {
//					self.set(state.queue[len - 1], &mut state);
//				} else {
//				}
//			}
//		}

		gui_request_update();
	}

	//-------------------------------------------------- Restore Audio State.
	fn restore_audio_state(&mut self) {
		trace!("Audio - restore_audio_state()");

		let mut state = AUDIO_STATE.write();

		// INVARIANT:
		// `Kernel` validates `AUDIO_STATE` before handing
		// it off to `Audio` so we should be safe to assume
		// the state holds proper indices into the `Collection`.
		let key = {
			state.if_copy(&mut self.state);

			if let Some(key) = state.queue_idx {
				Some(state.queue[key])
			} else {
				None
			}
		};

		trace!("Audio - Restore: {:#?}", self.state);
		if let Some(key) = key {
			debug!("Audio - Restore ... setting {key:?}");
			self.set(key, &mut state);
		}

		let elapsed = self.state.elapsed.usize();
		if elapsed > 0 {
			debug!("Audio - Restore ... seeking {}/{}", self.state.elapsed, self.state.runtime);
			self.seek(elapsed, &mut state);
		} else {
			debug!("Audio - Restore ... skipping seek");
		}
	}

	//-------------------------------------------------- Collection.
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
