use std::marker::PhantomData;

use nalgebra::Matrix3;

macro_rules! make_family {
    ($type_name:ident, $name:literal) => {
        /// Crystal Family
        pub enum $type_name {}
        impl CrystalFamily for $type_name {
            const NAME: &'static str = $name;
            const FULL_NAME: &'static str = stringify!($type_name);
        }
    };
}

macro_rules! make_centering {
    ($name:ident) => {
        /// Crystallogrphic Centering
        pub enum $name {}
        impl Centering for $name {
            const NAME: &'static str = stringify!($name);
        }
    };
}

macro_rules! give_centering_opts {
    ($family:ident, $($centering:ident),+) => {
        $(
            impl CenteringOf<$family> for $centering {}
        )*
    };
}

/// a struct representing the crystal parameters
#[derive(Debug, Clone, Copy)]
pub struct UnitCellParameters {
    lengths: [f32; 3],
    angles: [f32; 3],
}

impl UnitCellParameters {
    /// this function returns the metric tensor
    pub fn get_metric_tensor(&self) -> Matrix3<f32> {
        [
            [
                self.lengths[0] * self.lengths[0],
                self.lengths[0] * self.lengths[1] * self.angles[2].cos(),
                self.lengths[0] * self.lengths[2] * self.angles[1].cos(),
            ],
            [
                self.lengths[0] * self.lengths[1] * self.angles[2].cos(),
                self.lengths[1] * self.lengths[1],
                self.lengths[1] * self.lengths[2] * self.angles[0].cos(),
            ],
            [
                self.lengths[0] * self.lengths[2] * self.angles[1].cos(),
                self.lengths[1] * self.lengths[2] * self.angles[0].cos(),
                self.lengths[2] * self.lengths[2],
            ],
        ]
        .into()
    }
}

/// A struct representing a
pub struct BravaisCell<S, C>
where
    S: CrystalFamily,
    C: CenteringOf<S>,
{
    shape: PhantomData<S>,
    centering: PhantomData<C>,
}

trait CrystalFamily {
    const NAME: &'static str;
    const FULL_NAME: &'static str;
}

trait Centering {
    const NAME: &'static str;
}

trait CenteringOf<T: CrystalFamily> {}

make_family! {Triclinic, "t"}
make_family! {Monoclinic, "m"}
make_family! {Orthorhombic, "o"}
make_family! {Tetragonal, "t"}
make_family! {Hexagonal, "t"}
make_family! {Cubic, "c"}

make_centering!(P);
make_centering!(A);
make_centering!(B);
make_centering!(C);
make_centering!(F);
make_centering!(I);
make_centering!(R);

give_centering_opts! {Cubic, P, I, F}
