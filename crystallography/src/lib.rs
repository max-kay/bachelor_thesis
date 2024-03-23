//! This crate provides types and methods to work with crystallographic groups
#![warn(missing_docs)]
pub mod affine_space;
mod frac;
pub(crate) mod parsers;
pub mod symmetry;

pub use affine_space::{Affine3, Mat3, Pos3, Vec3};
pub use frac::Frac;
pub use parsers::oplist::{Parser as OpListParser, Rule as OpListRule};

/// implements the Mul trait for references of type that is copy and implements Mul
#[macro_export]
macro_rules! copy_mul_impl {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Mul<&$rhs> for $lhs {
            type Output = <$lhs as std::ops::Mul<$rhs>>::Output;

            fn mul(self, rhs: &$rhs) -> Self::Output {
                self * *rhs
            }
        }

        impl std::ops::Mul<$rhs> for &$lhs {
            type Output = <$lhs as std::ops::Mul<$rhs>>::Output;

            fn mul(self, rhs: $rhs) -> Self::Output {
                *self * rhs
            }
        }

        impl std::ops::Mul<&$rhs> for &$lhs {
            type Output = <$lhs as std::ops::Mul<$rhs>>::Output;

            fn mul(self, rhs: &$rhs) -> Self::Output {
                *self * *rhs
            }
        }
    };
}
