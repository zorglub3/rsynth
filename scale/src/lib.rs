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

    pub fn from_file(_filename: &str, _base_pitch: usize) -> Self {
        todo!("Read scale from file")
    }

    pub fn pitch_value(&self, pitch: u8) -> f32 {
        self.0[(pitch & 0x7F) as usize]
    }
}
