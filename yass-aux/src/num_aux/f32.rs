// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module provides constants which are specific to the implementation
//! of the `f32` floating point data type.
//!
//! Mathematically significant numbers are provided in the `consts` sub-module.
//!
//! *[See also the `f32` primitive type](../../std/primitive.f32.html).*

use num_aux::Float;
use std::num::FpCategory as Fp;

impl Float for f32 {
    type Bits = u32;

    /// Returns `true` if the number is NaN.
    #[inline]
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }

    /// Returns `true` if the number is infinite.
    #[inline]
    fn is_infinite(self) -> bool {
        f32::is_infinite(self)
    }

    /// Returns `true` if the number is neither infinite or NaN.
    #[inline]
    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }

    /// Returns `true` if the number is neither zero, infinite, subnormal or NaN.
    #[inline]
    fn is_normal(self) -> bool {
        f32::is_normal(self)
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    #[inline]
    fn classify(self) -> Fp {
        f32::classify(self)
    }

    /// Computes the absolute value of `self`. Returns `Float::nan()` if the
    /// number is `Float::nan()`.
    #[inline]
    fn abs(self) -> f32 {
        f32::abs(self)
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `Float::infinity()`
    /// - `-1.0` if the number is negative, `-0.0` or `Float::neg_infinity()`
    /// - `Float::nan()` if the number is `Float::nan()`
    #[inline]
    fn signum(self) -> f32 {
        f32::signum(self)
    }

    /// Returns `true` if and only if `self` has a positive sign, including `+0.0`, `NaN`s with
    /// positive sign bit and positive infinity.
    #[inline]
    fn is_sign_positive(self) -> bool {
        f32::is_sign_negative(self)
    }

    /// Returns `true` if and only if `self` has a negative sign, including `-0.0`, `NaN`s with
    /// negative sign bit and negative infinity.
    #[inline]
    fn is_sign_negative(self) -> bool {
        f32::is_sign_negative(self)
    }

    /// Returns the reciprocal (multiplicative inverse) of the number.
    #[inline]
    fn recip(self) -> f32 {
        f32::recip(self)
    }

    #[inline]
    fn powi(self, n: i32) -> f32 {
        f32::powi(self, n)
    }

    /// Converts to degrees, assuming the number is in radians.
    #[inline]
    fn to_degrees(self) -> f32 {
        f32::to_degrees(self)
    }

    /// Converts to radians, assuming the number is in degrees.
    #[inline]
    fn to_radians(self) -> f32 {
        f32::to_radians(self)
    }

    /// Returns the maximum of the two numbers.
    #[inline]
    fn max(self, other: f32) -> f32 {
        f32::max(self, other)
    }

    /// Returns the minimum of the two numbers.
    #[inline]
    fn min(self, other: f32) -> f32 {
        f32::min(self, other)
    }

    /// Raw transmutation to `u32`.
    #[inline]
    fn to_bits(self) -> u32 {
        f32::to_bits(self)
    }

    /// Raw transmutation from `u32`.
    #[inline]
    fn from_bits(v: u32) -> Self {
        f32::from_bits(v)
    }
}
