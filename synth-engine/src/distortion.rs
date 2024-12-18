use libm::{tanhf, logf, expf};

fn smoothstep(x: f32) -> f32 {
    let x = x.clamp(0., 1.);
    let x2 = x * x;
    let x3 = x * x2;

    3. * x2 - 2. * x3
}

#[allow(non_snake_case)]
pub enum DistortionType {
    Tanh,
    Smoothstep,
    Diodelike {
        a: f32,
        b: f32,
        c: f32,
        ln_R: f32,
        R: f32,
    },
    Logistic(f32),
}

#[allow(non_snake_case)]
pub fn make_diodelike(R: f32, n: f32) -> DistortionType {
    let V_T = 0.026;
    let I_S = 10e-12;

    let a = logf(I_S * R / (n * V_T));
    let b = 1. / (n * V_T);
    let c = n * V_T / R;
    let ln_R = logf(R);

    DistortionType::Diodelike { a, b, c, ln_R, R }
}

pub trait Distort {
    fn distort(&self, tpe: &Option<DistortionType>) -> Self;
}

impl Distort for f32 {
    fn distort(&self, tpe: &Option<DistortionType>) -> Self {
        use DistortionType::*;

        match tpe {
            None => *self,
            Some(Tanh) => tanhf(*self),
            Some(Smoothstep) => smoothstep(0.5 + self / 2.) * 2. - 1.,
            Some(Diodelike { a, b, c, ln_R, R }) => {
                let x = ln_R + a + b * self;
                if x <= 0. {
                    *self
                } else {
                    #[allow(non_snake_case)]
                    let I = c * (x - logf(x));
                    *self - I * R
                }
            }
            Some(Logistic(amount)) => 2. / (1. + expf(-amount * self)) - 1.,
        }
    }
}
