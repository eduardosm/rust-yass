// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

Floating-point number to decimal conversion routines.

# Problem statement

We are given the floating-point number `v = f * 2^e` with an integer `f`,
and its bounds `minus` and `plus` such that any number between `v - minus` and
`v + plus` will be rounded to `v`. For the simplicity we assume that
this range is exclusive. Then we would like to get the unique decimal
representation `V = 0.d[0..n-1] * 10^k` such that:

- `d[0]` is non-zero.

- It's correctly rounded when parsed back: `v - minus < V < v + plus`.
  Furthermore it is shortest such one, i.e. there is no representation
  with less than `n` digits that is correctly rounded.

- It's closest to the original value: `abs(V - v) <= 10^(k-n) / 2`. Note that
  there might be two representations satisfying this uniqueness requirement,
  in which case some tie-breaking mechanism is used.

We will call this mode of operation as to the *shortest* mode. This mode is used
when there is no additional constraint, and can be thought as a "natural" mode
as it matches the ordinary intuition (it at least prints `0.1f32` as "0.1").

We have two more modes of operation closely related to each other. In these modes
we are given either the number of significant digits `n` or the last-digit
limitation `limit` (which determines the actual `n`), and we would like to get
the representation `V = 0.d[0..n-1] * 10^k` such that:

- `d[0]` is non-zero, unless `n` was zero in which case only `k` is returned.

- It's closest to the original value: `abs(V - v) <= 10^(k-n) / 2`. Again,
  there might be some tie-breaking mechanism.

When `limit` is given but not `n`, we set `n` such that `k - n = limit`
so that the last digit `d[n-1]` is scaled by `10^(k-n) = 10^limit`.
If such `n` is negative, we clip it to zero so that we will only get `k`.
We are also limited by the supplied buffer. This limitation is used to print
the number up to given number of fractional digits without knowing
the correct `k` beforehand.

We will call the mode of operation requiring `n` as to the *exact* mode,
and one requiring `limit` as to the *fixed* mode. The exact mode is a subset of
the fixed mode: the sufficiently large last-digit limitation will eventually fill
the supplied buffer and let the algorithm to return.

# Implementation overview

It is easy to get the floating point printing correct but slow (Russ Cox has
[demonstrated](http://research.swtch.com/ftoa) how it's easy), or incorrect but
fast (naïve division and modulo). But it is surprisingly hard to print
floating point numbers correctly *and* efficiently.

There are two classes of algorithms widely known to be correct.

- The "Dragon" family of algorithm is first described by Guy L. Steele Jr. and
  Jon L. White. They rely on the fixed-size big integer for their correctness.
  A slight improvement was found later, which is posthumously described by
  Robert G. Burger and R. Kent Dybvig. David Gay's `dtoa.c` routine is
  a popular implementation of this strategy.

- The "Grisu" family of algorithm is first described by Florian Loitsch.
  They use very cheap integer-only procedure to determine the close-to-correct
  representation which is at least guaranteed to be shortest. The variant,
  Grisu3, actively detects if the resulting representation is incorrect.

We implement both algorithms with necessary tweaks to suit our requirements.
In particular, published literatures are short of the actual implementation
difficulties like how to avoid arithmetic overflows. Each implementation,
available in `strategy::dragon` and `strategy::grisu` respectively,
extensively describes all necessary justifications and many proofs for them.
(It is still difficult to follow though. You have been warned.)

Both implementations expose two public functions:

- `format_shortest(decoded, buf)`, which always needs at least
  `MAX_SIG_DIGITS` digits of buffer. Implements the shortest mode.

- `format_exact(decoded, buf, limit)`, which accepts as small as
  one digit of buffer. Implements exact and fixed modes.

They try to fill the `u8` buffer with digits and returns the number of digits
written and the exponent `k`. They are total for all finite `f32` and `f64`
inputs (Grisu internally falls back to Dragon if necessary).

*/

// while this is extensively documented, this is in principle private which is
// only made public for testing. do not expose us.
#![doc(hidden)]

pub use self::decoder::{decode, DecodableFloat, FullDecoded, Decoded};

pub mod estimator;
pub mod decoder;

/// Digit-generation algorithms.
pub mod strategy {
    pub mod dragon;
    pub mod grisu;
}

/// The minimum size of buffer necessary for the shortest mode.
///
/// It is a bit non-trivial to derive, but this is one plus the maximal number of
/// significant decimal digits from formatting algorithms with the shortest result.
/// The exact formula is `ceil(# bits in mantissa * log_10 2 + 1)`.
pub const MAX_SIG_DIGITS: usize = 17;

/// When `d[..n]` contains decimal digits, increase the last digit and propagate carry.
/// Returns a next digit when it causes the length change.
#[doc(hidden)]
pub fn round_up(d: &mut [u8], n: usize) -> Option<u8> {
    match d[..n].iter().rposition(|&c| c != b'9') {
        Some(i) => { // d[i+1..n] is all nines
            d[i] += 1;
            for j in i+1..n { d[j] = b'0'; }
            None
        }
        None if n > 0 => { // 999..999 rounds to 1000..000 with an increased exponent
            d[0] = b'1';
            for j in 1..n { d[j] = b'0'; }
            Some(b'0')
        }
        None => { // an empty buffer rounds up (a bit strange but reasonable)
            Some(b'1')
        }
    }
}
