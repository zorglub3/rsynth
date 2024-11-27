pub mod scl;

use crate::scl::SclError;
use crate::scl::SclFile;
use std::path::Path;

const PITCH_VALUES_SIZE: usize = 128;

pub struct Scale([f32; PITCH_VALUES_SIZE]);

impl Scale {
    pub fn equal_temperament() -> Self {
        let mut pitch_values = [0.; PITCH_VALUES_SIZE];

        for i in 0..PITCH_VALUES_SIZE {
            pitch_values[i] = (i as f32) / 12.;
        }

        Self(pitch_values)
    }

    pub fn from_file(filename: &str, base_pitch: usize) -> Result<Self, SclError> {
        let scl_file = SclFile::from_file(Path::new(filename))?;
        let pitches = scl_file.to_pitch_vec(base_pitch, 0., PITCH_VALUES_SIZE);

        let mut pitch_values = [0.0_f32; PITCH_VALUES_SIZE];

        pitch_values.copy_from_slice(&pitches);

        Ok(Scale(pitch_values))
    }

    pub fn pitch_value(&self, pitch_index: usize) -> Option<f32> {
        if pitch_index < self.0.len() {
            Some(self.0[pitch_index])
        } else {
            None
        }
    }
}
