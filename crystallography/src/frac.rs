use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

/// A type for rational numbers
/// uses i32 internaly to represent the numerator and the denominator
/// Is allways reduced
#[derive(Debug, Clone, Copy, Hash)]
pub struct Frac(i32, i32);

impl Frac {
    /// creates a fraction and reduces it
    pub fn new(numerator: i32, denomiator: i32) -> Self {
        let mut out = Self(numerator, denomiator);
        out.reduce();
        out
    }

    /// reduce the fraction
    fn reduce(&mut self) {
        assert_ne!(self.1, 0);
        let gcd = gcd(self.0, self.1);
        self.0 /= gcd;
        self.1 /= gcd;
        if self.1 < 0 {
            self.0 *= -1;
            self.1 *= -1
        }
    }

    /// returns the absolute value of the Frac
    pub fn abs(&self) -> Self {
        if self.0 < 0 {
            let mut res = *self;
            res.0 *= -1;
            res
        } else {
            *self
        }
    }

    /// returns the floor of the Fraction
    pub fn floor(self) -> Self {
        let rem = self.0.rem_euclid(self.1);
        let int = if rem == 0 {
            self.0 / self.1
        } else {
            (self.0 - rem) / self.1
        };
        Self(int, 1)
    }

    /// returns the ceil of the Fraction
    pub fn ceil(self) -> Self {
        let rem = self.0.rem_euclid(self.1);
        let int = if rem == 0 {
            self.0 / self.1
        } else {
            (self.0 - rem) / self.1 + 1
        };
        Self(int, 1)
    }

    /// returns the numerator
    pub fn get_numerator(&self) -> i32 {
        self.0
    }

    /// returns the denominator
    pub fn get_denominator(&self) -> i32 {
        self.1
    }
}

/// Different string representations of Frac.
impl Frac {
    /// returns the fraction as a string allways displaying the sign
    pub fn as_string_signed(self) -> String {
        if self.0 > 0 {
            format!("+{}", self)
        } else {
            format!("{}", self)
        }
    }

    /// returns the fraction as a prefactor almost like display but if -1 only sign is returned
    /// empty string if 1
    pub fn as_prefactor(self) -> String {
        if self == Self(-1, 1) {
            return "-".to_string();
        }
        if self == 1.into() {
            return String::new();
        }
        format!("{}", self)
    }

    /// returns the fraction as a signed prefactor like as_prefactor but for +1 the sign is also
    /// returned
    pub fn as_signed_prefactor(self) -> String {
        if self == Self(-1, 1) {
            return "-".to_string();
        }
        if self == 1.into() {
            return "+".to_string();
        }
        format!("{}", self)
    }
}

impl FromStr for Frac {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("/");
        let numerator = iter.next().unwrap();
        if let Some(denominator) = iter.next() {
            Ok(Self::new(numerator.parse()?, denominator.parse()?))
        } else {
            Ok(Self(numerator.parse()?, 1))
        }
    }
}

impl Default for Frac {
    fn default() -> Self {
        Self(0, 1)
    }
}

impl Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 == 1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

impl Add for Frac {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl Add<&Frac> for Frac {
    type Output = Frac;

    fn add(self, rhs: &Frac) -> Self::Output {
        self + *rhs
    }
}

impl Add<Frac> for &Frac {
    type Output = Frac;

    fn add(self, rhs: Frac) -> Self::Output {
        *self + rhs
    }
}

impl Add<&Frac> for &Frac {
    type Output = Frac;

    fn add(self, rhs: &Frac) -> Self::Output {
        *self + *rhs
    }
}

impl AddAssign for Frac {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<&Frac> for Frac {
    fn add_assign(&mut self, rhs: &Frac) {
        *self += *rhs
    }
}

impl Sub for Frac {
    type Output = Frac;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl Sub<&Frac> for Frac {
    type Output = Frac;

    fn sub(self, rhs: &Frac) -> Self::Output {
        self - *rhs
    }
}

impl Sub<Frac> for &Frac {
    type Output = Frac;

    fn sub(self, rhs: Frac) -> Self::Output {
        *self - rhs
    }
}

impl Sub<&Frac> for &Frac {
    type Output = Frac;

    fn sub(self, rhs: &Frac) -> Self::Output {
        *self - *rhs
    }
}

impl SubAssign for Frac {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl SubAssign<&Frac> for Frac {
    fn sub_assign(&mut self, rhs: &Frac) {
        *self -= *rhs
    }
}

impl Neg for Frac {
    type Output = Frac;

    fn neg(mut self) -> Self::Output {
        self.0 *= -1;
        self
    }
}

impl Neg for &Frac {
    type Output = Frac;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl Mul for Frac {
    type Output = Frac;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul<&Frac> for Frac {
    type Output = Frac;

    fn mul(self, rhs: &Frac) -> Self::Output {
        self * *rhs
    }
}

impl Mul<Frac> for &Frac {
    type Output = Frac;

    fn mul(self, rhs: Frac) -> Self::Output {
        *self * rhs
    }
}

impl Mul<&Frac> for &Frac {
    type Output = Frac;

    fn mul(self, rhs: &Frac) -> Self::Output {
        *self * *rhs
    }
}

impl MulAssign for Frac {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl MulAssign<&Frac> for Frac {
    fn mul_assign(&mut self, rhs: &Frac) {
        *self *= *rhs
    }
}

impl Div for Frac {
    type Output = Frac;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.1, self.1 * rhs.0)
    }
}

impl Div<&Frac> for Frac {
    type Output = Frac;

    fn div(self, rhs: &Frac) -> Self::Output {
        self / *rhs
    }
}

impl Div<Frac> for &Frac {
    type Output = Frac;

    fn div(self, rhs: Frac) -> Self::Output {
        *self / rhs
    }
}

impl Div<&Frac> for &Frac {
    type Output = Frac;

    fn div(self, rhs: &Frac) -> Self::Output {
        *self / *rhs
    }
}

impl DivAssign for Frac {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl DivAssign<&Frac> for Frac {
    fn div_assign(&mut self, rhs: &Frac) {
        *self /= *rhs
    }
}

impl Rem for Frac {
    type Output = Frac;

    fn rem(self, rhs: Self) -> Self::Output {
        self + (-self / rhs).ceil() * rhs
    }
}

impl Rem<&Frac> for Frac {
    type Output = Frac;

    fn rem(self, rhs: &Frac) -> Self::Output {
        self % *rhs
    }
}

impl Rem<Frac> for &Frac {
    type Output = Frac;

    fn rem(self, rhs: Frac) -> Self::Output {
        *self % rhs
    }
}

impl Rem<&Frac> for &Frac {
    type Output = Frac;

    fn rem(self, rhs: &Frac) -> Self::Output {
        *self % *rhs
    }
}

impl RemAssign for Frac {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs
    }
}

impl RemAssign<&Frac> for Frac {
    fn rem_assign(&mut self, rhs: &Frac) {
        *self %= *rhs
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Frac {}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.0 * other.1).cmp(&(other.0 * self.1)))
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 * other.1).cmp(&(other.0 * self.1))
    }
}

macro_rules! from_integer {
    ($int:ty) => {
        impl From<$int> for Frac {
            fn from(val: $int) -> Frac {
                Self(val as i32, 1)
            }
        }
    };
}

from_integer!(u8);
from_integer!(i8);
from_integer!(u16);
from_integer!(i16);
from_integer!(i32);

impl Into<f32> for Frac {
    fn into(self) -> f32 {
        self.0 as f32 / self.1 as f32
    }
}

impl Into<f64> for Frac {
    fn into(self) -> f64 {
        self.0 as f64 / self.1 as f64
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a: u32 = a.unsigned_abs();
    let mut b: u32 = b.unsigned_abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(6, 2), 2);
        assert_eq!(gcd(12, 21), 3);
        assert_eq!(gcd(4, -2), 2);
        assert_eq!(gcd(-4, 2), 2);
        assert_eq!(gcd(4, 4), 4);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(5, 3), 1);
        assert_eq!(gcd(0, 3), 3);
        assert_eq!(gcd(5, 0), 5);
    }
    #[test]
    fn normalize_test() {
        assert_eq!(
            Frac::from_str("1/1").unwrap(),
            Frac::from_str("2/2").unwrap()
        );
        assert_eq!(
            Frac::from_str("12/1").unwrap(),
            Frac::from_str("24/2").unwrap()
        );
        assert_eq!(
            Frac::from_str("1/-2").unwrap(),
            Frac::from_str("-1/2").unwrap()
        );
    }

    macro_rules! test_operations {
        ($name:ident, $op:tt, $(($operand1:expr, $operand2:expr, $expected:expr)),*) => {
            #[test]
            fn $name() {
                $(
                    let frac1 = Frac::from_str($operand1).unwrap();
                    let frac2 = Frac::from_str($operand2).unwrap();
                    let result = frac1 $op frac2;
                    assert_eq!(result, Frac::from_str($expected).unwrap());
                )*
            }
        };
    }

    test_operations!(
        test_addition,
        +,
        ("1/2", "1/3", "5/6"),
        ("3/4", "1/2", "5/4"),
        ("3/4", "-1/2", "1/4")
    );

    test_operations!(
        test_subtraction,
        -,
        ("1/2", "1/3", "1/6"),
        ("3/4", "1/2", "1/4")
    );

    test_operations!(
        test_multiplication,
        *,
        ("2/3", "3/4", "1/2"),
        ("1/2", "1/3", "1/6")
    );

    test_operations!(
        test_division,
        /,
        ("1/2", "2/3", "3/4"),
        ("3/4", "1/2", "3/2")
    );

    #[test]
    fn test_comparison() {
        let frac1 = Frac::new(1, 2);
        let frac2 = Frac::new(1, 3);
        assert!(frac1 > frac2);
        assert!(frac2 < frac1);
    }

    #[test]
    fn test_floor_ceil() {
        let frac = Frac::new(1, 2);
        assert_eq!(frac.floor(), 0.into());
        assert_eq!(frac.ceil(), 1.into());

        let frac = Frac::new(-1, 2);
        assert_eq!(frac.floor(), (-1).into());
        assert_eq!(frac.ceil(), 0.into());

        let frac = Frac::new(-3, 2);
        assert_eq!(frac.floor(), (-2).into());
        assert_eq!(frac.ceil(), (-1).into());
    }

    #[test]
    fn test_rem() {
        let result: Frac = Frac::from_str("1/1").unwrap() % Frac::from_str("1/2").unwrap();
        println!("{result}");
        assert_eq!(result, 0.into());
        let result: Frac = Frac::from_str("5/8").unwrap() % Frac::from_str("1/4").unwrap();
        assert_eq!(result, Frac::from_str("1/8").unwrap());
        let result: Frac = Frac::from_str("-5/8").unwrap() % Frac::from_str("1/4").unwrap();
        assert_eq!(result, Frac::from_str("1/8").unwrap());
    }
}
