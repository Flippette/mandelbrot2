use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    #[must_use]
    #[inline]
    pub fn new<T: Into<f64>, U: Into<f64>>(re: T, im: U) -> Self {
        Self {
            re: re.into(),
            im: im.into(),
        }
    }

    #[must_use]
    #[inline]
    pub fn square(&self) -> Self {
        Self::new(
            self.re * self.re - self.im * self.im,
            2.0 * self.re * self.im,
        )
    }
}

impl Add<Self> for Complex64 {
    type Output = Self;

    #[must_use]
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}
