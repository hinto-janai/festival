// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// This code is a modified version of:
// `https://github.com/pdeljanov/Symphonia/blob/master/symphonia-play/src/output.rs`

//---------------------------------------------------------------------------------------------------- Use
use symphonia::core::audio::*;
use symphonia::core::units::Duration;
use crate::constants::FESTIVAL;
use benri::log::*;
use anyhow::anyhow;

//---------------------------------------------------------------------------------------------------- Audio Output
// This `Output` trait describes the functions
// needed to output audio to the device.
//
// It's needed because Linux uses `PulseAudio`
// while Windows/macOS will use the `cpal` backend.
pub(crate) trait Output: Sized {
	fn write(&mut self, decoded: AudioBufferRef<'_>) -> std::result::Result<(), AudioOutputError>;
	// Discard current audio samples.
	fn flush(&mut self);
	fn try_open(spec: SignalSpec, duration: Duration) -> std::result::Result<Self, AudioOutputError>;

	// Open the audio device with dummy values.
	fn dummy() -> std::result::Result<Self, AudioOutputError> {
		let spec = SignalSpec {
			// INVARIANT: Must be non-zero.
			rate: 48_000,

			// INVARIANT: Must be a valid entry in the below map `match`.
			channels: Channels::FRONT_LEFT,
		};

		Self::try_open(spec, 4096)
	}
}

#[derive(Debug)]
pub(crate) enum AudioOutputError {
	OpenStream(anyhow::Error),
	PlayStream(anyhow::Error),
	StreamClosed(anyhow::Error),
	Channel(anyhow::Error),
	InvalidSpec(anyhow::Error),
	NonF32(anyhow::Error),
	Resampler(anyhow::Error),
}

impl AudioOutputError {
	pub(crate) fn into_anyhow(self) -> anyhow::Error {
		use AudioOutputError::*;
		match self {
			OpenStream(a)   => a,
			PlayStream(a)   => a,
			StreamClosed(a) => a,
			Channel(a)      => a,
			InvalidSpec(a)  => a,
			NonF32(a)       => a,
			Resampler(a)    => a,
		}
	}
}

pub(crate) use output::*;

//---------------------------------------------------------------------------------------------------- Linux
#[cfg(target_os = "linux")]
mod output {
	use super::*;

	use libpulse_binding as pulse;
	use libpulse_simple_binding as psimple;

	use log::{error, warn};

	pub(crate) struct AudioOutput {
		pa: psimple::Simple,
		sample_buf: RawSampleBuffer<f32>,
		pub(crate) spec: SignalSpec,
		pub(crate) duration: Duration,
	}

	impl Output for AudioOutput {
		fn try_open(spec: SignalSpec, duration: Duration) -> std::result::Result<Self, AudioOutputError> {
			// An interleaved buffer is required to send data to PulseAudio. Use a SampleBuffer to
			// move data between Symphonia AudioBuffers and the byte buffers required by PulseAudio.
			let sample_buf = RawSampleBuffer::<f32>::new(duration, spec);

			// Create a PulseAudio stream specification.
			let pa_spec = pulse::sample::Spec {
				format: pulse::sample::Format::FLOAT32NE,
				channels: spec.channels.count() as u8,
				rate: spec.rate,
			};

			if !pa_spec.is_valid() {
				return Err(AudioOutputError::InvalidSpec(anyhow!("invalid stream specification: {pa_spec:#?}")));
			}

			let pa_ch_map = map_channels_to_pa_channelmap(spec.channels);

			if pa_ch_map.is_none() {
				return Err(AudioOutputError::Channel(anyhow!("invalid channels: {:#?}", spec.channels)));
			}

			// Create PulseAudio buffer attribute.
			const T_LENGTH: u32 = 16384;
			let pa_buf_attr = pulse::def::BufferAttr {
				// This reduces the audio buffer we hold.
				//
				// The default will hold around 200ms~ of audio
				// which creates a noticeable delay when pausing
				// via Festival since we flush the samples that
				// haven't been played yet.
				//
				// This sets it to around 50ms~.
				tlength: T_LENGTH,

				maxlength: std::u32::MAX,
				prebuf: std::u32::MAX,
				minreq: std::u32::MAX,
				fragsize: std::u32::MAX,
			};

			// Create a PulseAudio connection.
			let pa_result = psimple::Simple::new(
				None,                               // Use default server
				FESTIVAL,                           // Application name
				pulse::stream::Direction::Playback, // Playback stream
				None,                               // Default playback device
				"Music",                            // Description of the stream
				&pa_spec,                           // Signal specifications
				pa_ch_map.as_ref(),                 // Channel map
				Some(&pa_buf_attr),                 // Custom buffering attributes
			);

			match pa_result {
				Ok(pa) => Ok(AudioOutput { pa, sample_buf, spec, duration, }),
				Err(err) => {
					fail!("Audio - AudioOutput stream open error: {err}");
					Err(AudioOutputError::OpenStream(anyhow!(err)))
				}
			}
		}

		fn write(&mut self, decoded: AudioBufferRef<'_>) -> std::result::Result<(), AudioOutputError> {
			// Do nothing if there are no audio frames.
			if decoded.frames() == 0 {
				return Ok(());
			}

			// Interleave samples from the audio buffer into the sample buffer.
			self.sample_buf.copy_interleaved_ref(decoded);

			// Write interleaved samples to PulseAudio.
			match self.pa.write(self.sample_buf.as_bytes()) {
				Err(err) => {
					error!("Audio - AudioOutput stream write error: {err}");
					Err(AudioOutputError::StreamClosed(anyhow!(err)))
				}
				_ => Ok(()),
			}
		}

		fn flush(&mut self) {
			_ = self.pa.flush();
		}
	}

	/// Maps a set of Symphonia `Channels` to a PulseAudio channel map.
	fn map_channels_to_pa_channelmap(channels: Channels) -> Option<pulse::channelmap::Map> {
		let mut map: pulse::channelmap::Map = Default::default();
		map.init();
		map.set_len(channels.count() as u8);

		let is_mono = channels.count() == 1;

		for (i, channel) in channels.iter().enumerate() {
			map.get_mut()[i] = match channel {
				Channels::FRONT_LEFT if is_mono => pulse::channelmap::Position::Mono,
				Channels::FRONT_LEFT => pulse::channelmap::Position::FrontLeft,
				Channels::FRONT_RIGHT => pulse::channelmap::Position::FrontRight,
				Channels::FRONT_CENTRE => pulse::channelmap::Position::FrontCenter,
				Channels::REAR_LEFT => pulse::channelmap::Position::RearLeft,
				Channels::REAR_CENTRE => pulse::channelmap::Position::RearCenter,
				Channels::REAR_RIGHT => pulse::channelmap::Position::RearRight,
				Channels::LFE1 => pulse::channelmap::Position::Lfe,
				Channels::FRONT_LEFT_CENTRE => pulse::channelmap::Position::FrontLeftOfCenter,
				Channels::FRONT_RIGHT_CENTRE => pulse::channelmap::Position::FrontRightOfCenter,
				Channels::SIDE_LEFT => pulse::channelmap::Position::SideLeft,
				Channels::SIDE_RIGHT => pulse::channelmap::Position::SideRight,
				Channels::TOP_CENTRE => pulse::channelmap::Position::TopCenter,
				Channels::TOP_FRONT_LEFT => pulse::channelmap::Position::TopFrontLeft,
				Channels::TOP_FRONT_CENTRE => pulse::channelmap::Position::TopFrontCenter,
				Channels::TOP_FRONT_RIGHT => pulse::channelmap::Position::TopFrontRight,
				Channels::TOP_REAR_LEFT => pulse::channelmap::Position::TopRearLeft,
				Channels::TOP_REAR_CENTRE => pulse::channelmap::Position::TopRearCenter,
				Channels::TOP_REAR_RIGHT => pulse::channelmap::Position::TopRearRight,
				_ => {
					// If a Symphonia channel cannot map to a PulseAudio position then return None
					// because PulseAudio will not be able to open a stream with invalid channels.
					warn!("Audio - failed to map channel {channel:?} to output");
					return None;
				}
			}
		}

		Some(map)
	}
}

//---------------------------------------------------------------------------------------------------- Windows/macOS
#[cfg(not(target_os = "linux"))]
mod output {
	use super::*;

	use crate::audio::resampler::Resampler;

	use symphonia::core::audio::{AudioBufferRef, RawSample, SampleBuffer, SignalSpec};
	use symphonia::core::conv::{ConvertibleSample, IntoSample};
	use symphonia::core::units::Duration;

	use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
	use rb::*;

	use log::{error, info, warn, trace};

	// SOMEDAY: support i16/u16.
	pub(crate) struct AudioOutput {
		ring_buf_producer: rb::Producer<f32>,
		sample_buf: SampleBuffer<f32>,
		stream: cpal::Stream,
		resampler: Option<Resampler<f32>>,
		pub(crate) spec: SignalSpec,
		pub(crate) duration: Duration,
	}

	impl Output for AudioOutput {
		fn try_open(spec: SignalSpec, duration: Duration) -> std::result::Result<Self, AudioOutputError> {
			// Get default host.
			let host = cpal::default_host();

			// Get the default audio output device.
			let device = match host.default_output_device() {
				Some(device) => device,
				_            => return Err(AudioOutputError::OpenStream(anyhow!("no default audio output device"))),
			};

			let config = match device.default_output_config() {
				Ok(config) => config,
				Err(err) => return Err(AudioOutputError::OpenStream(anyhow!(err))),
			};

			// SOMEDAY: support i16/u16.
			if config.sample_format() != cpal::SampleFormat::F32 {
				return Err(AudioOutputError::NonF32(anyhow!("sample format is not f32")));
			}

			let num_channels = spec.channels.count();

			// Output audio stream config.
			#[cfg(windows)]
			let config = config.config();
			#[cfg(unix)]
			let config = cpal::StreamConfig {
				channels: num_channels as cpal::ChannelCount,
				sample_rate: cpal::SampleRate(spec.rate),
				buffer_size: cpal::BufferSize::Default,
			};

			// Create a ring buffer with a capacity for up-to 50ms of audio.
			let ring_len = ((50 * config.sample_rate.0 as usize) / 1000) * num_channels;

			let ring_buf = SpscRb::new(ring_len);
			let (ring_buf_producer, ring_buf_consumer) = (ring_buf.producer(), ring_buf.consumer());

			let stream_result = device.build_output_stream(
				&config,
				move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
					// Write out as many samples as possible from the ring buffer to the audio output.
					let written = ring_buf_consumer.read(data).unwrap_or(0);

					// Mute any remaining samples.
					data[written..].iter_mut().for_each(|s| *s = 0.0);
				},
				move |err| warn!("Audio - audio output error: {err}"),
				None,
			);

			let stream = match stream_result {
				Ok(s) => s,
				Err(err) => return Err(AudioOutputError::OpenStream(anyhow!(err))),
			};

			// Start the output stream.
			if let Err(err) = stream.play() {
				return Err(AudioOutputError::PlayStream(anyhow!(err)));
			}

			let sample_buf = SampleBuffer::<f32>::new(duration, spec);

			let resampler = if spec.rate != config.sample_rate.0 {
				trace!("Audio - resampling {} Hz to {} Hz", spec.rate, config.sample_rate.0);
				match Resampler::new(spec, config.sample_rate.0 as usize, duration) {
					Ok(r)  => Some(r),
					Err(e) => {
						error!("Audio - failed to create resampler: {e}");
						return Err(AudioOutputError::Resampler(anyhow!(e)));
					},
				}
			} else {
				None
			};

			Ok(Self { ring_buf_producer, sample_buf, stream, resampler, spec, duration })
		}

		fn write(&mut self, decoded: AudioBufferRef<'_>) -> std::result::Result<(), AudioOutputError> {
			// Do nothing if there are no audio frames.
			if decoded.frames() == 0 {
				return Ok(());
			}

			let mut samples = if let Some(resampler) = &mut self.resampler {
				// Resampling is required. The resampler will return interleaved samples in the
				// correct sample format.
				match resampler.resample(decoded) {
					Ok(resampled) => resampled,
					Err(e) => {
						trace!("Audio - write(): {e}");
						return Ok(());
					},
				}
			} else {
				// Resampling is not required. Interleave the sample for cpal using a sample buffer.
				self.sample_buf.copy_interleaved_ref(decoded);

				self.sample_buf.samples()
			};

			// Write all samples to the ring buffer.
			while let Some(written) = self.ring_buf_producer.write_blocking(samples) {
				samples = &samples[written..];
			}

			Ok(())
		}

		fn flush(&mut self) {
			// If there is a resampler, then it may need to be
			// flushed depending on the number of samples it has.
			if let Some(resampler) = &mut self.resampler {
				let mut remaining_samples = resampler.flush().unwrap_or_default();

				while let Some(written) = self.ring_buf_producer.write_blocking(remaining_samples) {
					remaining_samples = &remaining_samples[written..];
				}
			}

			// DO NOT USE: this hangs forever.
			//
//			// Flush is best-effort, ignore the returned result.
//			drop(self.stream.pause());
		}
	}
}
