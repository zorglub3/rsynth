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

pub trait Interpolation {
    fn cubic_interpolate(&self, x: f32) -> f32;
    fn linear_interpolate(&self, x: f32) -> f32;
}

impl<T: Deref<Target = [f32]>> Interpolation for T {
    fn cubic_interpolate(&self, x: f32) -> f32 {
        let len = self.len() as i32;
        let index: i32 = x.floor() as i32;

        let i0 = ((((index - 1) % len) + len) % len) as usize;
        let i1 = (((index % len) + len) % len) as usize;
        let i2 = ((((index + 1) % len) + len) % len) as usize;
        let i3 = ((((index + 2) % len) + len) % len) as usize;

        let x = x - x.floor();

        cubic(self[i0], self[i1], self[i2], self[i3], x)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cubic_interpolation_on_line() {
        let ys: Vec<f32> = vec![0., 1., 2., 3., 4., 5.];

        assert_eq!(ys.cubic_interpolate(3.), 3.);

        assert_eq!(ys.cubic_interpolate(1.), 1.);

        assert_eq!(ys.cubic_interpolate(3.5), 3.5);

        assert_eq!(ys.cubic_interpolate(0.5), 0.125);
    }

    #[test]
    fn linear_interpolation_on_line() {
        let ys: Vec<f32> = vec![0., 1., 2., 3., 4., 5.];

        assert_eq!(ys.linear_interpolate(0.), 0.);
        assert_eq!(ys.linear_interpolate(1.), 1.);
        assert_eq!(ys.linear_interpolate(1.5), 1.5);
    }
}
