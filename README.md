# str_to_bytes

A library to parse a string representations of numbers or ascii text and return a ```Vec<u8>```.

This library was originally created to be used in [BlueRepl-rs](https://github.com/Yohannfra/BlueRepl-rs)

## Examples

```
"0xff 0xfa 0b11" => [0xff, 0xfa, 0b11];
"0b01 ASCII(abc) 12 44 0x35" => [0b01, 97, 98, 99, 12, 44, 0x35];
...
```

See more examples in the example directory.

## Formats

- **0b**... or **0B**... -> Binary number
- **0x**... or **0X**... -> Hexadecimal number
- **123** -> decimal number
- **ASCII(**...**)** -> Ascii text
