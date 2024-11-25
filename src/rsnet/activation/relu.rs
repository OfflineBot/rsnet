
use ndarray::Array2;

#[allow(unused)]
pub fn relu(x: Array2<f32>) -> Array2<f32> {
    x.mapv(|f| if f > 0.0 { f } else { 0.0 })
}


#[allow(unused)]
pub fn deriv_relu(x: Array2<f32>) -> Array2<f32> {
    x.mapv(|f| if f > 0.0 { 1.0 } else { 0.0 })
}

