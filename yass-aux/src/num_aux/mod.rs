// Taken from rust 1.25.0 libcore.
//
// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod f32;
mod f64;
mod bignum;
mod diy_float;
pub mod dec2flt;
pub mod flt2dec;

use std::num::FpCategory;

/// A built-in floating point number.
#[doc(hidden)]
pub trait Float: Sized {
    /// Type used by `to_bits` and `from_bits`.
    type Bits;

    /// Returns `true` if this value is NaN and false otherwise.
    fn is_nan(self) -> bool;
    /// Returns `true` if this value is positive infinity or negative infinity and
    /// false otherwise.
    fn is_infinite(self) -> bool;
    /// Returns `true` if this number is neither infinite nor NaN.
    fn is_finite(self) -> bool;
    /// Returns `true` if this number is neither zero, infinite, denormal, or NaN.
    fn is_normal(self) -> bool;
    /// Returns the category that this number falls into.
    fn classify(self) -> FpCategory;

    /// Computes the absolute value of `self`. Returns `Float::nan()` if the
    /// number is `Float::nan()`.
    fn abs(self) -> Self;
    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `Float::infinity()`
    /// - `-1.0` if the number is negative, `-0.0` or `Float::neg_infinity()`
    /// - `Float::nan()` if the number is `Float::nan()`
    fn signum(self) -> Self;

    /// Returns `true` if `self` is positive, including `+0.0` and
    /// `Float::infinity()`.
    fn is_sign_positive(self) -> bool;
    /// Returns `true` if `self` is negative, including `-0.0` and
    /// `Float::neg_infinity()`.
    fn is_sign_negative(self) -> bool;

    /// Take the reciprocal (inverse) of a number, `1/x`.
    fn recip(self) -> Self;

    /// Raise a number to an integer power.
    ///
    /// Using this function is generally faster than using `powf`
    fn powi(self, n: i32) -> Self;

    /// Convert radians to degrees.
    fn to_degrees(self) -> Self;
    /// Convert degrees to radians.
    fn to_radians(self) -> Self;

    /// Returns the maximum of the two numbers.
    fn max(self, other: Self) -> Self;
    /// Returns the minimum of the two numbers.
    fn min(self, other: Self) -> Self;

    /// Raw transmutation to integer.
    fn to_bits(self) -> Self::Bits;
    /// Raw transmutation from integer.
    fn from_bits(v: Self::Bits) -> Self;
}
