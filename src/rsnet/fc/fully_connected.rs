
use ndarray;

use crate::rsnet::activation::activation::Activation;


#[allow(unused)]
pub struct FullyConnected {
    pub(crate) weight:      ndarray::Array2<f32>,
    pub(crate) bias:        ndarray::Array1<f32>,
    pub(crate) activation:  Activation,
    pub(crate) activated:   Option<ndarray::Array2<f32>>,
    pub(crate) forwarded:   Option<ndarray::Array2<f32>>,
}

