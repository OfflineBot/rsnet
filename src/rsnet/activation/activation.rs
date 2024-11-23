
use ndarray;

#[allow(unused)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Custom(Box<dyn Fn(ndarray::Array2<f32>) -> ndarray::Array2<f32>>, Box<dyn Fn(ndarray::Array2<f32>) -> ndarray::Array2<f32>>),
    None,
}

