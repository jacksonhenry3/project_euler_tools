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

//efficient primality check
pub fn is_prime<T: Into<u128>>(number: T) -> bool {
    let number = number.into();
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }
    let mut divisor: u128 = 3;
    while divisor < (number as f64).sqrt() as u128 + 1 {
        if number % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

pub fn gcd<T: Into<u128>>(a: T, b: T) -> u128 {
    let a = a.into();
    let b = b.into();
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
