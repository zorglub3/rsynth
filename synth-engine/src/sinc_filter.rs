use core::f32::consts::PI;
use libm::{cosf, fabsf, sinf};

fn sinc(fc: f32, x: isize) -> f32 {
    let x = x as f32;

    if fabsf(x) < f32::EPSILON {
        1.
    } else {
        sinf(2. * PI * fc * x) / x
    }
}

#[allow(dead_code)]
fn hamming_window(i: isize, m: usize) -> f32 {
    let m = m as f32;
    let i = i as f32;

    0.54 - 0.46 * cosf(2. * PI * i / m)
}

#[allow(dead_code)]
fn blackman_window(i: isize, m: usize) -> f32 {
    let m = m as f32;
    let i = i as f32;

    0.42 - 0.5 * cosf(2. * PI * i / m) + 0.08 * cosf(4. * PI * i / m)
}

#[cfg(any(feature = "allocator", test))]
pub fn sinc_kernel(fc: f32, m: usize) -> alloc::vec::Vec<f32> {
    let m2 = (m as isize) / 2;
    let mut result = alloc::vec::Vec::with_capacity((m2 * 2 + 1) as usize);
    let mut k: f32 = 0.;

    for i in -m2..=m2 {
        let window = blackman_window(i + m2, m);
        let filter = sinc(fc, i);
        let v = filter * window;

        k += v;
        result.push(v);
    }

    let k = 1. / k;

    for i in 0..result.len() {
        result[i] *= k;
    }

    result
}

pub fn convolve(kernel: &[f32], samples: &[f32], index: usize) -> f32 {
    let mut acc = 0.0_f32;

    for kernel_index in 0..kernel.len() {
        let samples_index = (index + kernel_index) % samples.len();
        acc += kernel[kernel_index] * samples[samples_index];
    }

    acc
}

#[cfg(any(feature = "allocator", test))]
pub fn downsample_half(m: usize, samples: &[f32]) -> alloc::vec::Vec<f32> {
    use alloc::vec;

    let kernel = sinc_kernel(0.25, m);

    let mut result = vec![0.; samples.len() / 2];

    let m2 = m / 2;

    for i in 0..result.len() {
        let i2 = i * 2;
        let index = (i + m2) % result.len();
        result[index] = convolve(&kernel, samples, i2);
    }

    result
}
