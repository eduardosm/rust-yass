# YASS

YASS is a data and configuration text format. The implementation is written in [Rust][rust].

[rust]: https://www.rust-lang.org/

Example
-------

```text
\ Based on 2nd example at https://json.org/example.html (as of 2018-03-31)
(example-1-widget)
debug true
window {
  title "Sample Konfabulator Widget"
  name "main_window"
  width 500
  height 500
}
item (image){
  src "Images/Sun.png"
  name "sun1"
  hoffset 250
  voffset 250
  alignment center
}
item (text){
  data "Click Here"
  size 36
  style bold
  name "text1"
  hoffset 250
  voffset 100
  alignment center
  on-mouse-up "sun1.opacity = (sun1.opacity / 100) * 90;"
}
```

Crates
------

This project consists of the following Rust crates:
 * yass: Main data structures
 * yass-parser: Parser
 * yass-serializer: Serializer
 * yass-codegen: Code generator for schemas
 * yass-schema-error: Errors that can be returned by code generated by yass-codegen
 * yass-aux: Auxiliary functions used by by code generated by yass-codegen

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.