use roman_numerals;

fn main() {
    // let test = roman_numerals::arab_to_roman(10).unwrap();
    // println!("result: {:?}", test);
    roman_numerals::arab_to_roman(18).unwrap();
    roman_numerals::arab_to_roman(3).unwrap();
    roman_numerals::arab_to_roman(30).unwrap();
    roman_numerals::arab_to_roman(10).unwrap();
    roman_numerals::arab_to_roman(32).unwrap();
    roman_numerals::arab_to_roman(1000).unwrap();
    roman_numerals::arab_to_roman(499).unwrap();
    roman_numerals::arab_to_roman(49).unwrap();
    roman_numerals::arab_to_roman(106).unwrap();
    roman_numerals::arab_to_roman(99).unwrap();
    roman_numerals::arab_to_roman(13).unwrap();
    roman_numerals::arab_to_roman(19).unwrap();
    roman_numerals::arab_to_roman(6).unwrap();
    roman_numerals::arab_to_roman(9).unwrap();
}
// "XVI"
// "I"
// "X"
// "X"
// "XI"
// "M"
// "CLXVI"
// "XVI"
// "CV"
 


// 49 => XLIX
//
// I => 49 % 1 = 0 x49
// V => 49 % 5 = 4 x9
// X => 49 % 10 = 9 x4
// L => 49 % 50 = 49
// XL => 49 % 40 = 9
//
// 9 => IX
// I => 9 % 1 = 0 x9
// V => 9 % 5 = 4 x1
// X => 9 % 10 = 9
//
