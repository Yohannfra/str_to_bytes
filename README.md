# str_to_bytes

A library to parse a string representations of numbers or ascii text and return a ```Vec<u8>```.

This library was originally created to be used in [bluerepl](https://github.com/Yohannfra/bluerepl)

## Examples

```
"0xff 0xfa 0b11" => [0xff, 0xfa, 0b11]
"0b01 ASCII(abc) 12 44 0x35" => [0b01, 97, 98, 99, 12, 44, 0x35]
...
```

## Formats

- **0b**... or **0B**... -> Binary number
- **0x**... or **0X**... -> Hexadecimal number
- **123** -> decimal number
- **ASCII(**...**)** -> Ascii text

# License

This library is licensed under the terms of the MIT license.
