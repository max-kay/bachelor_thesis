//! defines macros used in this crate

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
