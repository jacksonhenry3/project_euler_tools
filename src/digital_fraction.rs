use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

use crate::digit_list;
use crate::primes;

//derive
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DigitalFraction {
    pub numerator: digit_list::DigitList,
    pub denominator: digit_list::DigitList,
}

impl DigitalFraction {
    pub fn new<T: Into<u128>>(numerator: T, denominator: T) -> DigitalFraction {
        DigitalFraction {
            numerator: digit_list::DigitList::new(numerator),
            denominator: digit_list::DigitList::new(denominator),
        }
    }
}

impl std::ops::Mul for DigitalFraction {
    type Output = DigitalFraction;

    fn mul(self, rhs: Self) -> Self::Output {
        DigitalFraction {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl std::ops::MulAssign for DigitalFraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl std::ops::Div for DigitalFraction {
    type Output = DigitalFraction;

    fn div(self, rhs: Self) -> Self::Output {
        DigitalFraction {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
    }
}

impl std::ops::DivAssign for DigitalFraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

//addition
impl std::ops::Add for DigitalFraction {
    type Output = DigitalFraction;

    fn add(self, rhs: Self) -> Self::Output {
        DigitalFraction {
            numerator: self.numerator * rhs.denominator.clone()
                + self.denominator.clone() * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl std::ops::AddAssign for DigitalFraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

//impl reduce
impl DigitalFraction {
    pub fn reduce(&self) -> DigitalFraction {
        //reduce self by converting to u128 and back
        let numerator = self.numerator.to_number::<u128>();
        let denominator = self.denominator.to_number::<u128>();
        let gcd = primes::gcd(numerator, denominator);
        DigitalFraction::new(numerator / gcd, denominator / gcd)
    }
}

//to_number
impl DigitalFraction {
    pub fn to_number<T: From<u8> + Add<Output = T> + Mul<Output = T> + Div<Output = T>>(
        &self,
    ) -> T {
        let numerator = self.numerator.to_number::<T>();
        let denominator = self.denominator.to_number::<T>();
        (numerator / denominator) as T
    }

    pub fn reciprocal(&self) -> DigitalFraction {
        DigitalFraction {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }
}

//impl tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = DigitalFraction::new(1u128, 2u128);
        let b = DigitalFraction::new(2u128, 3u128);
        let c = DigitalFraction::new(2u128, 6u128);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_div() {
        let a = DigitalFraction::new(1u128, 2u128);
        let b = DigitalFraction::new(2u128, 3u128);
        let c = DigitalFraction::new(3u128, 4u128);
        assert_eq!(a / b, c);
    }

    #[test]
    fn test_add() {
        let a = DigitalFraction::new(1u128, 2u128);
        let b = DigitalFraction::new(2u128, 3u128);
        let c = DigitalFraction::new(7u128, 6u128);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_reduce() {
        let a = DigitalFraction::new(12u128, 4u128);
        let b = DigitalFraction::new(3u128, 1u128);

        assert_eq!(a.reduce(), b);
    }
}
