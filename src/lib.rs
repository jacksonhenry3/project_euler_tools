use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use std::ops::{Index, IndexMut};

pub fn primes_less_than(largest_number: u128) -> Vec<u128> {
    // Returned vec will contain all primes less than the given maximum

    //start with all numbers
    let mut all_numbers: Vec<u128> = (2..largest_number).collect();

    //we will begin by removing all numbers that are divisible by 2
    let mut divisor: u128 = 2;

    //we will loop through all divisors untill we get to the square root of the max.
    //checking any divisors beyond this would be redundant.
    while divisor < (largest_number as f64).sqrt() as u128 + 1 {
        //keep all numbers that are not divisble by divisor (except divisor itself)
        all_numbers.retain(|&x| (x % divisor != 0 || x == divisor));

        //prepare to check the next divisor
        let current_divisor_index = all_numbers.iter().position(|x| *x == divisor).unwrap();
        divisor = all_numbers[current_divisor_index + 1];
    }
    all_numbers
}

//create an enum to represent the different base 10 digits, and a function to convert any integer to a digit

//add derive to and from u8 and u128 to allow conversion to and from integers
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

//impl display and debug
impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl Digit {
    pub fn new<T: Into<u128>>(digit: T) -> Option<Digit> {
        match digit.into() {
            0 => Some(Digit::Zero),
            1 => Some(Digit::One),
            2 => Some(Digit::Two),
            3 => Some(Digit::Three),
            4 => Some(Digit::Four),
            5 => Some(Digit::Five),
            6 => Some(Digit::Six),
            7 => Some(Digit::Seven),
            8 => Some(Digit::Eight),
            9 => Some(Digit::Nine),
            _ => None,
        }
    }
}

//construct a u8 from a digit
impl From<Digit> for u8 {
    fn from(digit: Digit) -> Self {
        digit as u8
    }
}

//construct a u16 from a digit
impl From<Digit> for u16 {
    fn from(digit: Digit) -> Self {
        digit as u16
    }
}

//construct a u32 from a digit
impl From<Digit> for u32 {
    fn from(digit: Digit) -> Self {
        digit as u32
    }
}

//construct a u64 from a digit
impl From<Digit> for u64 {
    fn from(digit: Digit) -> Self {
        digit as u64
    }
}

//construct a u128 from a digit
impl From<Digit> for u128 {
    fn from(digit: Digit) -> Self {
        digit as u128
    }
}

// ======================
// ===== DigitList ======
// ======================

//derive
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct DigitList(Vec<Digit>);

impl DigitList {
    pub fn new<T: Into<u128>>(number: T) -> DigitList {
        let mut result: Vec<Digit> = Vec::new();
        let mut n = number.into();
        while n != 0 {
            result.push(Digit::new(n % 10).unwrap());
            n /= 10;
        }
        result.reverse();
        DigitList(result)
    }

    pub fn to_number<T: From<u8> + Mul<Output = T> + Add<Output = T>>(&self) -> T {
        let mut result = T::from(0u8);
        for digit in self.0.iter() {
            result = result * T::from(10) + T::from(*digit as u8);
        }
        result
    }

    pub fn digital_sum<T: From<u8> + Add<Output = T>>(&self) -> T {
        let mut result = T::from(0u8);
        for digit in &self.0 {
            result = result + T::from(*digit as u8);
        }
        result
    }

    pub fn reverse(&self) -> DigitList {
        let mut result = self.0.clone();
        result.reverse();
        DigitList(result)
    }

    pub fn is_palindrome(&self) -> bool {
        self.0 == self.reverse().0
    }

    pub fn pow(&self, exponent: u32) -> DigitList {
        let mut result = DigitList::new(1u128);
        for _ in 0..exponent {
            result = result * self.clone();
        }
        result
    }
}

impl Index<usize> for DigitList {
    type Output = Digit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[&self.0.len() - index - 1]
    }
}

impl IndexMut<usize> for DigitList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let l = &self.0.len();
        &mut self.0[l - index - 1]
    }
}

//impl arithmetic operators for DigitList
impl Add for DigitList {
    type Output = DigitList;

    fn add(self, other: DigitList) -> DigitList {
        //make sure that the first digit is the largest
        let mut result = DigitList::new(0u128);
        let mut carry = 0u8;
        let mut i = 0usize;

        while i < self.0.len() || i < other.0.len() {
            let mut sum = carry;
            if i < self.0.len() {
                sum += self[i] as u8;
            }
            if i < other.0.len() {
                sum += other[i] as u8;
            }
            carry = sum / 10;
            result.0.push(Digit::new(sum % 10).unwrap());
            i += 1;
        }

        if carry != 0 {
            result.0.push(Digit::new(carry).unwrap());
        }

        result.reverse()
    }
}

impl AddAssign for DigitList {
    fn add_assign(&mut self, other: DigitList) {
        *self = self.clone() + other;
    }
}

//impliment mul by repeated addition
impl Mul for DigitList {
    type Output = DigitList;

    fn mul(self, other: DigitList) -> DigitList {
        let mut result = DigitList::new(0u128);
        for i in 0..other.0.len() {
            let mut partial = DigitList::new(0u128);
            for _ in 0..other[i] as u8 {
                partial += self.clone();
            }
            for _ in 0..i {
                partial.0.push(Digit::Zero);
            }
            result += partial;
        }
        result
    }
}

impl MulAssign for DigitList {
    fn mul_assign(&mut self, other: DigitList) {
        *self = self.clone() * other;
    }
}

//reverse the digits of a number by converting it to a digit list, reversing the list, and converting back to a number
pub fn reverse_number<T: Into<u128> + From<u8> + Mul<Output = T> + Add<Output = T>>(
    number: T,
) -> T {
    let number = number.into();
    let mut digits = DigitList::new(number);
    digits.0.reverse();
    digits.to_number()
}

//implement some tests of the DigitList struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let number = 1234567890u128;
        let digits = DigitList::new(number);
        assert_eq!(digits.to_number::<u128>(), number);
    }

    #[test]
    fn test_to_number() {
        let number = 1111111110u128;
        let digits = DigitList::new(number);
        assert_eq!(digits.to_number::<u128>(), number);
    }

    #[test]
    fn test_digital_sum() {
        let number = 1234567890u128;
        let digits = DigitList::new(number);
        assert_eq!(digits.digital_sum::<u128>(), 45);
    }

    #[test]
    fn test_pow() {
        let number = 1234567890u128;
        let digits = DigitList::new(number);
        assert_eq!(digits.pow(2).to_number::<u128>(), 1524157875019052100);
    }

    #[test]
    fn test_add() {
        let number1 = 987654321u128;
        let number2 = 123456789u128;
        let digits1 = DigitList::new(number1);
        let digits2 = DigitList::new(number2);
        assert_eq!((digits1 + digits2).to_number::<u128>(), 1111111110u128);
    }

    #[test]
    fn test_add_assign() {
        let number1 = 1234567890u128;
        let number2 = 9876543210u128;
        let mut digits1 = DigitList::new(number1);
        let digits2 = DigitList::new(number2);
        digits1 += digits2;
        assert_eq!(digits1.to_number::<u128>(), 11111111100);
    }

    #[test]
    fn test_mul() {
        let number1 = 1234567890u128;
        let number2 = 9876543210u128;
        let digits1 = DigitList::new(number1);
        let digits2 = DigitList::new(number2);
        assert_eq!(
            (digits1 * digits2).to_number::<u128>(),
            12193263111263526900
        );
    }

    #[test]
    fn test_mul_assign() {
        let number1 = 1234567890u128;
        let number2 = 9876543210u128;
        let mut digits1 = DigitList::new(number1);
        let digits2 = DigitList::new(number2);
        digits1 *= digits2;
        assert_eq!(digits1.to_number::<u128>(), 12193263111263526900);
    }
}
