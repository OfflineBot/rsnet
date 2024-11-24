
use ndarray::{Array2, Array4, Axis};

pub trait FlattenTo2D {
    fn flatten_to_2d(&self) -> Array2<f32>;
}

impl FlattenTo2D for Array4<f32> {
    fn flatten_to_2d(&self) -> Array2<f32> {
        let samples = self.shape()[0];
        let size = self.shape()[1] * self.shape()[2] * self.shape()[3];

        let mut output = Array2::zeros((samples, size));
        for i in 0..samples {
            let flattened = self.index_axis(Axis(0), i).flatten().iter().cloned().collect::<Vec<f32>>();
            output.row_mut(i).assign(&Array2::from_shape_vec((1, size), flattened).unwrap().row(0));
        }
        output
    }
}
