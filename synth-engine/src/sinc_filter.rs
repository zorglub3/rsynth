use core::f32::consts::PI;

fn sinc(fc: f32, x: isize) -> f32 {
    let x = x as f32;

    if x.abs() < f32::EPSILON {
        1.
    } else {
        (2. * PI * fc * x).sin() / x
    }
}

#[allow(dead_code)]
fn hamming_window(i: isize, m: usize) -> f32 {
    let m = m as f32;
    let i = i as f32;

    0.54 - 0.46 * (2. * PI * i / m).cos()
}

#[allow(dead_code)]
fn blackman_window(i: isize, m: usize) -> f32 {
    let m = m as f32;
    let i = i as f32;

    0.42 - 0.5 * (2. * PI * i / m).cos() + 0.08 * (4. * PI * i / m).cos()
}

pub fn sinc_kernel(fc: f32, m: usize) -> Vec<f32> {
    let m2 = (m as isize) / 2;
    let mut result = Vec::with_capacity((m2 * 2 + 1) as usize);
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

pub fn downsample_half(m: usize, samples: &[f32]) -> Vec<f32> {
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
