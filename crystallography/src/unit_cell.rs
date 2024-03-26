use nalgebra::Matrix3;

/// a struct representing the crystal parameters
pub struct UnitCellParameters {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
}

impl UnitCellParameters {
    /// this function returns the metric tensor
    pub fn get_metric_tensor(&self) -> Matrix3<f32> {
        [
            [
                self.a * self.a,
                self.a * self.b * self.gamma.cos(),
                self.a * self.c * self.beta.cos(),
            ],
            [
                self.a * self.b * self.gamma.cos(),
                self.b * self.b,
                self.b * self.c * self.alpha.cos(),
            ],
            [
                self.a * self.c * self.beta.cos(),
                self.b * self.c * self.alpha.cos(),
                self.c * self.c,
            ],
        ]
        .into()
    }
}
