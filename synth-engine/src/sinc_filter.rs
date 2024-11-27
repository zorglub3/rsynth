use core::f32::consts::PI;

fn sinc(fc: f32, x: isize) -> f32 {
    let x = x as f32;

    if x.abs() < f32::EPSILON {
        1.
    } else {
        (fc * x).sin() / x
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
    let mut result = vec![0.; m];

    for index in 0..m {
        let i: isize = (index as isize).saturating_sub((m as isize) / 2);

        result[index] = sinc(fc, i) * blackman_window(i, m);
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
    let kernel = sinc_kernel(0.5, m);

    let mut result = vec![0.; samples.len() / 2];

    let m2 = m / 2;

    for i in 0..result.len() {
        let index = (i + m2) % result.len();
        result[index] = convolve(&kernel, samples, i);
    }

    result
}
