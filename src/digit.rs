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
    pub fn new<T: Into<u8>>(digit: T) -> Option<Digit> {
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

//impl from u8
impl From<u8> for Digit {
    fn from(digit: u8) -> Self {
        Digit::new(digit).unwrap()
    }
}

//impl tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Digit::new(0u8), Some(Digit::Zero));
        assert_eq!(Digit::new(1u8), Some(Digit::One));
        assert_eq!(Digit::new(2u8), Some(Digit::Two));
        assert_eq!(Digit::new(3u8), Some(Digit::Three));
        assert_eq!(Digit::new(4u8), Some(Digit::Four));
        assert_eq!(Digit::new(5u8), Some(Digit::Five));
        assert_eq!(Digit::new(6u8), Some(Digit::Six));
        assert_eq!(Digit::new(7u8), Some(Digit::Seven));
        assert_eq!(Digit::new(8u8), Some(Digit::Eight));
        assert_eq!(Digit::new(9u8), Some(Digit::Nine));
        assert_eq!(Digit::new(10u8), None);
    }

    #[test]
    fn test_from() {
        assert_eq!(Digit::from(0u8), Digit::Zero);
        assert_eq!(Digit::from(1u8), Digit::One);
        assert_eq!(Digit::from(2u8), Digit::Two);
        assert_eq!(Digit::from(3u8), Digit::Three);
        assert_eq!(Digit::from(4u8), Digit::Four);
        assert_eq!(Digit::from(5u8), Digit::Five);
        assert_eq!(Digit::from(6u8), Digit::Six);
        assert_eq!(Digit::from(7u8), Digit::Seven);
        assert_eq!(Digit::from(8u8), Digit::Eight);
        assert_eq!(Digit::from(9u8), Digit::Nine);
    }

    #[test]
    fn test_into_u32() {
        assert_eq!(Digit::Zero as u8, 0);
        assert_eq!(Digit::One as u8, 1);
        assert_eq!(Digit::Two as u8, 2);
        assert_eq!(Digit::Three as u8, 3);
        assert_eq!(Digit::Four as u8, 4);
        assert_eq!(Digit::Five as u8, 5);
        assert_eq!(Digit::Six as u8, 6);
        assert_eq!(Digit::Seven as u8, 7);
        assert_eq!(Digit::Eight as u8, 8);
        assert_eq!(Digit::Nine as u8, 9);
    }
}
