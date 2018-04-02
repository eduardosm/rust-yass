// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod yass_aux {
    pub use ::*;
}

#[test]
fn test_parse_bool() {
    assert_eq!(yass_aux::parse_bool("true"), Some(true));
    assert_eq!(yass_aux::parse_bool("false"), Some(false));
    
    assert_eq!(yass_aux::parse_bool("TRUE"), None);
    assert_eq!(yass_aux::parse_bool("FALSE"), None);
    assert_eq!(yass_aux::parse_bool("abc"), None);
    assert_eq!(yass_aux::parse_bool(""), None);
}

#[test]
fn test_parse_i32() {
    assert_eq!(yass_aux::parse_i32("0"), Some(0));
    assert_eq!(yass_aux::parse_i32("-0"), Some(0));
    assert_eq!(yass_aux::parse_i32("+0"), Some(0));
    assert_eq!(yass_aux::parse_i32("1"), Some(1));
    assert_eq!(yass_aux::parse_i32("-1"), Some(-1));
    assert_eq!(yass_aux::parse_i32("+1"), Some(1));
    
    assert_eq!(yass_aux::parse_i32("-2147483648"), Some(-2147483648));
    assert_eq!(yass_aux::parse_i32("2147483647"), Some(2147483647));
    assert_eq!(yass_aux::parse_i32("-2147483647"), Some(-2147483647));
    
    assert_eq!(yass_aux::parse_i32("-2147483649"), None);
    assert_eq!(yass_aux::parse_i32("2147483648"), None);
    
    assert_eq!(yass_aux::parse_i32(""), None);
}

#[test]
fn test_parse_i64() {
    assert_eq!(yass_aux::parse_i64("0"), Some(0));
    assert_eq!(yass_aux::parse_i64("-0"), Some(0));
    assert_eq!(yass_aux::parse_i64("+0"), Some(0));
    assert_eq!(yass_aux::parse_i64("1"), Some(1));
    assert_eq!(yass_aux::parse_i64("-1"), Some(-1));
    assert_eq!(yass_aux::parse_i64("+1"), Some(1));
    
    assert_eq!(yass_aux::parse_i64("-9223372036854775808"), Some(-9223372036854775808));
    assert_eq!(yass_aux::parse_i64("9223372036854775807"), Some(9223372036854775807));
    assert_eq!(yass_aux::parse_i64("-9223372036854775807"), Some(-9223372036854775807));

    assert_eq!(yass_aux::parse_i64("-9223372036854775809"), None);
    assert_eq!(yass_aux::parse_i64("9223372036854775808"), None);
    
    assert_eq!(yass_aux::parse_i64(""), None);
}

#[test]
fn test_parse_u32() {
    assert_eq!(yass_aux::parse_u32("0"), Some(0));
    assert_eq!(yass_aux::parse_u32("1"), Some(1));
    
    assert_eq!(yass_aux::parse_u32("4294967295"), Some(4294967295));
    
    assert_eq!(yass_aux::parse_u32(""), None);
    assert_eq!(yass_aux::parse_u32("4294967296"), None);
}

#[test]
fn test_parse_u64() {
    assert_eq!(yass_aux::parse_u64("0"), Some(0));
    assert_eq!(yass_aux::parse_u64("1"), Some(1));
    
    assert_eq!(yass_aux::parse_u64("18446744073709551615"), Some(18446744073709551615));
    
    assert_eq!(yass_aux::parse_u64(""), None);
    assert_eq!(yass_aux::parse_u64("18446744073709551616"), None);
}

#[test]
fn test_parse_f64() {
    fn approx(x: f64, y: f64) -> bool {
        (x - y).abs() < (10.0 * ::std::f64::EPSILON * f64::max(x.abs(), y.abs()))
    }
    
    assert_eq!(yass_aux::parse_f64("0"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("0.0"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("1"), Some(1.0));
    assert_eq!(yass_aux::parse_f64("1.0"), Some(1.0));
    assert_eq!(yass_aux::parse_f64("-1"), Some(-1.0));
    assert_eq!(yass_aux::parse_f64("-1.0"), Some(-1.0));
    assert_eq!(yass_aux::parse_f64("+1"), Some(1.0));
    assert_eq!(yass_aux::parse_f64("+1.0"), Some(1.0));
    assert_eq!(yass_aux::parse_f64(".0"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("+.0"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("-.0"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("0."), Some(0.0));
    assert_eq!(yass_aux::parse_f64("+0."), Some(0.0));
    assert_eq!(yass_aux::parse_f64("-0."), Some(0.0));
    
    assert!(approx(yass_aux::parse_f64("1.1").unwrap(), 1.1));
    assert!(approx(yass_aux::parse_f64("1e2").unwrap(), 1.0e2));
    assert!(approx(yass_aux::parse_f64("1E2").unwrap(), 1.0e2));
    assert!(approx(yass_aux::parse_f64("1.e1").unwrap(), 1.0e1));
    assert!(approx(yass_aux::parse_f64("1.E1").unwrap(), 1.0e1));
    assert!(approx(yass_aux::parse_f64(".1e1").unwrap(), 0.1e1));
    assert!(approx(yass_aux::parse_f64(".1E1").unwrap(), 0.1e1));
    assert!(approx(yass_aux::parse_f64("1.1e2").unwrap(), 1.1e2));
    assert!(approx(yass_aux::parse_f64("1.1e+2").unwrap(), 1.1e2));
    assert!(approx(yass_aux::parse_f64("1.1e-2").unwrap(), 1.1e-2));
    assert!(approx(yass_aux::parse_f64("123e-90").unwrap(), 123.0e-90));
    assert!(approx(yass_aux::parse_f64("123e+90").unwrap(), 123.0e90));
    assert!(approx(yass_aux::parse_f64("1000000000000").unwrap(), 1000000000000.0));
    assert!(approx(yass_aux::parse_f64("0.000000000001").unwrap(), 0.000000000001));
    assert!(approx(yass_aux::parse_f64("1.23456789").unwrap(), 1.23456789));
    assert!(approx(yass_aux::parse_f64("1.2345678987654321").unwrap(), 1.2345678987654321));
    assert!(approx(yass_aux::parse_f64("480.144468355317204515627862").unwrap(), 480.144468355317204515627862));
    assert!(approx(yass_aux::parse_f64("480.144468355317204515627862e-250").unwrap(), 480.144468355317204515627862e-250));
    assert!(approx(yass_aux::parse_f64("480.144468355317204515627862e+250").unwrap(), 480.144468355317204515627862e+250));
    assert_eq!(yass_aux::parse_f64("0e99999999999999999"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("1e-99999999999999999"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("0e9999999999999999999999999999999999"), Some(0.0));
    assert_eq!(yass_aux::parse_f64("1e-9999999999999999999999999999999999"), Some(0.0));
    
    assert_eq!(yass_aux::parse_f64(""), None);
    assert_eq!(yass_aux::parse_f64("."), None);
    assert_eq!(yass_aux::parse_f64("+."), None);
    assert_eq!(yass_aux::parse_f64("-."), None);
    assert_eq!(yass_aux::parse_f64("e1"), None);
    assert_eq!(yass_aux::parse_f64("E1"), None);
    assert_eq!(yass_aux::parse_f64("+e1"), None);
    assert_eq!(yass_aux::parse_f64("+E1"), None);
    assert_eq!(yass_aux::parse_f64("-e1"), None);
    assert_eq!(yass_aux::parse_f64("-E1"), None);
    assert_eq!(yass_aux::parse_f64("1e1000"), None);
    assert_eq!(yass_aux::parse_f64("1e9999999999999999999999999999999999"), None);
    assert_eq!(yass_aux::parse_f64("1e"), None);
    assert_eq!(yass_aux::parse_f64("1E"), None);
}

#[test]
fn test_parse_byte_string() {
    assert_eq!(yass_aux::parse_byte_string(r#""""#).unwrap(), b"");
    assert_eq!(yass_aux::parse_byte_string(r#""123""#).unwrap(), b"123");
    assert_eq!(yass_aux::parse_byte_string(r#"" \\ ""#).unwrap(), b" \\ ");
    assert_eq!(yass_aux::parse_byte_string(r#"" \" ""#).unwrap(), b" \" ");
    assert_eq!(yass_aux::parse_byte_string(r#"" \r\n\t ""#).unwrap(), b" \r\n\t ");
    assert_eq!(yass_aux::parse_byte_string(r#"" \x00\xFF ""#).unwrap(), b" \x00\xFF ");
    assert_eq!(yass_aux::parse_byte_string(r#"" \xaB\xCd ""#).unwrap(), b" \xAB\xCD ");
    
    assert_eq!(yass_aux::parse_byte_string(r#"""#), None);
    assert_eq!(yass_aux::parse_byte_string(r#""" "#), None);
    assert_eq!(yass_aux::parse_byte_string(r#""\""#), None);
    assert_eq!(yass_aux::parse_byte_string(r#"" \M ""#), None);
    assert_eq!(yass_aux::parse_byte_string(r#"" \xT0 ""#), None);
    assert_eq!(yass_aux::parse_byte_string(r#"" \x0T ""#), None);
}

#[test]
fn test_parse_ascii_string() {
    assert_eq!(yass_aux::parse_ascii_string(r#""""#).unwrap(), "");
    assert_eq!(yass_aux::parse_ascii_string(r#""123""#).unwrap(), "123");
    assert_eq!(yass_aux::parse_ascii_string(r#"" \\ ""#).unwrap(), " \\ ");
    assert_eq!(yass_aux::parse_ascii_string(r#"" \" ""#).unwrap(), " \" ");
    assert_eq!(yass_aux::parse_ascii_string(r#"" \r\n\t ""#).unwrap(), " \r\n\t ");
    assert_eq!(yass_aux::parse_ascii_string(r#"" \x00\x7F ""#).unwrap(), " \x00\x7F ");
    assert_eq!(yass_aux::parse_ascii_string(r#"" \x1B\x1d ""#).unwrap(), " \x1B\x1D ");
    
    assert_eq!(yass_aux::parse_ascii_string("\" \u{FF} \""), None);
    assert_eq!(yass_aux::parse_ascii_string(r#"""#), None);
    assert_eq!(yass_aux::parse_ascii_string(r#""" "#), None);
    assert_eq!(yass_aux::parse_ascii_string(r#""\""#), None);
    assert_eq!(yass_aux::parse_ascii_string(r#"" \M ""#), None);
    assert_eq!(yass_aux::parse_ascii_string(r#"" \xT0 ""#), None);
    assert_eq!(yass_aux::parse_ascii_string(r#"" \x0T ""#), None);
    assert_eq!(yass_aux::parse_ascii_string(r#"" \x80 ""#), None);
}

#[test]
fn test_parse_utf8_string() {
    assert_eq!(yass_aux::parse_utf8_string(r#""""#).unwrap(), "");
    assert_eq!(yass_aux::parse_utf8_string(r#""123""#).unwrap(), "123");
    assert_eq!(yass_aux::parse_utf8_string(r#"" \\ ""#).unwrap(), " \\ ");
    assert_eq!(yass_aux::parse_utf8_string(r#"" \" ""#).unwrap(), " \" ");
    assert_eq!(yass_aux::parse_utf8_string(r#"" \r\n\t ""#).unwrap(), " \r\n\t ");
    assert_eq!(yass_aux::parse_utf8_string(r#"" \x00\x7F ""#).unwrap(), " \x00\x7F ");
    assert_eq!(yass_aux::parse_utf8_string(r#"" \x1B\x1d ""#).unwrap(), " \x1B\x1D ");
    assert_eq!(yass_aux::parse_utf8_string("\" \u{FF} \"").unwrap(), " \u{FF} ");
    assert_eq!(yass_aux::parse_utf8_string(r#"" \u{FF} ""#).unwrap(), " \u{FF} ");
    assert_eq!(yass_aux::parse_utf8_string(r#"" \u{ff} ""#).unwrap(), " \u{FF} ");
    
    assert_eq!(yass_aux::parse_utf8_string(r#"""#), None);
    assert_eq!(yass_aux::parse_utf8_string(r#""" "#), None);
    assert_eq!(yass_aux::parse_utf8_string(r#""\""#), None);
    assert_eq!(yass_aux::parse_utf8_string(r#"" \M ""#), None);
    assert_eq!(yass_aux::parse_utf8_string(r#"" \xT0 ""#), None);
    assert_eq!(yass_aux::parse_utf8_string(r#"" \x0T ""#), None);
    assert_eq!(yass_aux::parse_utf8_string(r#"" \x80 ""#), None);
}

#[test]
fn test_serialize_bool() {
    assert_eq!(yass_aux::serialize_bool(true), "true");
    assert_eq!(yass_aux::serialize_bool(false), "false");
}

#[test]
fn test_serialize_i32() {
    assert_eq!(yass_aux::serialize_i32_as_string(0), "0");
    assert_eq!(yass_aux::serialize_i32_as_string(1), "1");
    assert_eq!(yass_aux::serialize_i32_as_string(-1), "-1");
    assert_eq!(yass_aux::serialize_i32_as_string(2147483647), "2147483647");
    assert_eq!(yass_aux::serialize_i32_as_string(-2147483648), "-2147483648");
}

#[test]
fn test_serialize_i64() {
    assert_eq!(yass_aux::serialize_i64_as_string(0), "0");
    assert_eq!(yass_aux::serialize_i64_as_string(1), "1");
    assert_eq!(yass_aux::serialize_i64_as_string(-1), "-1");
    assert_eq!(yass_aux::serialize_i64_as_string(9223372036854775807), "9223372036854775807");
    assert_eq!(yass_aux::serialize_i64_as_string(-9223372036854775808), "-9223372036854775808");
}

#[test]
fn test_serialize_u32() {
    assert_eq!(yass_aux::serialize_u32_as_string(0), "0");
    assert_eq!(yass_aux::serialize_u32_as_string(1), "1");
    assert_eq!(yass_aux::serialize_u32_as_string(4294967295), "4294967295");
}

#[test]
fn test_serialize_u64() {
    assert_eq!(yass_aux::serialize_u64_as_string(0), "0");
    assert_eq!(yass_aux::serialize_u64_as_string(1), "1");
    assert_eq!(yass_aux::serialize_u64_as_string(18446744073709551615), "18446744073709551615");
}

#[test]
fn test_serialize_f64() {
    assert_eq!(yass_aux::serialize_f64_as_string(0.0), "0.0");
    assert_eq!(yass_aux::serialize_f64_as_string(1.0), "1.0");
    assert_eq!(yass_aux::serialize_f64_as_string(-1.0), "-1.0");
    assert_eq!(yass_aux::serialize_f64_as_string(10.0), "10.0");
    assert_eq!(yass_aux::serialize_f64_as_string(100.0), "100.0");
    assert_eq!(yass_aux::serialize_f64_as_string(1.0e10), "1.0e10");
    assert_eq!(yass_aux::serialize_f64_as_string(0.1), "0.1");
    assert_eq!(yass_aux::serialize_f64_as_string(0.01), "0.01");
    assert_eq!(yass_aux::serialize_f64_as_string(0.001), "0.001");
    assert_eq!(yass_aux::serialize_f64_as_string(0.0001), "0.0001");
    assert_eq!(yass_aux::serialize_f64_as_string(0.00001), "0.00001");
    assert_eq!(yass_aux::serialize_f64_as_string(10.1), "10.1");
    assert_eq!(yass_aux::serialize_f64_as_string(10.101), "10.101");
    assert_eq!(yass_aux::serialize_f64_as_string(10.0000001), "10.0000001");
    assert_eq!(yass_aux::serialize_f64_as_string(1234.5678), "1234.5678");
    assert_eq!(yass_aux::serialize_f64_as_string(4.591856e50), "4.591856e50");
    assert_eq!(yass_aux::serialize_f64_as_string(4.591856e-50), "4.591856e-50");
    assert_eq!(yass_aux::serialize_f64_as_string(4.80144468355317e-300), "4.80144468355317e-300");
    assert_eq!(yass_aux::serialize_f64_as_string(4.80144468355317e+300), "4.80144468355317e300");
    assert_eq!(yass_aux::serialize_f64_as_string(1.2e-315), "1.2e-315");
    assert_eq!(yass_aux::serialize_f64_as_string(1.7976931348623157e+308), "1.7976931348623157e308");
    assert_eq!(yass_aux::serialize_f64_as_string(2.2250738585072014e-308), "2.2250738585072014e-308");
    assert_eq!(yass_aux::serialize_f64_as_string(5.0e-324), "5.0e-324");
}

#[test]
fn test_serialize_byte_string() {
    assert_eq!(yass_aux::serialize_byte_string_as_string(b""), r#""""#);
    assert_eq!(yass_aux::serialize_byte_string_as_string(b"\x20\x7E"), r#"" ~""#);
    assert_eq!(yass_aux::serialize_byte_string_as_string(b" \t "), r#"" \t ""#);
    assert_eq!(yass_aux::serialize_byte_string_as_string(b" \n "), r#"" \n ""#);
    assert_eq!(yass_aux::serialize_byte_string_as_string(b" \" "), r#"" \" ""#);
    assert_eq!(yass_aux::serialize_byte_string_as_string(b" \\ "), r#"" \\ ""#);
    assert_eq!(yass_aux::serialize_byte_string_as_string(b" \x00 "), r#"" \x00 ""#);
    assert_eq!(yass_aux::serialize_byte_string_as_string(b" \xFF "), r#"" \xff ""#);
}


#[test]
fn test_serialize_ascii_string() {
    assert_eq!(yass_aux::serialize_ascii_string_as_string(""), r#""""#);
    assert_eq!(yass_aux::serialize_ascii_string_as_string("\x20\x7E"), r#"" ~""#);
    assert_eq!(yass_aux::serialize_ascii_string_as_string(" \t "), r#"" \t ""#);
    assert_eq!(yass_aux::serialize_ascii_string_as_string(" \n "), r#"" \n ""#);
    assert_eq!(yass_aux::serialize_ascii_string_as_string(" \" "), r#"" \" ""#);
    assert_eq!(yass_aux::serialize_ascii_string_as_string(" \\ "), r#"" \\ ""#);
    assert_eq!(yass_aux::serialize_ascii_string_as_string(" \x00 "), r#"" \x00 ""#);
    assert_eq!(yass_aux::serialize_ascii_string_as_string(" \x7F "), r#"" \x7f ""#);
}

#[test]
fn test_serialize_utf8_string() {
    assert_eq!(yass_aux::serialize_utf8_string_as_string(""), r#""""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string("\x20\x7E"), r#"" ~""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string(" \t "), r#"" \t ""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string(" \n "), r#"" \n ""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string(" \" "), r#"" \" ""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string(" \\ "), r#"" \\ ""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string(" \x00 "), r#"" \u{0} ""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string(" \x7F "), r#"" \u{7f} ""#);
    assert_eq!(yass_aux::serialize_utf8_string_as_string(" \u{FFFD} "), r#"" \u{fffd} ""#);
}
