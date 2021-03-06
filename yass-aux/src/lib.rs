// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(test)]
mod tests;

mod num_aux;

// Parse
pub fn parse_bool(atom: &str) -> Option<bool> {
    match atom {
        "false" => Some(false),
        "true" => Some(true),
        _ => None,
    }
}

macro_rules! parse_signed_int {
    ($sint:ident, $uint:ident, $atom:expr) => {{
        enum State {
            Beginning,
            AfterSign,
            Digits,
        }
        
        let mut sign = false;
        let mut num: $uint = 0;
        
        let mut iter = $atom.chars();
        let mut state = State::Beginning;
        loop {
            match state {
                State::Beginning => {
                    match iter.next() {
                        Some('-') => {
                            sign = true;
                            state = State::AfterSign;
                        }
                        Some('+') => state = State::AfterSign,
                        Some(chr @ '0' ... '9') => {
                            num = (chr as u8 - b'0') as $uint;
                            state = State::Digits;
                        }
                        Some(_) | None => return None,
                    }
                }
                State::AfterSign => {
                    match iter.next() {
                        Some(chr @ '0' ... '9') => {
                            num = (chr as u8 - b'0') as $uint;
                            state = State::Digits;
                        }
                        Some(_) | None => return None,
                    }
                }
                State::Digits => {
                    match iter.next() {
                        Some(chr @ '0' ... '9') => {
                            num = num.checked_mul(10)?.checked_add((chr as u8 - b'0') as $uint)?;
                        }
                        Some(_) => return None,
                        None => break,
                    }
                }
            }
        }
        
        if sign {
            if num == ($sint::min_value() as $uint) {
                Some($sint::min_value())
            } else if num <= ($sint::max_value() as $uint) {
                Some(-(num as $sint))
            } else {
                None
            }
        } else {
            if num <= ($sint::max_value() as $uint) {
                Some(num as $sint)
            } else {
                None
            }
        }
    }};
}

pub fn parse_i32(atom: &str) -> Option<i32> {
    parse_signed_int!(i32, u32, atom)
}

pub fn parse_i64(atom: &str) -> Option<i64> {
    parse_signed_int!(i64, u64, atom)
}

macro_rules! parse_unsigned_int {
    ($uint:ident, $atom:expr) => {{
        enum State {
            Beginning,
            Digits,
        }
        
        let mut num: $uint = 0;
        
        let mut iter = $atom.chars();
        let mut state = State::Beginning;
        loop {
            match state {
                State::Beginning => {
                    match iter.next() {
                        Some(chr @ '0' ... '9') => {
                            num = (chr as u8 - b'0') as $uint;
                            state = State::Digits;
                        }
                        Some(_) | None => return None,
                    }
                }
                State::Digits => {
                    match iter.next() {
                        Some(chr @ '0' ... '9') => {
                            num = num.checked_mul(10)?.checked_add((chr as u8 - b'0') as $uint)?;
                        }
                        Some(_) => return None,
                        None => break,
                    }
                }
            }
        }
        
        Some(num)
    }};
}

pub fn parse_u32(atom: &str) -> Option<u32> {
    parse_unsigned_int!(u32, atom)
}

pub fn parse_u64(atom: &str) -> Option<u64> {
    parse_unsigned_int!(u64, atom)
}

fn parse_float_generic<T>(atom: &str) -> Option<T>
    where T: num_aux::dec2flt::rawfp::RawFloat
{
    enum State {
        Beginning,
        AfterSign,
        IntegerDigits,
        AfterDot,
        FractionalDigits,
        ExponentBeginning,
        AfterExponentSign,
        ExponentDigits,
    }
    
    let mut sign = false;
    let mut int_begin: usize = 0;
    let mut int_len: usize = 0;
    let mut int_is_zero = true;
    let mut frac_begin: usize = 0;
    let mut frac_len: usize = 0;
    let mut frac_is_zero = true;
    let mut exp_abs: u64 = 0;
    let mut exp_sign = false;
    
    let mut iter = atom.chars();
    let mut state = State::Beginning;
    let mut i = 0;
    loop {
        match state {
            State::Beginning => {
                match iter.next() {
                    Some('-') => {
                        sign = true;
                        state = State::AfterSign;
                    }
                    Some('+') => state = State::AfterSign,
                    Some(chr @ '0' ... '9') => {
                        int_begin = i;
                        int_len = 1;
                        int_is_zero &= chr == '0';
                        state = State::IntegerDigits;
                    }
                    Some('.') => state = State::AfterDot,
                    Some(_) | None => return None,
                }
            },
            State::AfterSign => {
                match iter.next() {
                    Some(chr @ '0' ... '9') => {
                        int_begin = i;
                        int_len = 1;
                        int_is_zero &= chr == '0';
                        state = State::IntegerDigits;
                    }
                    Some('.') => state = State::AfterDot,
                    Some(_) | None => return None,
                }
            }
            State::IntegerDigits => {
                match iter.next() {
                    Some(chr @ '0' ... '9') => {
                        int_len += 1;
                        int_is_zero &= chr == '0';
                    }
                    Some('.') => state = State::AfterDot,
                    Some('e') | Some('E') => state = State::ExponentBeginning,
                    Some(_) => return None,
                    None => break,
                }
            },
            State::AfterDot => {
                match iter.next() {
                    Some(chr @ '0' ... '9') => {
                        frac_begin = i;
                        frac_len = 1;
                        frac_is_zero &= chr == '0';
                        state = State::FractionalDigits;
                    }
                    Some('e') | Some('E') => state = State::ExponentBeginning,
                    Some(_) => return None,
                    None => break,
                }
            }
            State::FractionalDigits => {
                match iter.next() {
                    Some(chr @ '0' ... '9') => {
                        frac_len += 1;
                        frac_is_zero &= chr == '0';
                    }
                    Some('e') | Some('E') => state = State::ExponentBeginning,
                    Some(_) => return None,
                    None => break,
                }
            }
            State::ExponentBeginning => {
                match iter.next() {
                    Some('-') => {
                        exp_sign = true;
                        state = State::AfterExponentSign;
                    }
                    Some('+') => state = State::AfterExponentSign,
                    Some(chr @ '0' ... '9') => {
                        exp_abs = (chr as u8 - b'0') as u64;
                        state = State::ExponentDigits;
                    }
                    Some(_) | None => return None,
                }
            }
            State::AfterExponentSign => {
                match iter.next() {
                    Some(chr @ '0' ... '9') => {
                        exp_abs = (chr as u8 - b'0') as u64;
                        state = State::ExponentDigits;
                    }
                    Some(_) | None => return None,
                }
            }
            State::ExponentDigits => {
                match iter.next() {
                    Some(chr @ '0' ... '9') => {
                        if exp_abs != u64::max_value() {
                            exp_abs = exp_abs.checked_mul(10)
                                .and_then(|x| x.checked_add((chr as u8 - b'0') as u64))
                                .unwrap_or(u64::max_value());
                        }
                    }
                    Some(_) => return None,
                    None => break,
                }
            }
        }
        i += 1;
    }
    
    if int_is_zero && frac_is_zero {
        if int_len == 0 && frac_len == 0 {
            // Don't allow dots without digits (., .e10)
            None
        } else {
            Some(T::ZERO)
        }
    } else {
        // Less than 18 digits
        if exp_abs > 99999999999999999 {
            if exp_sign {
                Some(T::ZERO)
            } else {
                None
            }
        } else {
            let atom = atom.as_bytes();
            let integral = &atom[int_begin .. (int_begin + int_len)];
            let fractional = &atom[frac_begin .. (frac_begin + frac_len)];
            let exp = if exp_sign { -(exp_abs as i64) } else { exp_abs as i64 };
            
            let parsed_dec = num_aux::dec2flt::parse::Decimal::new(integral, fractional, exp);
            num_aux::dec2flt::convert::<T>(parsed_dec).and_then(|r| {
                if r.is_finite() {
                    Some(if sign { -r } else { r })
                } else {
                    None
                }
            })
        }
    }
}

pub fn parse_f64(atom: &str) -> Option<f64> {
    parse_float_generic::<f64>(atom)
}

fn hex_digit_to_u8(chr: char) -> Option<u8> {
    if chr >= '0' && chr <= '9' {
        Some(chr as u8 - b'0')
    } else if chr >= 'A' && chr <= 'F' {
        Some(chr as u8 - b'A' + 10)
    } else if chr >= 'a' && chr <= 'f' {
        Some(chr as u8 - b'a' + 10)
    } else {
        None
    }
}

pub fn parse_byte_string(atom: &str) -> Option<Vec<u8>> {
    enum State {
        Beginning,
        Normal,
        AfterBackslash,
        HexEscape1,
        HexEscape2(u8),
        Ending,
    }
    
    let mut string = Vec::new();
    
    let mut iter = atom.chars();
    let mut state = State::Beginning;
    loop {
        match state {
            State::Beginning => {
                match iter.next() {
                    Some('"') => state = State::Normal,
                    Some(_) | None => return None,
                }
            }
            State::Normal => {
                match iter.next() {
                    Some('\\') => state = State::AfterBackslash,
                    Some('"') => state = State::Ending,
                    Some(chr) => {
                        let mut utf8_buf = [0; 4];
                        string.extend_from_slice(chr.encode_utf8(&mut utf8_buf).as_bytes());
                    }
                    None => return None,
                }
            }
            State::AfterBackslash => {
                match iter.next() {
                    Some('t') => {
                        string.push(b'\t');
                        state = State::Normal;
                    }
                    Some('n') => {
                        string.push(b'\n');
                        state = State::Normal;
                    }
                    Some('r') => {
                        string.push(b'\r');
                        state = State::Normal;
                    }
                    Some('"') => {
                        string.push(b'"');
                        state = State::Normal;
                    }
                    Some('\\') => {
                        string.push(b'\\');
                        state = State::Normal;
                    }
                    Some('x') => {
                        state = State::HexEscape1;
                    }
                    Some(_) | None => return None,
                }
            }
            State::HexEscape1 => {
                match iter.next() {
                    Some(chr) => {
                        let hex1 = hex_digit_to_u8(chr)?;
                        state = State::HexEscape2(hex1);
                    }
                    None => return None,
                }
            }
            State::HexEscape2(hex1) => {
                match iter.next() {
                    Some(chr) => {
                        let hex2 = hex_digit_to_u8(chr)?;
                        string.push((hex1 << 4) | hex2);
                        state = State::Normal;
                    }
                    None => return None,
                }
            }
            State::Ending => {
                match iter.next() {
                    None => return Some(string),
                    _ => return None,
                }
            }
        }
    }
}

pub fn parse_ascii_string(atom: &str) -> Option<String> {
    enum State {
        Beginning,
        Normal,
        AfterBackslash,
        HexEscape1,
        HexEscape2(u8),
        Ending,
    }
    
    let mut string = String::new();
    
    let mut iter = atom.chars();
    let mut state = State::Beginning;
    loop {
        match state {
            State::Beginning => {
                match iter.next() {
                    Some('"') => state = State::Normal,
                    Some(_) | None => return None,
                }
            }
            State::Normal => {
                match iter.next() {
                    Some('\\') => state = State::AfterBackslash,
                    Some('"') => state = State::Ending,
                    Some(chr @ '\x00' ... '\x7F') => string.push(chr),
                    Some(_) | None => return None,
                }
            }
            State::AfterBackslash => {
                match iter.next() {
                    Some('t') => {
                        string.push('\t');
                        state = State::Normal;
                    }
                    Some('n') => {
                        string.push('\n');
                        state = State::Normal;
                    }
                    Some('r') => {
                        string.push('\r');
                        state = State::Normal;
                    }
                    Some('"') => {
                        string.push('"');
                        state = State::Normal;
                    }
                    Some('\\') => {
                        string.push('\\');
                        state = State::Normal;
                    }
                    Some('x') => {
                        state = State::HexEscape1;
                    }
                    Some(_) | None => return None,
                }
            }
            State::HexEscape1 => {
                match iter.next() {
                    Some(chr) => {
                        let hex1 = hex_digit_to_u8(chr)?;
                        state = State::HexEscape2(hex1);
                    }
                    None => return None,
                }
            }
            State::HexEscape2(hex1) => {
                match iter.next() {
                    Some(chr) => {
                        let hex2 = hex_digit_to_u8(chr)?;
                        let chr = (hex1 << 4) | hex2;
                        if chr > 0x7F {
                            return None;
                        }
                        string.push(chr as char);
                        state = State::Normal;
                    }
                    None => return None,
                }
            }
            State::Ending => {
                match iter.next() {
                    None => return Some(string),
                    _ => return None,
                }
            }
        }
    }
}

pub fn parse_utf8_string(atom: &str) -> Option<String> {
    enum State {
        Beginning,
        Normal,
        AfterBackslash,
        HexEscape1,
        HexEscape2(u8),
        UnicodeEscape1,
        UnicodeEscape2,
        UnicodeEscape3(u32),
        Ending,
    }
    
    let mut string = String::new();
    
    let mut iter = atom.chars();
    let mut state = State::Beginning;
    loop {
        match state {
            State::Beginning => {
                match iter.next() {
                    Some('"') => state = State::Normal,
                    Some(_) | None => return None,
                }
            }
            State::Normal => {
                match iter.next() {
                    Some('\\') => state = State::AfterBackslash,
                    Some('"') => state = State::Ending,
                    Some(chr) => string.push(chr),
                    None => return None,
                }
            }
            State::AfterBackslash => {
                match iter.next() {
                    Some('t') => {
                        string.push('\t');
                        state = State::Normal;
                    }
                    Some('n') => {
                        string.push('\n');
                        state = State::Normal;
                    }
                    Some('r') => {
                        string.push('\r');
                        state = State::Normal;
                    }
                    Some('"') => {
                        string.push('"');
                        state = State::Normal;
                    }
                    Some('\\') => {
                        string.push('\\');
                        state = State::Normal;
                    }
                    Some('x') => {
                        state = State::HexEscape1;
                    }
                    Some('u') => {
                        state = State::UnicodeEscape1;
                    }
                    Some(_) | None => return None,
                }
            }
            State::HexEscape1 => {
                match iter.next() {
                    Some(chr) => {
                        let hex1 = hex_digit_to_u8(chr)?;
                        state = State::HexEscape2(hex1);
                    }
                    None => return None,
                }
            }
            State::HexEscape2(hex1) => {
                match iter.next() {
                    Some(chr) => {
                        let hex2 = hex_digit_to_u8(chr)?;
                        let chr = (hex1 << 4) | hex2;
                        if chr > 0x7F {
                            return None;
                        }
                        string.push(chr as char);
                        state = State::Normal;
                    }
                    None => return None,
                }
            }
            State::UnicodeEscape1 => {
                match iter.next() {
                    Some('{') => state = State::UnicodeEscape2,
                    Some(_) | None => return None,
                }
            }
            State::UnicodeEscape2 => {
                match iter.next() {
                    Some(chr) => {
                        let hex1 = hex_digit_to_u8(chr)?;
                        state = State::UnicodeEscape3(hex1 as u32);
                    }
                    None => return None,
                }
            }
            State::UnicodeEscape3(current_hex) => {
                match iter.next() {
                    Some('}') => {
                        string.push(std::char::from_u32(current_hex)?);
                        state = State::Normal;
                    }
                    Some(chr) => {
                        if current_hex >= 0x10000000 {
                            return None;
                        }
                        let new_digit = hex_digit_to_u8(chr)? as u32;
                        state = State::UnicodeEscape3((current_hex << 4) | new_digit);
                    }
                    None => return None,
                }
            }
            State::Ending => {
                match iter.next() {
                    None => return Some(string),
                    _ => return None,
                }
            }
        }
    }
}

// Serialize
pub fn serialize_bool(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

macro_rules! serialize_signed_int {
    ($sint:ident, $uint:ident, $value:expr, $output:expr) => {
        let mut remaining_digits: $uint = {
            if $value == $sint::min_value() {
                $output.push('-');
                $sint::min_value() as $uint
            } else if $value < 0 {
                $output.push('-');
                (-$value) as $uint
            } else {
                $value as $uint
            }
        };
        let digits_beginning = $output.len();
        loop {
            let current_digit = (remaining_digits % 10) as u8;
            remaining_digits /= 10;
            $output.insert(digits_beginning, (current_digit + b'0') as char);
            if remaining_digits == 0 {
                break;
            }
        }
    }
}

pub fn serialize_i32(value: i32, output: &mut String) {
    serialize_signed_int!(i32, u32, value, output);
}

#[inline]
pub fn serialize_i32_as_string(value: i32) -> String {
    let mut output = String::new();
    serialize_i32(value, &mut output);
    output
}

pub fn serialize_i64(value: i64, output: &mut String) {
    serialize_signed_int!(i64, u64, value, output);
}

#[inline]
pub fn serialize_i64_as_string(value: i64) -> String {
    let mut output = String::new();
    serialize_i64(value, &mut output);
    output
}

macro_rules! serialize_unsigned_int {
    ($uint:ident, $value:expr, $output:expr) => {
        let mut remaining_digits: $uint = $value;
        let digits_beginning = $output.len();
        loop {
            let current_digit = (remaining_digits % 10) as u8;
            remaining_digits /= 10;
            $output.insert(digits_beginning, (current_digit + b'0') as char);
            if remaining_digits == 0 {
                break;
            }
        }
    }
}

pub fn serialize_u32(value: u32, output: &mut String) {
    serialize_unsigned_int!(u32, value, output);
}

#[inline]
pub fn serialize_u32_as_string(value: u32) -> String {
    let mut output = String::new();
    serialize_u32(value, &mut output);
    output
}

pub fn serialize_u64(value: u64, output: &mut String) {
    serialize_unsigned_int!(u64, value, output);
}

#[inline]
pub fn serialize_u64_as_string(value: u64) -> String {
    let mut output = String::new();
    serialize_u64(value, &mut output);
    output
}


fn serialize_float_aux(sign: bool, full_decoded: num_aux::flt2dec::decoder::FullDecoded, output: &mut String) {
    match full_decoded {
        num_aux::flt2dec::decoder::FullDecoded::Zero => {
            output.push_str("0.0");
        }
        num_aux::flt2dec::decoder::FullDecoded::Finite(ref decoded) => {
            if sign {
                output.push('-');
            }
            
            const ZEROS_THRESHOLD: u16 = 9;
            let mut buf = [0u8; num_aux::flt2dec::MAX_SIG_DIGITS];
            let (digits, exp) = num_aux::flt2dec::strategy::grisu::format_shortest(decoded, &mut buf);
            if exp <= 0 {
                let exp = (-exp) as u16;
                if exp > ZEROS_THRESHOLD {
                    output.push(buf[0] as char);
                    output.push('.');
                    if digits > 1 {
                        for &chr in buf[1 .. digits].iter() {
                            output.push(chr as char);
                        }
                    } else {
                        output.push('0');
                    }
                    output.push_str("e-");
                    serialize_u32((exp + 1) as u32, output);
                } else {
                    output.push_str("0.");
                    for _ in 0 .. exp {
                        output.push('0');
                    }
                    for &chr in buf[0 .. digits].iter() {
                        output.push(chr as char);
                    }
                }
            } else /*if exp>0*/ {
                let exp = exp as usize;
                if exp >= digits {
                    if exp > ZEROS_THRESHOLD as usize {
                        output.push(buf[0] as char);
                        output.push('.');
                        if digits > 1 {
                            for &chr in buf[1 .. digits].iter() {
                                output.push(chr as char);
                            }
                        } else {
                            output.push('0');
                        }
                        output.push('e');
                        serialize_u32((exp - 1) as u32, output);
                    } else {
                        for &chr in buf[0 .. digits].iter() {
                            output.push(chr as char);
                        }
                        for _ in 0 .. (exp - digits) {
                            output.push('0');
                        }
                        output.push_str(".0");
                    }
                } else /*if exp < digits*/ {
                    for &chr in buf[0 .. exp].iter() {
                        output.push(chr as char);
                    }
                    output.push('.');
                    for &chr in buf[exp .. digits].iter() {
                        output.push(chr as char);
                    }
                }
            }
        }
        _ => panic!("Number is not finite."),
    }
}

pub fn serialize_f64(value: f64, output: &mut String) {
    let (sign, full_decoded) = num_aux::flt2dec::decoder::decode(value);
    serialize_float_aux(sign, full_decoded, output);
}

#[inline]
pub fn serialize_f64_as_string(value: f64) -> String {
    let mut output = String::new();
    serialize_f64(value, &mut output);
    output
}


fn nibble_to_hex(nibble: u8) -> char {
    if nibble < 10 {
        (b'0' + nibble) as char
    } else {
        (b'a' + nibble - 10) as char
    }
}

pub fn serialize_byte_string(string: &[u8], output: &mut String) {
    output.push('"');
    for &chr in string {
        match chr {
            b'\t' => output.push_str("\\t"),
            b'\n' => output.push_str("\\n"),
            b'\r' => output.push_str("\\r"),
            b'"' => output.push_str("\\\""),
            b'\\' => output.push_str("\\\\"),
            0x20 ... 0x7E => output.push(chr as char),
            _ => {
                output.push_str("\\x");
                output.push(nibble_to_hex(chr >> 4));
                output.push(nibble_to_hex(chr & 0xF));
            }
        }
    }
    output.push('"');
}

#[inline]
pub fn serialize_byte_string_as_string(string: &[u8]) -> String {
    let mut output = String::new();
    serialize_byte_string(string, &mut output);
    output
}

pub fn serialize_ascii_string(string: &str, output: &mut String) {
    output.push('"');
    for &chr in string.as_bytes() {
        match chr {
            b'\t' => output.push_str("\\t"),
            b'\n' => output.push_str("\\n"),
            b'\r' => output.push_str("\\r"),
            b'"' => output.push_str("\\\""),
            b'\\' => output.push_str("\\\\"),
            0x20 ... 0x7E => output.push(chr as char),
            _ => {
                assert!(chr <= 0x7F, "Invalid ASCII character");
                output.push_str("\\x");
                output.push(nibble_to_hex(chr >> 4));
                output.push(nibble_to_hex(chr & 0xF));
            }
        }
    }
    output.push('"');
}

#[inline]
pub fn serialize_ascii_string_as_string(string: &str) -> String {
    let mut output = String::new();
    serialize_ascii_string(string, &mut output);
    output
}

pub fn serialize_utf8_string(string: &str, output: &mut String) {
    output.push('"');
    for chr in string.chars() {
        match chr {
            '\t' => output.push_str("\\t"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\x20' ... '\x7E' => output.push(chr as char),
            _ => {
                output.push_str("\\u{");
                let code_beginning = output.len();
                let mut remaining_digits = chr as u32;
                loop {
                    output.insert(code_beginning, nibble_to_hex((remaining_digits & 0xF) as u8));
                    remaining_digits >>= 4;
                    if remaining_digits == 0 {
                        break;
                    }
                }
                output.push('}');
            }
        }
    }
    output.push('"');
}

#[inline]
pub fn serialize_utf8_string_as_string(string: &str) -> String {
    let mut output = String::new();
    serialize_utf8_string(string, &mut output);
    output
}
