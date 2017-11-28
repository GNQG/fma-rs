//! Tiny crate to use FMA(Fused Multiply-Add) instruction easily.
#![feature(core_intrinsics)]
extern crate core;
use core::intrinsics::{fmaf32, fmaf64};

#[inline]
pub fn fma<T: Fma>(a: T, b: T, c: T) -> T {
    Fma::fma(a, b, c)
}

pub trait Fma {
    fn fma(a: Self, b: Self, c: Self) -> Self;
}

impl Fma for f64 {
    #[inline]
    fn fma(a: f64, b: f64, c: f64) -> f64 {
        unsafe { fmaf64(a, b, c) }
    }
}

impl Fma for f32 {
    #[inline]
    fn fma(a: f32, b: f32, c: f32) -> f32 {
        unsafe { fmaf32(a, b, c) }
    }
}


#[cfg(test)]
mod tests {
    use super::fma;
    #[test]
    fn test() {
        assert_eq!(fma(1., 2., 3.), 5.);
        assert_eq!(fma(2f64.powi(-1000),
                       2f64.powi(-75),
                       2f64.powi(-1000) * 2f64.powi(-74)),
                   2f64.powi(-1000) * 2f64.powi(-73));
    }
}
