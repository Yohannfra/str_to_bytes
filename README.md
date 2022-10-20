# str_to_bytes

A library to parse a string representations of numbers or ascii text and return a ```Vec<u8>```.

Supports **decimal**, **binary**, **hexadecimal** and **ascii text**.

## Examples

```rust
assert!(str_to_bytes("0xff 0xfa 0b11").unwrap() == [0xff, 0xfa, 0b11]);
```
