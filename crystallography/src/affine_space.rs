//! # Affine Space
//! In this module an affine space is defined.
//!
//! All objects use rational coefficients.
//! A Pos3 is different from a Vec3 in that the translation part of an affine transformation aren't
//! applied to a Vec3.
//!
//! ## on the implementation of Rem (%)
//!
//! The remainder operation is used to model peridic boudary conditions.
//! Let the point (x, y, z) be aequivalent to all points (x + an, y + bm, z + cl) with n, m, l
//! integers.
//! this means that all points can be represented with coefficients
//! 0 <= x < a
//! 0 <= y < b
//! 0 <= z < c
//! This is what the implementation for Pos3 % Vec3 does.
//!
//! For Vec3 however they should be represented as the shortest equivalent translation. As such the
//! coefficients (x, y, z) are brought into the range
//!
//! -a/2 < x <= a/2
//! -b/2 < y <= b/2
//! -c/2 < z <= c/2
//!
//! Furthermore, note that implemntation of Ord on structs do not provide mathematical information
//! about the struct, rather provide a unique way to sort a Vec<T> of the given struct.

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use nalgebra::{Matrix3, Point3, Vector3};
use pest::iterators::Pair;

use crate::{copy_mul_impl, Frac, OpListRule};

/// A vector type using rational indexes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3([Frac; 3]);

impl Vec3 {
    /// constructor
    pub fn new(x: Frac, y: Frac, z: Frac) -> Self {
        Self([x, y, z])
    }

    /// returns a vector with all values set to `val`
    pub fn splat(val: Frac) -> Self {
        Self([val, val, val])
    }

    /// returns the zero vector
    pub fn zero() -> Self {
        Self([0.into(); 3])
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0[0], self.0[1], self.0[2])
    }
}

impl<T: Into<Frac>> From<[T; 3]> for Vec3 {
    fn from(value: [T; 3]) -> Self {
        let mut arr: [Frac; 3] = Default::default();
        for (i, val) in value.into_iter().enumerate() {
            arr[i] = val.into();
        }
        Self(arr)
    }
}

impl Into<Vector3<f32>> for Vec3 {
    fn into(self) -> Vector3<f32> {
        [self.0[0].into(), self.0[1].into(), self.0[2].into()].into()
    }
}

impl Vec3 {
    /// The dot product
    pub fn dot(&self, other: &Self) -> Frac {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.into(), |acc, (a, b)| acc + *a * *b)
    }

    /// returns the square of the norm of the vector
    pub fn norm_sq(&self) -> Frac {
        self.dot(&self)
    }

    /// returns the norm of the vector as an f32
    pub fn norm(&self) -> f32 {
        let len_sq: f32 = self.norm_sq().into();
        len_sq.sqrt()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul<Frac> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: Frac) -> Self::Output {
        self.0.iter_mut().for_each(|val| *val *= rhs);
        self
    }
}

impl Mul<Vec3> for Frac {
    type Output = Vec3;

    fn mul(self, mut rhs: Vec3) -> Self::Output {
        rhs.0.iter_mut().for_each(|val| *val *= self);
        rhs
    }
}

impl Div<Frac> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Frac) -> Self::Output {
        self * rhs.reciprocal()
    }
}

impl Div<&Frac> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Frac) -> Self::Output {
        self / *rhs
    }
}

impl DivAssign<Frac> for Vec3 {
    fn div_assign(&mut self, rhs: Frac) {
        *self = *self / rhs
    }
}

impl DivAssign<&Frac> for Vec3 {
    fn div_assign(&mut self, rhs: &Frac) {
        *self = *self / rhs
    }
}
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Self::Output {
        self.0.iter_mut().for_each(|val| *val = -*val);
        self
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl Rem<Bounds3> for Vec3 {
    type Output = Vec3;

    fn rem(mut self, rhs: Bounds3) -> Self::Output {
        self.0.iter_mut().zip(rhs.0.iter()).for_each(|(a, b)| {
            let b = Frac::new(*b, 1);
            *a %= b;
            if *a / b > Frac::new(1, 2) {
                *a -= b
            }
        });
        self
    }
}

impl Rem<&Bounds3> for Vec3 {
    type Output = Vec3;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        self % *rhs
    }
}

impl RemAssign<Bounds3> for Vec3 {
    fn rem_assign(&mut self, rhs: Bounds3) {
        *self = *self % rhs;
    }
}

impl RemAssign<&Bounds3> for Vec3 {
    fn rem_assign(&mut self, rhs: &Bounds3) {
        *self = *self % *rhs
    }
}

/// A position type using rational values
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos3([Frac; 3]);

impl Pos3 {
    /// constructor
    pub fn new(x: Frac, y: Frac, z: Frac) -> Self {
        Self([x, y, z])
    }

    /// returns a position where all components are set to `val`
    pub fn splat(val: Frac) -> Self {
        Self([val, val, val])
    }

    /// returns the origin (0, 0, 0)
    pub fn origin() -> Self {
        Self([0_u16.into(); 3])
    }
}

impl Display for Pos3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0[0], self.0[1], self.0[2])
    }
}

impl<T: Into<Frac>> From<[T; 3]> for Pos3 {
    fn from(value: [T; 3]) -> Self {
        let mut arr: [Frac; 3] = Default::default();
        for (i, val) in value.into_iter().enumerate() {
            arr[i] = val.into();
        }
        Self(arr)
    }
}

impl From<(Frac, Frac, Frac)> for Pos3 {
    fn from(value: (Frac, Frac, Frac)) -> Self {
        Self([value.0, value.1, value.2])
    }
}

impl From<Vec3> for Pos3 {
    fn from(value: Vec3) -> Self {
        Self(value.0)
    }
}

impl From<Pos3> for Vec3 {
    fn from(value: Pos3) -> Self {
        Self(value.0)
    }
}

impl Into<Point3<f32>> for Pos3 {
    fn into(self) -> Point3<f32> {
        [self.0[0].into(), self.0[1].into(), self.0[2].into()].into()
    }
}

impl Sub for Pos3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut arr = self.0;
        arr.iter_mut().zip(rhs.0.iter()).for_each(|(l, r)| *l -= *r);
        Vec3(arr)
    }
}

impl Add<Pos3> for Vec3 {
    type Output = Pos3;

    fn add(self, mut rhs: Pos3) -> Self::Output {
        rhs.0
            .iter_mut()
            .zip(self.0.iter())
            .for_each(|(a, b)| *a += *b);
        rhs
    }
}

impl Add<Vec3> for Pos3 {
    type Output = Pos3;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a += *b);
        self
    }
}

impl AddAssign<Vec3> for Pos3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs
    }
}

impl Rem<Bounds3> for Pos3 {
    type Output = Pos3;

    fn rem(mut self, rhs: Bounds3) -> Self::Output {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a %= Frac::new(*b, 1));
        self
    }
}

impl Rem<&Bounds3> for Pos3 {
    type Output = Pos3;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        self % *rhs
    }
}

impl RemAssign<Bounds3> for Pos3 {
    fn rem_assign(&mut self, rhs: Bounds3) {
        *self = *self % rhs
    }
}

impl RemAssign<&Bounds3> for Pos3 {
    fn rem_assign(&mut self, rhs: &Bounds3) {
        *self = *self % *rhs
    }
}

// [x x x
//  x x x
//  x x x]
// represented like this
/// A 3x3 Matrix using rational components
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat3([Frac; 9]);

impl Mat3 {
    /// creates a matrix from three column vectors
    pub fn from_columns(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        let mut array: [Frac; 9] = Default::default();
        for i in 0..3 {
            array[3 * i] = v1.0[i];
            array[3 * i + 1] = v2.0[i];
            array[3 * i + 2] = v3.0[i];
        }
        Self(array)
    }

    /// returns the identity matrix
    #[rustfmt::skip]
    pub fn identity() -> Self {
        let i: Frac = 1.into();
        let o: Frac = 0.into();
        Self([i, o, o,
              o, i, o,
              o, o, i])
    }
}

impl Display for Mat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_affine: Affine3 = (*self).into();
        Display::fmt(&as_affine, f)
    }
}

impl Mat3 {
    /// calculates the determinant of the matrix
    #[rustfmt::skip]
    pub fn determinant(&self) -> Frac {
        let [a, b, c,
             d, e, f,
             g, h, i] = self.0;

        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    /// returns the cofactor matrix
    #[rustfmt::skip]
    pub fn cofactor(&self) -> Self {
        let [a, b, c,
             d, e, f,
             g, h, i] = self.0;
        Self([e*i - f*h, f*g - d*i, d*h - e*g,
              h*c - i*b, a*i - c*g, b*g - a*h,
              b*f - c*e, c*d - a*f, a*e - b*d])
    }

    /// returns the adjoint of the matrix
    pub fn adjoint(&self) -> Self {
        self.cofactor().transpose()
    }

    /// returns the inverse of a matrix if it exists
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.into() {
            return None;
        }
        Some((Frac::new(1, 1) / det) * self.adjoint())
    }

    /// retruns true if the matrix is invertible
    pub fn is_invertible(&self) -> bool {
        !(self.determinant() == 0.into())
    }

    /// returns true if the matrix is orthogonal
    pub fn is_orthogonal(&self) -> bool {
        Self::identity() == self * self.transpose()
    }

    /// returns the transpose of the matrix
    #[rustfmt::skip]
    pub fn transpose(&self) -> Self {
        let [a, b, c,
             d, e, f,
             g, h, i] = self.0;
        Self([a, d, g,
              b, e, h,
              c, f, i])
    }
}

impl<T: Into<Frac>> From<[T; 9]> for Mat3 {
    fn from(value: [T; 9]) -> Self {
        let mut arr: [Frac; 9] = Default::default();
        for (i, val) in value.into_iter().enumerate() {
            arr[i] = val.into();
        }
        Self(arr)
    }
}

impl Into<Matrix3<f32>> for Mat3 {
    fn into(self) -> Matrix3<f32> {
        [
            [self.0[0].into(), self.0[3].into(), self.0[6].into()],
            [self.0[1].into(), self.0[4].into(), self.0[7].into()],
            [self.0[2].into(), self.0[5].into(), self.0[8].into()],
        ]
        .into()
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [Frac::new(0, 1); 9];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i * 3 + j] += self.0[i * 3 + k] * rhs.0[k * 3 + j];
                }
            }
        }

        Self(result)
    }
}

copy_mul_impl!(Mat3, Mat3);

impl Mul<Frac> for Mat3 {
    type Output = Mat3;

    fn mul(mut self, rhs: Frac) -> Self::Output {
        self.0.iter_mut().for_each(|val| *val *= rhs);
        self
    }
}

copy_mul_impl!(Mat3, Frac);

impl Mul<Mat3> for Frac {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        rhs * self
    }
}

copy_mul_impl!(Frac, Mat3);

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    #[rustfmt::skip]
    fn mul(self, rhs: Vec3) -> Self::Output {
        let Vec3([x, y, z]) = rhs;
        let [a, b, c,
             d, e, f,
             g, h, i] = self.0;
        Vec3([
            x*a + y*b + z*c,
            x*d + y*e + z*f,
            x*g + y*h + z*i,
        ])
    }
}

copy_mul_impl!(Mat3, Vec3);

impl Mul<Pos3> for Mat3 {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        let as_vec: Vec3 = rhs.into();
        (self * as_vec).into()
    }
}

copy_mul_impl!(Mat3, Pos3);

/// an affine transformation using rational components
/// the affine transformation consists of a matrix multiplication and then the addition of a vector
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Affine3 {
    mat: Mat3,
    translation: Vec3,
}

impl Affine3 {
    /// constructor for an affine transformation
    pub fn new(mat: Mat3, translation: Vec3) -> Self {
        Self { mat, translation }
    }

    /// constructor from a matrix
    pub fn from_mat(mat: Mat3) -> Self {
        Self {
            mat,
            translation: Vec3::zero(),
        }
    }

    /// constructor for translations
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            mat: Mat3::identity(),
            translation,
        }
    }

    /// returns the identity transformation
    pub fn identity() -> Self {
        Self {
            mat: Mat3::identity(),
            translation: Vec3::zero(),
        }
    }

    pub(crate) fn from_parser_trans_op(pair: Pair<OpListRule>) -> Self {
        let mut translation: [Frac; 3] = Default::default();
        for (i, t) in pair.into_inner().enumerate() {
            debug_assert_eq!(t.as_rule(), OpListRule::coeff_translation);
            let mut active_minus = false;
            let mut iter = t.into_inner();
            while let Some(p) = iter.next() {
                if p.as_rule() == OpListRule::sign {
                    if p.as_str() == "-" {
                        active_minus = true;
                    }
                }
                if p.as_rule() == OpListRule::p_rational_num {
                    let mut num = Frac::from_str(p.as_str()).expect("enforced by grammar");
                    if active_minus {
                        num = -num;
                    }
                    translation[i] = num;
                }
            }
        }
        Affine3::from_translation(translation.into())
    }

    pub(crate) fn from_parser_operation(pair: Pair<OpListRule>) -> Self {
        let mut mat: [Frac; 9] = Default::default();
        let mut translation: [Frac; 3] = Default::default();
        for (i, p) in pair.into_inner().enumerate() {
            debug_assert_eq!(p.as_rule(), OpListRule::coeff_op);
            let mut active_minus = false;
            for op in p.into_inner() {
                use OpListRule::*;
                match op.as_rule() {
                    x => {
                        if active_minus {
                            mat[3 * i + 0] = Frac::new(-1, 1)
                        } else {
                            mat[3 * i + 0] = 1.into()
                        }
                        active_minus = false;
                    }
                    y => {
                        if active_minus {
                            mat[3 * i + 1] = Frac::new(-1, 1)
                        } else {
                            mat[3 * i + 1] = 1.into()
                        }
                        active_minus = false;
                    }
                    z => {
                        if active_minus {
                            mat[3 * i + 2] = Frac::new(-1, 1)
                        } else {
                            mat[3 * i + 2] = 1.into()
                        }
                        active_minus = false;
                    }
                    p_rational_num => {
                        let mut num = Frac::from_str(op.as_str()).expect("enforced by grammar");
                        if active_minus {
                            num *= Frac::new(-1, 1)
                        }
                        active_minus = false;
                        translation[i] = num;
                    }
                    sign => {
                        if op.as_str() == "-" {
                            active_minus = true
                        }
                    }
                    _ => unreachable!(), // by grammar
                }
            }
        }
        Self {
            mat: mat.into(),
            translation: translation.into(),
        }
    }

    /// creates an object from the parsed pair
    pub(crate) fn from_parser(pair: Pair<OpListRule>) -> Self {
        match pair.as_rule() {
            OpListRule::operation => Self::from_parser_operation(pair),
            OpListRule::translation_op => Self::from_parser_trans_op(pair),
            _ => unreachable!(), // by grammar
        }
    }

    /// returns the matrix of the transformation
    pub fn mat(&self) -> Mat3 {
        self.mat
    }

    /// returns the translation of the transformation
    pub fn translation(&self) -> Vec3 {
        self.translation
    }
}

impl From<Vec3> for Affine3 {
    fn from(value: Vec3) -> Self {
        Affine3 {
            mat: Mat3::identity(),
            translation: value,
        }
    }
}

impl From<Mat3> for Affine3 {
    fn from(value: Mat3) -> Self {
        Self {
            mat: value,
            translation: Vec3::zero(),
        }
    }
}

impl Into<nalgebra::Affine3<f32>> for Affine3 {
    fn into(self) -> nalgebra::Affine3<f32> {
        let mat: Matrix3<f32> = self.mat.into();
        let mut mat = mat
            .insert_fixed_rows::<1>(3, 0.0)
            .insert_fixed_columns::<1>(3, 0.0);
        let translation: Vector3<f32> = self.translation.into();
        let temp = translation.insert_fixed_rows::<1>(3, 1.0);
        mat.set_column(3, &temp);
        nalgebra::Affine3::from_matrix_unchecked(mat)
    }
}

impl Affine3 {
    /// calculates the determinant of the matrix of the transformation
    pub fn mat_determinant(&self) -> Frac {
        self.mat.determinant()
    }

    /// returns true if the transformation is invertible
    pub fn is_invertible(&self) -> bool {
        self.mat.is_invertible()
    }

    /// returns the inverse if it exists
    pub fn inverse(&self) -> Option<Self> {
        let inverse_mat = self.mat.inverse()?;
        Some(Self {
            mat: inverse_mat,
            translation: -(inverse_mat * self.translation),
        })
    }
}

/// A struct used for the remainder implementation
#[derive(Copy, Clone, Debug)]
pub struct Bounds3([i32; 3]);

impl Bounds3 {
    /// Creates a bounds struct with all values set to the given value
    pub fn splat(val: i32) -> Self {
        assert!(val > 0, "bounds must be positive");
        Self([val; 3])
    }

    /// Creates bounds which can include all roatations around the origin of the give vector
    pub fn include_rotations_of(vec: Vec3) -> Self {
        let max = vec.norm().ceil() as i32 + 1;
        Self([max; 3])
    }

    /// counts how many unit cells are inculded in the bounds
    pub fn volume(&self) -> i32 {
        self.0.iter().fold(1, |acc, a| acc * a)
    }

    /// returns x bound
    pub fn x(&self) -> i32 {
        self.0[0]
    }

    /// returns y bound
    pub fn y(&self) -> i32 {
        self.0[1]
    }

    /// returns z bound
    pub fn z(&self) -> i32 {
        self.0[2]
    }

    /// returns true if the position is in the correct form for these bounds
    pub fn contains_pos(&self, pos: Pos3) -> bool {
        self.0
            .iter()
            .zip(pos.0.into_iter())
            .fold(true, |acc, (&b, p)| {
                acc && (p < b.into()) && (p >= 0.into())
            })
    }

    /// returns true if the vector is in the correct form for these bounds
    pub fn contains_vec(&self, vec: Vec3) -> bool {
        self.0
            .iter()
            .zip(vec.0.into_iter())
            .fold(true, |acc, (&b, p)| {
                acc && (p <= Frac::new(b, 2)) && (p > Frac::new(-b, 2))
            })
    }

    /// returns true if all positions in the array are bigger than zero and smaller than the bounds
    /// at the corresponding index
    pub fn contains_arr(&self, arr: &[i32; 3]) -> bool {
        self.0.iter().zip(arr.iter()).fold(true, |acc, (&b, &p)| {
            acc && (p < b.into()) && (p >= 0.into())
        })
    }
}

impl<T: Into<i32>> From<[T; 3]> for Bounds3 {
    fn from(value: [T; 3]) -> Self {
        let mut arr: [i32; 3] = Default::default();
        for (i, val) in value.into_iter().enumerate() {
            arr[i] = val.into();
        }
        Self(arr)
    }
}
impl Display for Affine3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for (row, &translation) in (&self.mat.0).chunks(3).zip(self.translation.0.iter()) {
            if !out.is_empty() {
                out.push(',')
            }
            let mut coeff_op = String::new();
            for (val, ch) in row.iter().zip("xyz".chars()) {
                if *val != 0.into() {
                    coeff_op.push_str(&format!("{}{}", val.as_signed_prefactor(), ch));
                }
            }
            if translation != 0.into() {
                coeff_op.push_str(&format!("{}", translation.as_string_signed()));
            }
            if coeff_op.starts_with("+") {
                out.push_str(&coeff_op[1..]);
            } else {
                out.push_str(&coeff_op);
            }
        }
        write!(formatter, "{}", out)
    }
}

impl Mul for Affine3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            mat: self.mat * rhs.mat,
            translation: self.mat * rhs.translation + self.translation,
        }
    }
}

copy_mul_impl!(Affine3, Affine3);

impl Mul<Vec3> for Affine3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mat * rhs
    }
}

copy_mul_impl!(Affine3, Vec3);

impl Mul<Pos3> for Affine3 {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        self.mat * rhs + self.translation
    }
}

copy_mul_impl!(Affine3, Pos3);

impl Rem<Bounds3> for Affine3 {
    type Output = Affine3;

    fn rem(mut self, rhs: Bounds3) -> Self::Output {
        self.translation %= rhs;
        self
    }
}

impl Rem<&Bounds3> for Affine3 {
    type Output = Affine3;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        self % *rhs
    }
}

impl Rem<Bounds3> for &Affine3 {
    type Output = Affine3;

    fn rem(self, rhs: Bounds3) -> Self::Output {
        *self % rhs
    }
}

impl Rem<&Bounds3> for &Affine3 {
    type Output = Affine3;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        *self % *rhs
    }
}

impl RemAssign<Bounds3> for Affine3 {
    fn rem_assign(&mut self, rhs: Bounds3) {
        *self = *self % rhs
    }
}

impl RemAssign<&Bounds3> for Affine3 {
    fn rem_assign(&mut self, rhs: &Bounds3) {
        *self = *self % *rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_dot() {
        let vec1 = Vec3::new(1.into(), 2.into(), 3.into());
        let vec2 = Vec3::new(2.into(), 3.into(), 4.into());
        assert_eq!(vec1.dot(&vec2), (1 * 2 + 2 * 3 + 3 * 4).into());
    }

    #[test]
    fn test_mat3_from_columns() {
        let vec1 = Vec3::new(1.into(), 2.into(), 3.into());
        let vec2 = Vec3::new(4.into(), 5.into(), 6.into());
        let vec3 = Vec3::new(7.into(), 8.into(), 9.into());
        let mat = Mat3::from_columns(vec1, vec2, vec3);
        assert_eq!(mat.0[0], 1.into());
        assert_eq!(mat.0[3], 2.into());
        assert_eq!(mat.0[6], 3.into());
    }

    #[rustfmt::skip]
    #[test]
    fn test_mat3_determinant() {
        let mat = Mat3([
            1.into(), 2.into(), 3.into(),
            4.into(), 5.into(), 6.into(),
            7.into(), 8.into(), 9.into(),
        ]);
        assert_eq!(
            mat.determinant(),
            (1 * 5 * 9 - 1 * 6 * 8 - 2 * 4 * 9 + 2 * 6 * 7 + 3 * 4 * 8 - 3 * 5 * 7).into()
        );
    }

    // Add more tests for other Mat3 methods

    #[test]
    #[rustfmt::skip]
    fn test_mat3_mul() {
        let mat = Mat3([
            9.into(), 8.into(), 7.into(),
            6.into(), 5.into(), 4.into(),
            3.into(), 2.into(), 1.into(),
        ]);
        let other = Mat3([
            1.into(), 0.into(), 0.into(),
            0.into(), 1.into(), 0.into(),
            0.into(), 0.into(), 1.into(),
        ]);
        assert_eq!(other * mat, mat);
        assert_eq!(mat * other, mat);

        let other = Mat3([
            0.into(), 1.into(), 0.into(),
            1.into(), 0.into(), 0.into(),
            0.into(), 0.into(), 1.into(),
        ]);
        assert_eq!(other * mat, Mat3([
            6.into(), 5.into(), 4.into(),
            9.into(), 8.into(), 7.into(),
            3.into(), 2.into(), 1.into(),
        ]));
        assert_eq!(mat * other, Mat3([
            8.into(), 9.into(), 7.into(),
            5.into(), 6.into(), 4.into(),
            2.into(), 3.into(), 1.into(),

        ]));

        let other = Mat3([
            1.into(), 0.into(), 0.into(),
            0.into(), 0.into(), 1.into(),
            0.into(), 1.into(), 0.into(),
        ]);
        assert_eq!(other * mat, Mat3([
            9.into(), 8.into(), 7.into(),
            3.into(), 2.into(), 1.into(),
            6.into(), 5.into(), 4.into(),
        ]));
        assert_eq!(mat * other, Mat3([
            9.into(), 7.into(), 8.into(), 
            6.into(), 4.into(), 5.into(), 
            3.into(), 1.into(), 2.into(), 

        ]))
    }

    #[rustfmt::skip]
    #[test]
    fn test_inverse_mat() {
        let mat = Mat3([
            1.into(), 4.into(), 4.into(),
            5.into(), 2.into(), 5.into(),
            3.into(), 2.into(), 5.into(),
        ]);
        assert_eq!(Mat3::identity(), mat * mat.inverse().unwrap());
        let mat = Mat3([
            1.into(), 4.into(), 4.into(),
            5.into(), 2.into(), 3.into(),
            Frac::new(1, 2), 1.into(), 5.into(),
        ]);
        assert_eq!(Mat3::identity(), mat * mat.inverse().unwrap());
        let mat: Mat3 = [1, 4, 4,
                   5, 2, 5,
                   0, 0, 0].into();
        assert!(mat.inverse().is_none());
    }

    #[rustfmt::skip]
    #[test]
    fn test_inverse_affine() {
        let mat = [1, 4, 4,
                   5, 2, 5,
                   3, 2, 5].into();
        let translation = [1, 2, 0].into();
        let affine = Affine3::new(mat, translation);
        let inverse = affine.inverse().unwrap();
        assert_eq!(Affine3::identity(), affine * inverse);
        assert_eq!(Affine3::identity(), inverse * affine);
        let pos: Pos3 = [1, 0, 1].into();
        assert_eq!(pos, inverse * (affine * pos));
        let vec: Vec3 = [11, 12, 2].into();
        assert_eq!(vec, inverse * (affine * vec));
    }

    #[test]
    fn test_rem() {
        let super_cell: Bounds3 = [3, 2, 1].into();
        assert_eq!(
            Pos3::origin(),
            Pos3::new(3.into(), 2.into(), 1.into()) % super_cell
        );
        assert_eq!(
            Pos3::new(2.into(), 0.into(), 0.into()),
            Pos3::new(2.into(), 2.into(), 1.into()) % super_cell
        );
        assert_eq!(
            Vec3::new((-1).into(), 1.into(), 0.into()),
            Vec3::new(2.into(), 1.into(), 1.into()) % super_cell
        );
    }

    #[test]
    fn contains_test() {
        let bounds: Bounds3 = [1, 4, 5].into();
        let mut vec: Vec3 = [2, 4, 4].into();
        assert!(bounds.contains_vec(vec % bounds));
        vec /= Frac::new(2, 1);
        assert!(bounds.contains_vec(vec % bounds));
        vec /= Frac::new(3, 1);
        assert!(bounds.contains_vec(vec % bounds));
    }
}
