//! this module defines Symmetry groups and elements

use std::{
    fmt::Display,
    fs::read_to_string,
    ops::{Mul, Rem, RemAssign},
    path::Path,
};

use anyhow::Result;
use nalgebra::Matrix3;
use pest::iterators::Pair;
use pest::Parser;
use thiserror::Error;

use crate::{
    affine_space::Bounds3, copy_mul_impl, Affine3, Mat3, OpListParser, OpListRule, Pos3, Vec3,
};

#[derive(Error, Debug)]
enum InvalidOpError {
    #[error("{0} doesnt have determinant +/- 1")]
    SpaceGroup(Affine3),
    #[error("{0} doesnt have determinant +/- 1")]
    PointGroup(Mat3),
}

/// a type representing a point group element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointGroupElement(Mat3);

impl PointGroupElement {
    /// constructor returns Err if the determinant is not +/- 1
    pub fn new(mat: Mat3) -> Result<Self> {
        if !(mat.determinant().abs() == 1.into()) {
            return Err(InvalidOpError::PointGroup(mat).into());
        }
        Ok(Self(mat))
    }
}

impl Into<Matrix3<f32>> for PointGroupElement {
    fn into(self) -> Matrix3<f32> {
        self.0.into()
    }
}

impl Mul for PointGroupElement {
    type Output = PointGroupElement;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

copy_mul_impl!(PointGroupElement, PointGroupElement);

impl Mul<Vec3> for PointGroupElement {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(PointGroupElement, Vec3);

impl Mul<Pos3> for PointGroupElement {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(PointGroupElement, Pos3);

/// a struct representing a pointgroup
pub struct PointGroup {
    operations: Vec<PointGroupElement>,
}

impl PointGroup {
    /// constructor from generators
    /// this function tries to produce closure under multiplication
    /// panics if closure cannot be reached within 10_000 moves
    pub fn from_generators(generators: Vec<PointGroupElement>) -> Self {
        let mut operations: Vec<PointGroupElement> = Vec::new();
        for op in generators {
            if !operations.contains(&op) {
                operations.push(op)
            }
        }
        let mut counter = 0;
        let mut added_new = true;
        while added_new && counter < 10_000 {
            counter += 1;
            added_new = false;
            for i in 0..operations.len() {
                for j in 0..operations.len() {
                    let op = operations[i] * operations[j];
                    if !operations.contains(&op) {
                        operations.push(op);
                        added_new = true;
                    }
                }
            }
        }
        if !(counter < 10_000) {
            panic!("didn't manage to close group within 10'000 iterations");
        }
        Self { operations }
    }
}

impl PartialEq for PointGroup {
    fn eq(&self, other: &Self) -> bool {
        if !self.operations.len() == other.operations.len() {
            return false;
        }
        for op in &self.operations {
            if !other.operations.contains(&op) {
                return false;
            }
        }
        true
    }
}

impl Eq for PointGroup {}

/// a type representing a crystallographic symmetry operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpaceGroupElement(Affine3);

impl SpaceGroupElement {
    /// constructor returns some if the operation has determinant +/-1
    pub fn new(operation: Affine3) -> Result<Self> {
        if !(operation.mat_determinant().abs() == 1.into()) {
            return Err(InvalidOpError::SpaceGroup(operation).into());
        }
        Ok(Self(operation))
    }

    /// constructor from matrix returns Some if the matrix has determinant +/-1
    pub fn from_mat(mat: Mat3) -> Result<Self> {
        Self::new(Affine3::from_mat(mat))
    }

    /// constructor from translation
    pub fn from_translation(translation: Vec3) -> Self {
        Self(Affine3::from_translation(translation))
    }

    /// creates the symmetry element from a parsed pair
    pub(crate) fn from_parser(pair: Pair<OpListRule>) -> Result<Self> {
        Ok(Self::new(Affine3::from_parser(pair))?)
    }
}

impl Into<nalgebra::Affine3<f32>> for SpaceGroupElement {
    fn into(self) -> nalgebra::Affine3<f32> {
        self.0.into()
    }
}

impl SpaceGroupElement {
    /// returns the inverse of the operation
    pub fn invert(&self) -> Self {
        Self(
            self.0
                .inverse()
                .expect("SymmetryElements are always invertible"),
        )
    }
}

impl Display for SpaceGroupElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Mul for SpaceGroupElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

copy_mul_impl!(SpaceGroupElement, SpaceGroupElement);

impl Mul<Vec3> for SpaceGroupElement {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(SpaceGroupElement, Vec3);

impl Mul<Pos3> for SpaceGroupElement {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(SpaceGroupElement, Pos3);

impl Rem<Bounds3> for SpaceGroupElement {
    type Output = SpaceGroupElement;

    fn rem(mut self, rhs: Bounds3) -> Self::Output {
        self.0 %= rhs;
        self
    }
}

impl Rem<&Bounds3> for SpaceGroupElement {
    type Output = SpaceGroupElement;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        self % *rhs
    }
}

impl Rem<Bounds3> for &SpaceGroupElement {
    type Output = SpaceGroupElement;

    fn rem(self, rhs: Bounds3) -> Self::Output {
        *self % rhs
    }
}

impl Rem<&Bounds3> for &SpaceGroupElement {
    type Output = SpaceGroupElement;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        *self % *rhs
    }
}

impl RemAssign<Bounds3> for SpaceGroupElement {
    fn rem_assign(&mut self, rhs: Bounds3) {
        *self = *self % rhs
    }
}

impl RemAssign<&Bounds3> for SpaceGroupElement {
    fn rem_assign(&mut self, rhs: &Bounds3) {
        *self = *self % *rhs
    }
}
/// A struct representing a space group
/// internaly the space group is represented as the qutient group of the space group modulo the
/// group genreated by translations along axes.
#[derive(Debug)]
pub struct SpaceGroup {
    operations: Vec<SpaceGroupElement>,
}

impl PartialEq for SpaceGroup {
    fn eq(&self, other: &Self) -> bool {
        if !self.operations.len() == other.operations.len() {
            return false;
        }
        for op in &self.operations {
            if !other.operations.contains(&op) {
                return false;
            }
        }
        true
    }
}

impl Eq for SpaceGroup {}

impl SpaceGroup {
    /// this function takes a Vec of symmetries and tries to close them under multiplication.
    /// all operations are performed modulo (1, 1, 1) as defined in the affine space module
    /// panics if this cannot be done within 10_000 iterations to prevent an infinite loop.
    pub fn from_generators(generators: Vec<SpaceGroupElement>) -> Self {
        let mut operations: Vec<SpaceGroupElement> = Vec::new();
        for op in generators {
            let op = op % Bounds3::splat(1.into());
            if !operations.contains(&op) {
                operations.push(op)
            }
        }
        let mut counter = 0;
        let mut added_new = true;
        while added_new && counter < 10_000 {
            counter += 1;
            added_new = false;
            for i in 0..operations.len() {
                for j in 0..operations.len() {
                    let op = (operations[i] * operations[j]) % Bounds3::splat(1.into());
                    if !operations.contains(&op) {
                        operations.push(op);
                        added_new = true;
                    }
                }
            }
        }
        if !(counter < 10_000) {
            panic!("didn't manage to close group within 10'000 iterations");
        }
        Self { operations }
    }

    /// this function takes a oplist as a string and parses it
    /// note that the parsed symmetry operations are sent through SpaceGroup::from_symmetries thus
    /// the same conditions for panicing applies
    pub fn from_oplist(oplist: &str) -> Result<Self> {
        let parsed = OpListParser::parse(OpListRule::op_list, oplist)?
            .next()
            .expect("never fails according to docs");
        assert_eq!(parsed.as_rule(), OpListRule::op_list); // TODO might be unnecessary
        let pairs = parsed.into_inner();
        let mut operations = Vec::new();
        for pair in pairs {
            operations.push(SpaceGroupElement::from_parser(pair)?);
        }
        Ok(Self::from_generators(operations))
    }

    /// this function is a convenience function reading a file and passing the string to
    /// SpaceGroup::from_oplist
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let string = read_to_string(path)?;
        Self::from_oplist(&string).into()
    }
}

impl SpaceGroup {
    /// returns true if the operation is an element of the space group
    pub fn contains(&self, op: SpaceGroupElement) -> bool {
        let op = op % Bounds3::splat(1.into());
        self.operations.contains(&op)
    }

    /// returns the cardinality of the quotient_group with unit translations
    pub fn len(&self) -> usize {
        self.operations.len()
    }
}

impl Display for SpaceGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in &self.operations {
            writeln!(f, "{};", op)?;
        }
        Ok(())
    }
}

/// this struct represents
pub struct Site {
    position: Pos3,
    stabilizer: Vec<SpaceGroupElement>,
    orbit: Vec<Pos3>,
}

impl Site {
    /// create a new site calculating the orbit and the stabilizer
    pub fn new(group: &SpaceGroup, position: Pos3) -> Self {
        let mut orbit = vec![position];
        let mut stabilizer = Vec::new();
        for &op in group.operations.iter() {
            let new_pos = (op * position) % Bounds3::splat(1.into());
            if new_pos == position {
                stabilizer.push(op)
            }
            if !orbit.contains(&new_pos) {
                orbit.push(new_pos)
            }
        }
        Self {
            position,
            stabilizer,
            orbit,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_sg {
        ($path:literal, $expected_number:literal) => {
            let sg = SpaceGroup::from_file($path).unwrap();
            assert_eq!($expected_number, sg.len())
        };
    }

    #[test]
    pub fn cardinality_test() {
        test_sg!("groups/space_groups/P-1", 2);
        test_sg!("groups/space_groups/P2_1", 2);
        test_sg!("groups/space_groups/C2|m", 8);
        test_sg!("groups/space_groups/P2_12_12", 4);
        test_sg!("groups/space_groups/P2_12_12_1", 4);
        test_sg!("groups/space_groups/Pmna", 8);
        test_sg!("groups/space_groups/Cmcm", 16);
        test_sg!("groups/space_groups/P6_3|mmc", 24);
        test_sg!("groups/space_groups/Fm-3m", 192);
    }
}
