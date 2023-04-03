#![allow(dead_code)]

use std::ops::{Add, AddAssign, Div, Mul, MulAssign};
use std::ops::{Index, IndexMut};

use crate::digit::Digit;
use crate::digital_fraction::{self, DigitalFraction};
//derive
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct DigitList(pub Vec<Digit>);

impl DigitList {
    pub fn new<T: Into<u128>>(number: T) -> DigitList {
        let mut result: Vec<Digit> = Vec::new();
        let mut n = number.into();
        while n != 0 {
            result.push(Digit::new((n % 10) as u8).unwrap());
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
            result *= self.clone();
        }
        result
    }
}

impl Index<usize> for DigitList {
    type Output = Digit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[self.0.len() - index - 1]
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

//implement mul by repeated addition
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

impl Div for DigitList {
    type Output = DigitalFraction;

    fn div(self, other: DigitList) -> DigitalFraction {
        digital_fraction::DigitalFraction {
            numerator: self,
            denominator: other,
        }
    }
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

    #[test]
    fn test_div() {
        let number1 = 1234567890u128;
        let number2 = 9876543210u128;
        let digits1 = DigitList::new(number1);
        let digits2 = DigitList::new(number2);
        assert_eq!(
            (digits1 / digits2).to_number::<u128>(),
            1234567890 / 9876543210
        );
    }
}
