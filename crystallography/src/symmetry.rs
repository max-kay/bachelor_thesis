//! this module defines Symmetry groups and elements

use std::{
    fmt::Display,
    fs::read_to_string,
    ops::{Mul, Rem, RemAssign},
    path::Path,
};

use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser;
use thiserror::Error;

use crate::{copy_mul_impl, Affine3, Mat3, OpListParser, OpListRule, Pos3, Vec3};

#[derive(Error, Debug)]
enum InvalidOpError {
    #[error("{0} doesnt have deterimnant +/- 1")]
    Deterimnant(Affine3),
}

/// a type representing a crystallographic symmetry operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymmetryElement(Affine3);

impl SymmetryElement {
    /// constructor returns some if the operation has determinant +/-1
    pub fn new(operation: Affine3) -> Result<Self> {
        if !(operation.mat_determinant().abs() == 1.into()) {
            return Err(InvalidOpError::Deterimnant(operation))?;
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

    pub fn from_parser(pair: Pair<OpListRule>) -> Result<Self> {
        assert_eq!(pair.as_rule(), OpListRule::operation); // TODO might be unnecessary
        Ok(Self::new(Affine3::from_parser(pair))?)
    }
}

impl SymmetryElement {
    /// returns the inverse of the operation
    pub fn invert(&self) -> Self {
        Self(
            self.0
                .inverse()
                .expect("SymmetryElements are always invertible"),
        )
    }
}

impl Display for SymmetryElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Mul for SymmetryElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

copy_mul_impl!(SymmetryElement, SymmetryElement);

impl Mul<Vec3> for SymmetryElement {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(SymmetryElement, Vec3);

impl Mul<Pos3> for SymmetryElement {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(SymmetryElement, Pos3);

impl Rem<Vec3> for SymmetryElement {
    type Output = SymmetryElement;

    fn rem(mut self, rhs: Vec3) -> Self::Output {
        self.0 %= rhs;
        self
    }
}

impl Rem<&Vec3> for SymmetryElement {
    type Output = SymmetryElement;

    fn rem(self, rhs: &Vec3) -> Self::Output {
        self % *rhs
    }
}

impl Rem<Vec3> for &SymmetryElement {
    type Output = SymmetryElement;

    fn rem(self, rhs: Vec3) -> Self::Output {
        *self % rhs
    }
}

impl Rem<&Vec3> for &SymmetryElement {
    type Output = SymmetryElement;

    fn rem(self, rhs: &Vec3) -> Self::Output {
        *self % *rhs
    }
}

impl RemAssign<Vec3> for SymmetryElement {
    fn rem_assign(&mut self, rhs: Vec3) {
        *self = *self % rhs
    }
}

impl RemAssign<&Vec3> for SymmetryElement {
    fn rem_assign(&mut self, rhs: &Vec3) {
        *self = *self % *rhs
    }
}
/// A struct represnting a space group
#[derive(Debug)]
pub struct SpaceGroup {
    operations: Vec<SymmetryElement>,
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

impl SpaceGroup {
    /// this function takes a Vec of symmetries and tries to close them under multiplication.
    /// all operations are performed modulo (1, 1, 1) as defined in the affine space module
    /// panics if this cannot be done within 10_000 iterations to prevent an infinite loop.
    pub fn from_symmetries(symmetries: Vec<SymmetryElement>) -> Self {
        let mut operations: Vec<SymmetryElement> = Vec::new();
        for op in symmetries {
            let op = op % Vec3::splat(1.into());
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
                    let op = (operations[i] * operations[j]) % Vec3::splat(1.into());
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
            operations.push(SymmetryElement::from_parser(pair)?);
        }
        Ok(Self::from_symmetries(operations))
    }

    /// this function is a convenience function reading a file and passing the string to
    /// SpaceGroup::from_oplist
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let string = read_to_string(path)?;
        Self::from_oplist(&string).into()
    }
}

impl Display for SpaceGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in &self.operations {
            write!(f, "{};", op)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn parse_test() {
        let sg = SpaceGroup::from_file("groups/point_groups/example.sg").unwrap();
        let as_string = format!("{}", sg);
        println!("{:?}", sg);
        println!("{}", as_string);
        assert_eq!(sg, SpaceGroup::from_oplist(&as_string).unwrap());
    }
}
