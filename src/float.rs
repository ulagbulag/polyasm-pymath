//! The `f64` methods of `std` for PolyASM builds, which link `core` and `alloc` alone; `libm`
//! computes each one.

pub(crate) trait F64Ext {
    fn ln(self) -> f64;
    fn exp(self) -> f64;
    fn powf(self, n: f64) -> f64;
    fn sqrt(self) -> f64;
    fn sin(self) -> f64;
    fn cos(self) -> f64;
    fn floor(self) -> f64;
    fn round(self) -> f64;
    fn mul_add(self, a: f64, b: f64) -> f64;
}

impl F64Ext for f64 {
    #[inline]
    fn ln(self) -> f64 {
        libm::log(self)
    }

    #[inline]
    fn exp(self) -> f64 {
        libm::exp(self)
    }

    #[inline]
    fn powf(self, n: f64) -> f64 {
        libm::pow(self, n)
    }

    #[inline]
    fn sqrt(self) -> f64 {
        libm::sqrt(self)
    }

    #[inline]
    fn sin(self) -> f64 {
        libm::sin(self)
    }

    #[inline]
    fn cos(self) -> f64 {
        libm::cos(self)
    }

    #[inline]
    fn floor(self) -> f64 {
        libm::floor(self)
    }

    #[inline]
    fn round(self) -> f64 {
        libm::round(self)
    }

    #[inline]
    fn mul_add(self, a: f64, b: f64) -> f64 {
        libm::fma(self, a, b)
    }
}
