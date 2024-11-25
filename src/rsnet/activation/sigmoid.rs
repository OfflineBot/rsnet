
use ndarray::Array2;


#[allow(unused)]
pub fn sigmoid(x: Array2<f32>) -> Array2<f32> {
    x.mapv(|f| 1.0 / (1.0+(-f).exp()) )
}

#[allow(unused)]
pub fn deriv_sigmoid(x: Array2<f32>) -> Array2<f32> {
    let sig = sigmoid(x);
    &sig * (1.0 - &sig)
}

