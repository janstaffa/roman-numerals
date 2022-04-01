# roman-numerals
Roman numeral parser, validator and convertor written in Rust.

# Usage:

```rs
use roman_numerals::RomanNumber;

// convert roman to arabic
let roman: RomanNumber = RomanNumber::from_string("LXIX").unwrap();
assert_eq!(roman.to_arab().unwrap(), 69);

// convert arabic to roman
let roman: RomanNumber = RomanNumber::from_arab(69).unwrap();
assert_eq!(roman.to_string(), "LXIX");
```
