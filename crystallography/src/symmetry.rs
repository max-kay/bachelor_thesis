//! this module defines Symmetry groups and elements

use std::{
    fmt::Display,
    fs::read_to_string,
    ops::{Mul, Rem, RemAssign},
    path::Path,
    slice,
};

use anyhow::Result;
use nalgebra::Matrix3;
use pest::iterators::Pair as ParserPair;
use pest::Parser;
use thiserror::Error;

use crate::{
    affine_space::Bounds3, copy_mul_impl, Affine3, Mat3, OpListParser, OpListRule, Pos3, Vec3,
};

#[derive(Error, Debug)]
enum InvalidOpError {
    #[error("{0} doesnt have determinant +/- 1")]
    IsometryGroup(Affine3),
    #[error("{0} doesnt have determinant +/- 1")]
    PointGroup(Mat3),
}

/// a type representing a point group element
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointGroupElement(Mat3);

impl PointGroupElement {
    /// constructor returns Err if the determinant is not +/- 1
    pub fn new(mat: Mat3) -> Result<Self> {
        if !(mat.determinant().abs() == 1.into()) {
            return Err(InvalidOpError::PointGroup(mat).into());
        }
        Ok(Self(mat))
    }

    /// inverts the element
    pub fn invert(&self) -> Self {
        Self(
            self.0
                .inverse()
                .expect("point group elements are always inverible"),
        )
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
    symmetries: Vec<PointGroupElement>,
}

impl PointGroup {
    /// constructor from generators
    /// this function tries to produce closure under multiplication
    /// panics if closure cannot be reached within 10_000 moves
    pub fn from_generators(generators: Vec<PointGroupElement>) -> Self {
        let mut symmetries: Vec<PointGroupElement> = Vec::new();
        for op in generators {
            if !symmetries.contains(&op) {
                symmetries.push(op)
            }
        }
        let mut counter = 0;
        let mut added_new = true;
        while added_new && counter < 10_000 {
            counter += 1;
            added_new = false;
            for i in 0..symmetries.len() {
                for j in 0..symmetries.len() {
                    let op = symmetries[i] * symmetries[j];
                    if !symmetries.contains(&op) {
                        symmetries.push(op);
                        added_new = true;
                    }
                }
            }
        }
        if !(counter < 10_000) {
            panic!("didn't manage to close group within 10'000 iterations");
        }
        Self { symmetries }
    }

    /// creates a point group from a set of symmetries closed under multiplication
    /// dedups the elements first
    /// returns None if the group is not closed
    pub fn from_closed_symmetries(mut symmetries: Vec<PointGroupElement>) -> Option<Self> {
        symmetries.sort();
        symmetries.dedup();
        let this = Self { symmetries };
        if !this.is_closed() {
            return None;
        }
        Some(this)
    }

    /// returns true if the group is closed
    fn is_closed(&self) -> bool {
        for sym1 in &self.symmetries {
            if !self.symmetries.contains(&sym1.invert()) {
                return false;
            }
            for sym2 in &self.symmetries {
                if !self.symmetries.contains(&(sym1 * sym2)) {
                    return false;
                }
                if !self.symmetries.contains(&(sym2 * sym1)) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl PointGroup {
    /// retruns an iterator over the operations
    pub fn iter(&self) -> slice::Iter<PointGroupElement> {
        self.symmetries.iter()
    }
}

impl PartialEq for PointGroup {
    fn eq(&self, other: &Self) -> bool {
        if !self.symmetries.len() == other.symmetries.len() {
            return false;
        }
        for op in &self.symmetries {
            if !other.symmetries.contains(&op) {
                return false;
            }
        }
        true
    }
}

impl IntoIterator for PointGroup {
    type Item = PointGroupElement;

    type IntoIter = <Vec<PointGroupElement> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.symmetries.into_iter()
    }
}

impl Eq for PointGroup {}

/// a type representing a crystallographic symmetry operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Isometry(Affine3);

impl Isometry {
    /// constructor returns ok if the operation has determinant +/-1
    pub fn new(operation: Affine3) -> Result<Self> {
        if !(operation.mat_determinant().abs() == 1.into()) {
            return Err(InvalidOpError::IsometryGroup(operation).into());
        }
        Ok(Self(operation))
    }

    /// constructor from matrix returns Ok if the matrix has determinant +/-1
    pub fn from_mat(mat: Mat3) -> Result<Self> {
        Self::new(Affine3::from_mat(mat))
    }

    /// constructor from translation
    pub fn from_translation(translation: Vec3) -> Self {
        Self(Affine3::from_translation(translation))
    }

    /// creates the symmetry element from a parsed pair
    pub(crate) fn from_parser(pair: ParserPair<OpListRule>) -> Result<Self> {
        Ok(Self::new(Affine3::from_parser(pair))?)
    }

    /// removes the translation from the element and returns the associated point group element
    pub fn reduce_to_point_group_element(&self) -> PointGroupElement {
        PointGroupElement(self.0.mat())
    }
}

impl Into<nalgebra::Affine3<f32>> for Isometry {
    fn into(self) -> nalgebra::Affine3<f32> {
        self.0.into()
    }
}

impl Isometry {
    /// returns the inverse of the operation
    pub fn invert(&self) -> Self {
        Self(
            self.0
                .inverse()
                .expect("SymmetryElements are always invertible"),
        )
    }
}

impl Display for Isometry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Mul for Isometry {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

copy_mul_impl!(Isometry, Isometry);

impl Mul<Vec3> for Isometry {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(Isometry, Vec3);

impl Mul<Pos3> for Isometry {
    type Output = Pos3;

    fn mul(self, rhs: Pos3) -> Self::Output {
        self.0 * rhs
    }
}

copy_mul_impl!(Isometry, Pos3);

impl Rem<Bounds3> for Isometry {
    type Output = Isometry;

    fn rem(mut self, rhs: Bounds3) -> Self::Output {
        self.0 %= rhs;
        self
    }
}

impl Rem<&Bounds3> for Isometry {
    type Output = Isometry;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        self % *rhs
    }
}

impl Rem<Bounds3> for &Isometry {
    type Output = Isometry;

    fn rem(self, rhs: Bounds3) -> Self::Output {
        *self % rhs
    }
}

impl Rem<&Bounds3> for &Isometry {
    type Output = Isometry;

    fn rem(self, rhs: &Bounds3) -> Self::Output {
        *self % *rhs
    }
}

impl RemAssign<Bounds3> for Isometry {
    fn rem_assign(&mut self, rhs: Bounds3) {
        *self = *self % rhs
    }
}

impl RemAssign<&Bounds3> for Isometry {
    fn rem_assign(&mut self, rhs: &Bounds3) {
        *self = *self % *rhs
    }
}

/// A struct representing a space group
/// internaly the space group is represented as the qutient group of the space group modulo the
/// group genreated by translations along axes by the integers of the given Bounds3.
#[derive(Debug, Clone)]
pub struct IsometryGroup {
    symmetries: Vec<Isometry>,
}

impl PartialEq for IsometryGroup {
    fn eq(&self, other: &Self) -> bool {
        if !self.symmetries.len() == other.symmetries.len() {
            return false;
        }
        for op in &self.symmetries {
            if !other.symmetries.contains(&op) {
                return false;
            }
        }
        true
    }
}

impl Eq for IsometryGroup {}

impl IsometryGroup {
    /// this function takes a Vec of symmetries and tries to close them under multiplication.
    /// all operations are performed modulo (1, 1, 1) as defined in the affine space module
    /// panics if this cannot be done within 10_000 iterations to prevent an infinite loop.
    pub fn from_generators(generators: Vec<Isometry>) -> Self {
        let mut symmetries: Vec<Isometry> = Vec::new();
        for op in generators {
            let op = op % Bounds3::splat(1.into());
            if !symmetries.contains(&op) {
                symmetries.push(op)
            }
        }
        let mut counter = 0;
        let mut added_new = true;
        while added_new && counter < 10_000 {
            counter += 1;
            added_new = false;
            for i in 0..symmetries.len() {
                for j in 0..symmetries.len() {
                    let op = (symmetries[i] * symmetries[j]) % Bounds3::splat(1.into());
                    if !symmetries.contains(&op) {
                        symmetries.push(op);
                        added_new = true;
                    }
                }
            }
        }
        if !(counter < 10_000) {
            panic!("didn't manage to close group within 10'000 iterations");
        }
        Self { symmetries }
    }

    /// this function takes a oplist as a string and parses it
    /// note that the parsed symmetry operations are sent through SpaceGroup::from_symmetries thus
    /// the same conditions for panicing applies
    pub fn from_oplist(oplist: &str) -> Result<Self> {
        let parsed = OpListParser::parse(OpListRule::op_list, oplist)?
            .next()
            .expect("never fails according to docs");
        let pairs = parsed.into_inner();
        let mut symmetries = Vec::new();
        for pair in pairs {
            symmetries.push(Isometry::from_parser(pair)?);
        }
        Ok(Self::from_generators(symmetries))
    }

    /// this function is a convenience function reading a file and passing the string to
    /// SpaceGroup::from_oplist
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let string = read_to_string(path)?;
        Self::from_oplist(&string).into()
    }

    /// returns an iterator over the symmetry operations in the given bounds
    pub fn iter_with_bounds(&self, bounds: Bounds3) -> IsometryIter<'_, Isometry> {
        IsometryIter::new(&self.symmetries, bounds)
    }

    /// creates a group from the elements given
    /// returns None if the elements are not given modulo the bounds or if the set of elements is
    /// not closed under multiplication modulo the bounds, also dedups the elements
    pub fn from_closed_symmetries(mut symmetries: Vec<Isometry>) -> Option<Self> {
        symmetries.sort();
        symmetries.dedup();
        for sym in symmetries.iter() {
            assert_eq!(*sym, sym % Bounds3::splat(1))
        }
        let this = Self { symmetries };
        if !this.check_represetation() {
            return None;
        }
        Some(this)
    }

    /// checks closure and if the elements are given modulo the bounds
    /// ignores duplicates
    fn check_represetation(&self) -> bool {
        for sym1 in &self.symmetries {
            if !self
                .symmetries
                .contains(&(sym1.invert() % Bounds3::splat(1)))
            {
                return false;
            }
            if !(sym1 % Bounds3::splat(1) == *sym1) {
                return false;
            }
            for sym2 in &self.symmetries {
                if !self
                    .symmetries
                    .contains(&((sym1 * sym2) % Bounds3::splat(1)))
                {
                    return false;
                }
                if !self
                    .symmetries
                    .contains(&((sym2 * sym1) % Bounds3::splat(1)))
                {
                    return false;
                }
            }
        }
        true
    }

    /// removes the translation part of the each element of the spacae group and returns a point
    /// group
    pub fn reduce_to_point_group(&self) -> PointGroup {
        let symmetries = self
            .symmetries
            .iter()
            .map(Isometry::reduce_to_point_group_element)
            .collect();
        PointGroup::from_closed_symmetries(symmetries)
            .expect("a space group can always be reduced to a point group")
    }
}

impl IsometryGroup {
    /// returns true if the operation is an element of the space group
    pub fn contains(&self, op: Isometry) -> bool {
        let op = op % Bounds3::splat(1.into());
        self.symmetries.contains(&op)
    }

    /// returns the cardinality of the quotient_group with unit translations
    pub fn len(&self) -> usize {
        self.symmetries.len()
    }

    /// returns a list of all operations in the group
    pub fn get_operations(&self) -> &[Isometry] {
        &self.symmetries
    }
}

impl Display for IsometryGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in &self.symmetries {
            writeln!(f, "{};", op)?;
        }
        Ok(())
    }
}

/// An Iterator over the elements of a bounded space group
pub struct IsometryIter<'a, T>
where
    Isometry: Mul<&'a T, Output = T>,
    T: Rem<Bounds3, Output = T>,
{
    symmetries: std::slice::Iter<'a, T>,
    current_item: Option<&'a T>,
    bounds: Bounds3,
    state: [i32; 3],
}

impl<'a, T> IsometryIter<'a, T>
where
    Isometry: Mul<&'a T, Output = T>,
    T: Rem<Bounds3, Output = T>,
{
    /// constructor
    pub fn new(symmetries: &'a [T], bounds: Bounds3) -> Self {
        Self {
            symmetries: symmetries.iter(),
            current_item: None,
            bounds,
            state: [0; 3],
        }
    }

    fn increase_state(&mut self) {
        self.state[2] += 1;
        if self.state[2] >= self.bounds.z() {
            self.state[1] += 1;
            self.state[2] = 0;
        }
        if self.state[1] >= self.bounds.y() {
            self.state[0] += 1;
            self.state[1] = 0;
        }
    }
}

impl<'a, T> Iterator for IsometryIter<'a, T>
where
    Isometry: Mul<&'a T, Output = T>,
    T: Rem<Bounds3, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // the state of the iter here is such that if it is still in the bounds
        // I can just take it as the translation vector
        if self.current_item.is_none() {
            self.current_item = self.symmetries.next();
        }
        if let Some(item) = self.current_item {
            if self.state[0] < self.bounds.x() {
                let next =
                    Some((Isometry::from_translation(self.state.into()) * item) % self.bounds);
                self.increase_state();
                return next;
            } else {
                self.state = [0; 3];
                self.current_item = None;
                return self.next();
            }
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_sg {
        ($path:literal, $expected_number:literal) => {
            let sg = IsometryGroup::from_file($path).unwrap();
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
        test_sg!("groups/space_groups/R-3m", 36);
    }

    #[test]
    pub fn iter_test() {
        let sg = IsometryGroup::from_file("groups/space_groups/P-1").unwrap();
        let ops: Vec<_> = sg.iter_with_bounds(Bounds3::splat(1)).collect();
        assert_eq!(ops.len(), 2);
        let ops: Vec<_> = sg.iter_with_bounds(Bounds3::splat(2)).collect();
        assert_eq!(ops.len(), 2 * 2 * 2 * 2);
        let ops: Vec<_> = sg.iter_with_bounds([1, 2, 1].into()).collect();
        assert_eq!(ops.len(), 2 * 2);
        let ops: Vec<_> = sg.iter_with_bounds([3, 2, 1].into()).collect();
        assert_eq!(ops.len(), 3 * 2 * 2);
    }
}
