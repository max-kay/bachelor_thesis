//! this module defines Symmetry groups and elements
use std::{
    fs::read_to_string,
    ops::{Mul, Rem, RemAssign},
    path::Path,
};

use pest::Parser;
use pest::{error::Error, iterators::Pair};

use crate::{copy_mul_impl, Affine3, Mat3, OpListParser, OpListRule, Pos3, Vec3};

/// a type representing a crystallographic symmetry operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymmetryElement(Affine3);

impl SymmetryElement {
    /// constructor returns some if the operation is isometric
    pub fn new(operation: Affine3) -> Option<Self> {
        if !(operation.mat_determinant().abs() == 1.into()) {
            return None;
        }
        Some(Self(operation))
    }

    /// constructor from matrix returns Some if the matrix is orthogonal
    pub fn from_mat(mat: Mat3) -> Option<Self> {
        Self::new(Affine3::from_mat(mat))
    }

    /// constructor from translation
    pub fn from_translation(translation: Vec3) -> Self {
        Self(Affine3::from_translation(translation))
    }

    pub fn from_parser(pair: Pair<OpListRule>) -> Self {
        assert_eq!(pair.as_rule(), OpListRule::operation); // TODO might be unnecessary
        Self::new(Affine3::from_parser(pair)).unwrap()
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
pub struct SpaceGroup {
    operations: Vec<SymmetryElement>,
}

impl SpaceGroup {
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

    pub fn from_oplist(oplist: &str) -> Result<Self, Error<OpListRule>> {
        let parsed = OpListParser::parse(OpListRule::op_list, oplist)?
            .next()
            .expect("never fails according to docs");
        assert_eq!(parsed.as_rule(), OpListRule::op_list);
        let pairs = parsed.into_inner();
        for pair in pairs {
            println!("{}", pair)
        }
        todo!()
    }

    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let string = read_to_string(path).unwrap();
        Self::from_oplist(&string).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn parse_test() {
        SpaceGroup::from_file("groups/point_groups/example.sg");
    }
}
