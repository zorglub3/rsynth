use crate::interpolation::Interpolation;
use crate::sinc_filter::downsample_half;
use alloc::vec::Vec;

pub struct WavetableEntry {
    pub samples: Vec<f32>,
    pub len_f32: f32,
}

impl WavetableEntry {
    pub fn from_slice(data: &[f32]) -> Self {
        let mut samples = Vec::new();
        samples.extend_from_slice(data);
        let len_f32 = samples.len() as f32;
        Self { samples, len_f32 }
    }

    pub fn from_vec(samples: Vec<f32>) -> Self {
        let len_f32 = samples.len() as f32;
        Self { samples, len_f32 }
    }

    pub fn downsample(&self) -> Option<Self> {
        if self.samples.len() < 8 {
            None
        } else {
            let m = (self.samples.len() / 2).min(32);
            let downsampled = downsample_half(m, &self.samples);
            Some(Self::from_vec(downsampled))
        }
    }

    pub fn eval(&self, x: f32) -> f32 {
        let x = x * self.len_f32;
        self.samples.cubic_interpolate(x)
    }
}

pub struct Wavetable {
    pub data: Vec<WavetableEntry>,
    pub base_data_len: f32,
}

impl Wavetable {
    pub fn from_slice(samples: &[f32]) -> Self {
        let mut data = Vec::new();

        let mut current_entry = WavetableEntry::from_slice(samples);

        while let Some(next_entry) = current_entry.downsample() {
            data.push(current_entry);
            current_entry = next_entry;
        }

        data.push(current_entry);

        Self {
            data,
            base_data_len: samples.len() as f32,
        }
    }

    pub fn eval(&self, cycles_per_step: f32, x: f32) -> f32 {
        let mut samples_per_step = self.base_data_len * cycles_per_step;

        for i in 0..self.data.len() {
            if samples_per_step <= 1. {
                return self.data[i].eval(x);
            } else {
                samples_per_step /= 2.;
            }
        }

        0.
    }
}
