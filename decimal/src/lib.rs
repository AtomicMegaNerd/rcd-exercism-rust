use core::str::FromStr;
use num_bigint::BigInt;
use std::ops;
use std::string::ToString;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Decimal {
    // implement your type here
    int_part: BigInt,
    frac_part: BigInt,
}

impl ToString for Decimal {
    fn to_string(&self) -> String {
        format!("{}.{}", self.int_part, self.frac_part)
    }
}

impl Decimal {
    pub fn new(int_part: BigInt, frac_part: BigInt) -> Decimal {
        Decimal {
            int_part,
            frac_part,
        }
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut parts = input.split('.');

        let int_part = BigInt::from_str(parts.next()?);
        let frac_part = BigInt::from_str(parts.next()?);

        match (int_part, frac_part) {
            (Err(_), Err(_)) => None,
            (Ok(_), Err(_)) => None,
            (Err(_), Ok(_)) => None,
            (Ok(x), Ok(y)) => Some(Decimal::new(x, y)),
        }
    }
}

impl ops::Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Decimal::new(
            self.int_part + other.int_part,
            self.frac_part + other.frac_part,
        )
    }
}

impl ops::Sub for Decimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Decimal::new(
            self.int_part - other.int_part,
            self.frac_part - other.frac_part,
        )
    }
}

impl ops::Mul for Decimal {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Decimal::new(
            self.int_part * other.int_part,
            self.frac_part * other.frac_part,
        )
    }
}
