use std::fmt;
const ROMAN_LITERALS: [char; 7] = ['i', 'v', 'x', 'l', 'c', 'd', 'm'];
const ARAB_MAP: [i32; 7] = [1, 5, 10, 50, 100, 500, 1000];

pub struct RomanNumber {
    symbols: Vec<char>,
}

impl RomanNumber {
    /// Converts a string like "XLIX" to a RomanNumber instance
    pub fn from_string(s: &str) -> Result<Self, String> {
        let s = s.to_lowercase();
        let mut prev_n = 0;
        let mut n_count = 0;

        let chars: Vec<char> = s.chars().collect();
        // Check if the string is a valid roman number
        for ch in &chars {
            let print_ch = ch.to_uppercase();
            if !ROMAN_LITERALS.contains(&ch) {
                return Err(format!(
                    "Invalid input. '{}' is not a roman number.",
                    print_ch
                ));
            }
            let n = Self::get_arab(*ch)?;

            if n == prev_n {
                if let 5 | 50 | 500 = n {
                    return Err(format!("Invalid input. '{}' cannot be repeated.", print_ch));
                }
                if n_count >= 3 {
                    return Err(format!(
                        "Invalid input. '{}' was repeated more than 3 times.",
                        print_ch
                    ));
                }
                n_count += 1;
            } else {
                if n > prev_n && prev_n > 0 {
                    let prev_idx = ARAB_MAP.iter().position(|&i| i == prev_n).unwrap();
                    let current_idx = ARAB_MAP.iter().position(|&i| i == n).unwrap();

                    if prev_idx + 2 < current_idx {
                        return Err(format!(
                            "Invalid input. Cannot substract {} from {}.",
                            Self::get_roman(prev_n)?.to_uppercase(),
                            print_ch
                        ));
                    }

                    if n_count > 1 {
                        return Err(
                            "Invalid input. Cannot substract from a number more than once.".into(),
                        );
                    }
                    if let 5 | 50 | 500 = prev_n {
                        return Err(format!(
                            "Invalid input. Cannot use '{}' for substracting.",
                            Self::get_roman(prev_n)?
                        ));
                    }
                }
                prev_n = n;
                n_count = 1;
            }
        }
        Ok(RomanNumber { symbols: chars })
    }

    /// Returns arab equivalent of the roman number
    fn get_arab(roman: char) -> Result<i32, String> {
        let index = ROMAN_LITERALS.iter().position(|&r| r == roman);
        if let Some(index) = index {
            Ok(ARAB_MAP[index])
        } else {
            Err(format!("'{}' is not a valid roman number.", roman))
        }
    }

    /// Returns roman equivalent of the arab number
    fn get_roman(arab: i32) -> Result<char, String> {
        let index = ARAB_MAP.iter().position(|&a| a == arab);
        if let Some(index) = index {
            Ok(ROMAN_LITERALS[index])
        } else {
            Err(format!("'{}' does not have a roman equivalent.", arab))
        }
    }

    /// Converts roman numbers to arab numbers
    pub fn to_arab(&self) -> Result<u32, String> {
        let chars = &self.symbols;

        let mut result = 0i32;

        for i in 0..chars.len() {
            let n = Self::get_arab(chars[i])?;

            if i < chars.len() - 1 {
                let next_n = Self::get_arab(chars[i + 1])?;

                // Next number is greater, so substract from the result.
                if next_n > n {
                    result -= n;
                    continue;
                }
            }

            result += n;
        }
        Ok(result as u32)
    }
    /// Converts arab numbers to roman numbers
    pub fn from_arab(arab_number: u32) -> Result<Self, String> {
        let mut remainder = arab_number as i32;
        // Get highest exponent of 10, that fits in the number.
        let mut e = (arab_number as f32).log(10.0).floor() as u32;
        let mut result: Vec<char> = Vec::new();

        // loop until exponent is 0
        // the exponent represents one digit in the input number
        // i.e. 1234 -> 1 * 10**3, 2 * 10**2, 3 * 10**1, 4 * 10**0
        loop {
            let divisor: i32 = 10_i32.pow(e);
            // get the ammount of times the divisor will fit into the number
            let mut count: i32 = remainder / divisor;
            remainder %= divisor;

            if count > 0 {
                let mut nums: Vec<u32> = Vec::new();

                if count == 4 || count == 9 {
                    nums.push(1);
                    nums.push((count + 1) as u32);
                } else {
                    if count >= 5 {
                        nums.push(5);
                        count -= 5;
                    }
                    if count < 4 && count > 0 {
                        for _ in 0..count {
                            nums.push(1);
                        }
                    }
                }
                for n in &nums {
                    // find the roman number and add it to the result
                    let idx = ARAB_MAP
                        .iter()
                        .position(|&r| (r as u32) == n * divisor as u32);

                    if let Some(idx) = idx {
                        result.push(ROMAN_LITERALS[idx]);
                    } else {
                        return Err("Conversion failed.".into());
                    }
                }
            }
            // default case
            if e == 0 {
                break;
            }
            e -= 1;
        }
        Ok(Self { symbols: result })
    }
}
impl fmt::Display for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.symbols.iter().collect::<String>().to_uppercase();
        write!(f, "{}", s)
    }
}
