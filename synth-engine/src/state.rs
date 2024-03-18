use std::iter::zip;

#[derive(Clone, Copy, Debug)]
pub enum OutputType {
    Differentiable,
    Absolute,
    Lookup,
}

#[inline]
pub fn copy_state(v1: &[f32], v2: &mut [f32]) {
    v2.copy_from_slice(v1);
}

#[inline]
pub fn update_state(modes: &[OutputType], w: f32, v1: &[f32], v2: &[f32], out: &mut [f32]) {
    for ((mode, out), (a, b)) in zip(zip(modes, out), zip(v1, v2)) {
        match mode {
            OutputType::Differentiable => *out = a + w * b,
            OutputType::Absolute => *out = *b,
            OutputType::Lookup => todo!(),
        }
    }
}

#[inline]
pub fn finalize_state(
    dt: f32,
    modes: &[OutputType],
    weights: &[f32],
    k: &[&[f32]],
    state: &mut [f32],
) {
    for i in 0..state.len() {
        match modes[i] {
            OutputType::Differentiable => {
                let mut acc: f32 = 0.0_f32;

                for j in 0..k.len() {
                    acc += weights[j] * k[j][i];
                }

                state[i] += dt * acc;
            }
            OutputType::Absolute => {
                let mut acc: f32 = 0.0_f32;

                for j in 0..k.len() {
                    acc += weights[j] * k[j][i];
                }

                state[i] = acc;
            }
            OutputType::Lookup => todo!(),
        }
    }
}
