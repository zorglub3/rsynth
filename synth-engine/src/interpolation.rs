use core::ops::Deref;

// see https://www.paulinternet.nl/?page=bicubic
fn cubic(p0: f32, p1: f32, p2: f32, p3: f32, x: f32) -> f32 {
    let x2 = x * x;
    let x3 = x2 * x;

    (-0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3) * x3
        + (p0 - 2.5 * p1 + 2. * p2 - 0.5 * p3) * x2
        + (-0.5 * p0 + 0.5 * p2) * x
        + p1
}

fn sinc(x: f32) -> f32 {
    if x.abs() < f32::EPSILON {
        1.
    } else {
        x.sin() / x
    }
}

pub trait Interpolation {
    fn cubic_interpolate(&self, x: f32) -> f32;
    fn sinc_interpolate(&self, x: f32) -> f32;
    fn linear_interpolate(&self, x: f32) -> f32;
}

impl<T: Deref<Target = [f32]>> Interpolation for T {
    fn cubic_interpolate(&self, x: f32) -> f32 {
        let len = self.len();
        let index: usize = x.floor() as usize;

        let i0 = (((index - 2) % len) + len) % len;
        let i1 = (((index - 1) % len) + len) % len;
        let i2 = ((index % len) + len) % len;
        let i3 = (((index + 1) % len) + len) % len;

        let x = x - x.floor();

        cubic(self[i0], self[i1], self[i2], self[i3], x)
    }

    fn sinc_interpolate(&self, x: f32) -> f32 {
        todo!()
    }

    fn linear_interpolate(&self, x: f32) -> f32 {
        let len = self.len();
        let index: usize = x.floor() as usize;

        let i0 = ((index % len) + len) % len;
        let i1 = (((index + 1) % len) + len) % len;

        let x = x - x.floor();

        let p0 = self[i0];
        let p1 = self[i1];

        p0 + (p1 - p0) * x
    }
}
