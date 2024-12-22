/// One happy day, this module will replace the math functions of `libm` with
/// faster (but maybe slightly less precise) function. For now we just get
/// a wrapper on `libm`.

/// See [this note on fast
/// exponentation](https://codingforspeed.com/using-faster-exponential-approximation/)
/// It works
#[allow(dead_code)]
fn fast_exp_256(x: f32) -> f32 {
    let mut x = 1. + x / 256.;
    x *= x; x *= x; x *= x; x *= x;
    x *= x; x *= x; x *= x; x *= x;
    x
}

#[allow(dead_code)]
fn fast_exp_1024(x: f32) -> f32 {
    let mut x = 1. + x / 1024.;
    x *= x; x *= x; x *= x; x *= x;
    x *= x; x *= x; x *= x; x *= x;
    x *= x; x *= x;
    x
}

const LN2: f32 = 0.6931471805599453;

#[allow(dead_code)]
fn fast_exp2_256(x: f32) -> f32 {
    fast_exp_256(LN2 * x)
}

#[allow(dead_code)]
fn fast_exp2_1024(x: f32) -> f32 {
    fast_exp_1024(LN2 * x)
}

pub trait SynthMath {
    type Output;

    fn abs(&self) -> Self::Output;
    fn cos(&self) -> Self::Output;
    fn sin(&self) -> Self::Output;
    fn tan(&self) -> Self::Output;
    fn tanh(&self) -> Self::Output;

    fn ln(&self) -> Self::Output;
    fn exp(&self) -> Self::Output;
    fn exp2(&self) -> Self::Output;

    fn sqrt(&self) -> Self::Output;
    fn signum(&self) -> Self::Output;

    fn floor(&self) -> Self::Output;
    fn ceil(&self) -> Self::Output;
    fn fract(&self) -> Self::Output;
}

impl SynthMath for f32 {
    type Output = Self;

    fn abs(&self) -> Self {
        libm::fabsf(*self)
    }

    fn cos(&self) -> Self {
        libm::cosf(*self)
    }

    fn sin(&self) -> Self {
        libm::sinf(*self)
    }

    fn tan(&self) -> Self {
        libm::tanf(*self)
    }

    fn tanh(&self) -> Self {
        libm::tanhf(*self)
    }

    fn ln(&self) -> Self {
        libm::logf(*self)
    }

    fn exp(&self) -> Self {
        libm::expf(*self)
    }

    fn exp2(&self) -> Self {
        libm::exp2f(*self)
    }

    fn floor(&self) -> Self {
        libm::floorf(*self)
    }

    fn ceil(&self) -> Self {
        libm::ceilf(*self)
    }

    fn sqrt(&self) -> Self {
        libm::sqrtf(*self)
    }

    fn signum(&self) -> Self {
        if *self > 0. {
            1.
        } else if *self < 0. {
            -1.
        } else {
            0.
        }
    }

    fn fract(&self) -> Self {
        *self - self.floor()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fast_exp2_1024() {
        let x = 1.;
        assert_eq!(libm::exp2f(x), fast_exp2_1024(x));
        let x = 2.;
        assert_eq!(libm::exp2f(x), fast_exp2_1024(x));
        let x = 5.;
        assert_eq!(libm::exp2f(x), fast_exp2_1024(x));
        let x = 0.;
        assert_eq!(libm::exp2f(x), fast_exp2_1024(x));
        let x = 0.001;
        assert_eq!(libm::exp2f(x), fast_exp2_1024(x));
    }

    #[test]
    fn test_fast_exp2_256() {
        let x = 1.;
        assert_eq!(libm::exp2f(x), fast_exp2_256(x));
        let x = 2.;
        assert_eq!(libm::exp2f(x), fast_exp2_256(x));
        let x = 5.;
        assert_eq!(libm::exp2f(x), fast_exp2_256(x));
        let x = 0.;
        assert_eq!(libm::exp2f(x), fast_exp2_256(x));
        let x = 0.001;
        assert_eq!(libm::exp2f(x), fast_exp2_256(x));
    }


}
