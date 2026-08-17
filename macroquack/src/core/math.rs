// use rem_euclid instead of rem or % operator.
pub fn wrapf(value: f32, min: f32, max: f32) -> f32 {
    let range = max - min;
    if range == 0f32 {
        return min;
    };

    min + (value - min).rem_euclid(range)
}

pub trait MathWrap {
    fn wrapf(self, min: Self, max: Self) -> Self;
}

impl MathWrap for f64 {
    fn wrapf(self, min: f64, max: f64) -> f64 {
        let range = max - min;
        if range == 0.0 {
            min
        } else {
            min + (self - min).rem_euclid(range)
        }
    }
}

impl MathWrap for f32 {
    fn wrapf(self, min: f32, max: f32) -> f32 {
        let range = max - min;
        if range == 0.0 {
            min
        } else {
            min + (self - min).rem_euclid(range)
        }
    }
}
