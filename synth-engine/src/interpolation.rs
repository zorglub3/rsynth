use core::ops::Deref;
use libm::floorf;

// see https://www.paulinternet.nl/?page=bicubic
fn cubic(p0: f32, p1: f32, p2: f32, p3: f32, x: f32) -> f32 {
    let x2 = x * x;
    let x3 = x2 * x;

    (-0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3) * x3
        + (p0 - 2.5 * p1 + 2. * p2 - 0.5 * p3) * x2
        + (-0.5 * p0 + 0.5 * p2) * x
        + p1
}

const C1: [f32; 4] = [-0.1666667, 0.5, -0.5, 0.1666667];
const C2: [f32; 4] = [0.5, -1., 0.5, 0.];
const C3: [f32; 4] = [-0.33333333, -0.5, 1., -0.16666667];
const C4: [f32; 4] = [0., 1., 0., 0.];

fn lagrange_helper(d: &[f32; 4], i: usize) -> f32 {
    C1[i] * d[0] + C2[i] * d[1] + C3[i] * d[2] + C4[i] * d[3]
}

// see "Fractional Delay Farrow Filter" by Josef Hoffmann
// (Note paper has error in C3 above)
fn lagrange(p0: f32, p1: f32, p2: f32, p3: f32, delta: f32) -> f32 {
    let d: [f32; 4] = [delta * delta * delta, delta * delta, delta, 1.];
    let v: [f32; 4] = [
        lagrange_helper(&d, 0),
        lagrange_helper(&d, 1),
        lagrange_helper(&d, 2),
        lagrange_helper(&d, 3),
    ];

    p0 * v[0] + p1 * v[1] + p2 * v[2] + p3 * v[3]
}

pub trait Interpolation {
    fn cubic_interpolate(&self, x: f32) -> f32;
    fn linear_interpolate(&self, x: f32) -> f32;
    fn lagrange_interpolate(&self, x: f32) -> f32;
}

impl<T: Deref<Target = [f32]>> Interpolation for T {
    fn cubic_interpolate(&self, x: f32) -> f32 {
        let len = self.len() as i32;
        let index: i32 = floorf(x) as i32;

        let i0 = ((((index - 1) % len) + len) % len) as usize;
        let i1 = (((index % len) + len) % len) as usize;
        let i2 = ((((index + 1) % len) + len) % len) as usize;
        let i3 = ((((index + 2) % len) + len) % len) as usize;

        let x = x - floorf(x);

        cubic(self[i0], self[i1], self[i2], self[i3], x)
    }

    fn lagrange_interpolate(&self, x: f32) -> f32 {
        let len = self.len() as i32;
        let index: i32 = floorf(x) as i32;

        let i0 = ((((index - 1) % len) + len) % len) as usize;
        let i1 = (((index % len) + len) % len) as usize;
        let i2 = ((((index + 1) % len) + len) % len) as usize;
        let i3 = ((((index + 2) % len) + len) % len) as usize;

        let x = x - floorf(x);

        lagrange(self[i0], self[i1], self[i2], self[i3], x)
    }

    fn linear_interpolate(&self, x: f32) -> f32 {
        let len = self.len();
        let index: usize = floorf(x) as usize;

        let i0 = ((index % len) + len) % len;
        let i1 = (((index + 1) % len) + len) % len;

        let x = x - floorf(x);

        let p0 = self[i0];
        let p1 = self[i1];

        p0 + (p1 - p0) * x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn cubic_interpolation_on_line() {
        let ys: Vec<f32> = vec![0., 1., 2., 3., 4., 5.];

        assert_eq!(ys.cubic_interpolate(3.), 3.);
        assert_eq!(ys.cubic_interpolate(1.), 1.);
        assert_eq!(ys.cubic_interpolate(3.5), 3.5);
        assert_eq!(ys.cubic_interpolate(0.5), 0.125);
    }

    #[test]
    fn lagrange_interpolation_on_line() {
        let ys: Vec<f32> = vec![0., 1., 2., 3., 4., 5., 6., 5., 8.];

        assert_eq!(ys.lagrange_interpolate(3.), 3.);
        assert_eq!(ys.lagrange_interpolate(1.), 1.);
        assert_eq!(ys.lagrange_interpolate(2.), 2.);
        assert_eq!(ys.lagrange_interpolate(5.), 5.);
        assert_eq!(ys.lagrange_interpolate(3.5), 3.5);
        assert_eq!(ys.lagrange_interpolate(1.5), 1.5);
    }

    #[test]
    fn linear_interpolation_on_line() {
        let ys: Vec<f32> = vec![0., 1., 2., 3., 4., 5.];

        assert_eq!(ys.linear_interpolate(0.), 0.);
        assert_eq!(ys.linear_interpolate(1.), 1.);
        assert_eq!(ys.linear_interpolate(1.5), 1.5);
    }
}
