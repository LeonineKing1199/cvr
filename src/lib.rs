//! `cvr` is a home-grown attempt at porting some of the functionality offered by `OpenCV` to Rust
//! in a way that emphasizes type-safety and functional composition.
//!

#![warn(clippy::pedantic)]

/// `srgb_to_linear` converts an `sRGB` gamma-corrected 8-bit pixel value into its corresponding
/// value in the linear `sRGB` color space as a `f32` mapped to the range `[0, 1]`.
///
/// This function is the inverse of `linear_to_srgb`.
///
/// Notes on the algorithm and the constants used can be found [here](https://en.wikipedia.org/wiki/SRGB).
///
#[must_use]
pub fn srgb_to_linear(u: u8) -> f32 {
    // 1/ 255.0 => 0.00392156863
    //
    let u = f32::from(u) * 0.003_921_569;

    if u <= 0.04045 {
        // 1 / 12.92 => 0.0773993808
        //
        u * 0.077_399_38
    } else {
        // 1/ 1.055 => 0.947867299
        //
        ((u + 0.055) * 0.947_867_3).powf(2.4)
    }
}

/// `linear_to_srgb` takes a `f32` linear `sRGB` pixel value in the range `[0, 1]` and encodes it as
/// an 8-bit value in the gamma-corrected `sRGB` space.
///
/// Note: if the gamma-corrected value exceeds `1.0` then it is automatically clipped and `255` is
/// returned.
///
/// This function is the inverse of `srgb_to_linear`.
///
/// Notes on the algorithm and the constants used can be found [here](https://en.wikipedia.org/wiki/SRGB).
///
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn linear_to_srgb(u: f32) -> u8 {
    let u = if u <= 0.003_130_8 {
        12.92 * u
    } else {
        // 1 / 2.4 => 0.416666667
        //
        1.055 * u.powf(0.416_666_66) - 0.055
    };

    if u >= 1.0 {
        return 255;
    }

    if u < 0.0 {
        return 0;
    }

    (255.0 * u).round() as u8
}

/// `Numeric` represents such types as `u8` and `f32`.
///
pub trait Numeric: Copy {}

impl Numeric for u8 {}
impl Numeric for f32 {}

pub mod rgb {
    /// `Iter` enables the simultaneous traversal of 3 separate channels of image data. It works
    /// with any type that can be converted to a `&[Numeric]`. Image data is returned pixel-by-pixel
    /// in a `[N; 3]` format with `(R, G, B)` ordering.
    ///
    pub struct Iter<'a, N>
    where
        N: crate::Numeric,
    {
        r: std::slice::Iter<'a, N>,
        g: std::slice::Iter<'a, N>,
        b: std::slice::Iter<'a, N>,
    }

    /// `new` constructs a new `Iter` using the backing `&[N]` of the types passed in by the user.
    ///
    /// # Example
    /// ```
    /// let r = vec![1, 2, 3];
    /// let g = vec![4, 5, 6];
    /// let b = vec![7, 8, 9];
    ///
    /// let rgb_iter = cvr::rgb::Iter::new(&r, &g, &b);
    /// ```
    ///
    impl<'a, N> Iter<'a, N>
    where
        N: crate::Numeric,
    {
        pub fn new<R>(r: &'a R, g: &'a R, b: &'a R) -> Self
        where
            R: std::convert::AsRef<[N]>,
        {
            Self {
                r: r.as_ref().iter(),
                g: g.as_ref().iter(),
                b: b.as_ref().iter(),
            }
        }
    }

    impl<'a, N> std::iter::Iterator for Iter<'a, N>
    where
        N: crate::Numeric,
    {
        type Item = [N; 3];

        fn next(&mut self) -> Option<Self::Item> {
            match (self.r.next(), self.g.next(), self.b.next()) {
                (Some(r), Some(g), Some(b)) => Some([*r, *g, *b]),
                _ => None,
            }
        }
    }

    pub mod iter {
        /// `SRGBToLinear` lazily converts 8-bit `sRGB` pixels to their linear floating point
        /// counterparts.
        ///
        pub struct SRGBToLinear<Iter>
        where
            Iter: std::iter::Iterator<Item = [u8; 3]>,
        {
            iter: Iter,
        }

        impl<Iter> std::iter::Iterator for SRGBToLinear<Iter>
        where
            Iter: std::iter::Iterator<Item = [u8; 3]>,
        {
            type Item = [f32; 3];

            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next().map(|[r, g, b]| {
                    [
                        crate::srgb_to_linear(r),
                        crate::srgb_to_linear(g),
                        crate::srgb_to_linear(b),
                    ]
                })
            }
        }

        /// `SRGBLinear` is the public trait `std::iter::Iterator` types implement to enable
        /// `.srgb_to_linear()` as an iterator adapter.
        ///
        pub trait SRGBLinear: std::iter::Iterator<Item = [u8; 3]>
        where
            Self: Sized,
        {
            fn srgb_to_linear(self) -> SRGBToLinear<Self> {
                SRGBToLinear { iter: self }
            }
        }

        impl<Iter> SRGBLinear for Iter where Iter: std::iter::Iterator<Item = [u8; 3]> {}

        /// `LinearToSRGBIter` lazily converts linear floating point `(R, G, B)` data into its
        /// 8-bit `sRGB` representation.
        ///
        pub struct LinearToSRGB<Iter>
        where
            Iter: std::iter::Iterator<Item = [f32; 3]>,
        {
            iter: Iter,
        }

        impl<Iter> std::iter::Iterator for LinearToSRGB<Iter>
        where
            Iter: std::iter::Iterator<Item = [f32; 3]>,
        {
            type Item = [u8; 3];

            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next().map(|[r, g, b]| {
                    [
                        crate::linear_to_srgb(r),
                        crate::linear_to_srgb(g),
                        crate::linear_to_srgb(b),
                    ]
                })
            }
        }

        /// `LinearToSRGB` is the public trait `std::iter::Iterator` types implement to enable
        /// `.linear_to_srgb()` as an iterator adapter.
        ///
        pub trait LinearSRGB: std::iter::Iterator<Item = [f32; 3]>
        where
            Self: Sized,
        {
            fn linear_to_srgb(self) -> LinearToSRGB<Self> {
                LinearToSRGB { iter: self }
            }
        }

        impl<Iter> LinearSRGB for Iter where Iter: std::iter::Iterator<Item = [f32; 3]> {}
    }
}
