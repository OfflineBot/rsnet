
use ndarray::{Array2, Array4};

pub trait FlattenTo2D {
    fn flatten_to_2d(&self) -> Array2<f32>;
}

impl FlattenTo2D for Array4<f32> {
    fn flatten_to_2d(&self) -> Array2<f32> {
        println!("hello. you should implement this now :)");
        Array2::zeros((1, 1))
    }
}
