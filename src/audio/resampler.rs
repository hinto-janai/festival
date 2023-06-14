// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// This code is a modified version of:
// `https://github.com/pdeljanov/Symphonia/blob/master/symphonia-play/src/resampler.rs`
//
// Our `output.rs` only supports `f32` at the moment so these
// `<T>` conversions are useless but eventually will be used.
//
// INVARIANT:
// This _must_ use `rubato 0.12.0` or untold debugging pain will occur.
// Anything past `0.12.0` has tons of audio playback issues, panics, etc.
//
// I did not write this code so I have zero clue
// on how to reimplement it for `0.12.0`+ versions.

use symphonia::core::audio::{AudioBuffer, AudioBufferRef, Signal, SignalSpec};
use symphonia::core::conv::{FromSample, IntoSample};
use symphonia::core::sample::Sample;
use anyhow::{bail, Error};

pub(super) struct Resampler<T> {
	resampler: rubato::FftFixedIn<f32>,
	input: Vec<Vec<f32>>,
	output: Vec<Vec<f32>>,
	interleaved: Vec<T>,
	duration: usize,
}

impl<T> Resampler<T>
where
	T: Sample + FromSample<f32> + IntoSample<f32>,
{
	fn resample_inner(&mut self) -> Result<&[T], Error> {
		{
			let mut input: arrayvec::ArrayVec<&[f32], 32> = Default::default();

			for channel in self.input.iter() {
				input.push(&channel[..self.duration]);
			}

			// Resample.
			rubato::Resampler::process_into_buffer(
				&mut self.resampler,
				&input,
				&mut self.output,
				None,
			)?;
		}

		// Remove consumed samples from the input buffer.
		for channel in self.input.iter_mut() {
			channel.drain(0..self.duration);
		}

		// Interleave the planar samples from Rubato.
		let num_channels = self.output.len();

		self.interleaved.resize(num_channels * self.output[0].len(), T::MID);

		for (i, frame) in self.interleaved.chunks_exact_mut(num_channels).enumerate() {
			for (ch, s) in frame.iter_mut().enumerate() {
				*s = self.output[ch][i].into_sample();
			}
		}

		Ok(&self.interleaved)
	}

	pub(super) fn new(spec: SignalSpec, to_sample_rate: usize, duration: u64) -> Result<Self, Error> {
		let duration = TryInto::try_into::<usize>(duration)?;
		let num_channels = spec.channels.count();

		let resampler = rubato::FftFixedIn::<f32>::new(
			spec.rate as usize,
			to_sample_rate,
			duration,
			2,
			num_channels,
		)?;

		let output = rubato::Resampler::output_buffer_allocate(&resampler);

		let input = vec![Vec::with_capacity(duration); num_channels];

		Ok(Self { resampler, input, output, duration, interleaved: Default::default() })
	}

	/// Resamples a planar/non-interleaved input.
	///
	/// Returns the resampled samples in an interleaved format.
	pub(super) fn resample(&mut self, input: AudioBufferRef<'_>) -> Result<&[T], Error> {
		// Copy and convert samples into input buffer.
		convert_samples_any(&input, &mut self.input);

		// Check if more samples are required.
		if self.input[0].len() < self.duration {
			bail!("input len < duration")
		}

		self.resample_inner()
	}

	/// Resample any remaining samples in the resample buffer.
	pub(super) fn flush(&mut self) -> Result<&[T], Error> {
		let len = self.input[0].len();

		if len == 0 {
			bail!("len == 0")
		}

		let partial_len = len % self.duration;

		if partial_len != 0 {
			// Fill each input channel buffer with silence to the next multiple of the resampler
			// duration.
			for channel in self.input.iter_mut() {
				channel.resize(len + (self.duration - partial_len), f32::MID);
			}
		}

		self.resample_inner()
	}
}

fn convert_samples_any(input: &AudioBufferRef<'_>, output: &mut [Vec<f32>]) {
	match input {
		AudioBufferRef::U8(input) => convert_samples(input, output),
		AudioBufferRef::U16(input) => convert_samples(input, output),
		AudioBufferRef::U24(input) => convert_samples(input, output),
		AudioBufferRef::U32(input) => convert_samples(input, output),
		AudioBufferRef::S8(input) => convert_samples(input, output),
		AudioBufferRef::S16(input) => convert_samples(input, output),
		AudioBufferRef::S24(input) => convert_samples(input, output),
		AudioBufferRef::S32(input) => convert_samples(input, output),
		AudioBufferRef::F32(input) => convert_samples(input, output),
		AudioBufferRef::F64(input) => convert_samples(input, output),
	}
}

fn convert_samples<S>(input: &AudioBuffer<S>, output: &mut [Vec<f32>])
where
	S: Sample + IntoSample<f32>,
{
	for (c, dst) in output.iter_mut().enumerate() {
		let src = input.chan(c);
		dst.extend(src.iter().map(|&s| s.into_sample()));
	}
}
