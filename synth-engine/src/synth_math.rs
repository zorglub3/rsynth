/// One happy day, this module will replace the math functions of `libm` with
/// faster (but maybe slightly less precise) function. For now we just get
/// a wrapper on `libm`.

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
