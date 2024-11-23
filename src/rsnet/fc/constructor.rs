
use crate::rsnet::activation::activation::Activation;

use super::fully_connected::FullyConnected;

impl FullyConnected {
    pub fn new(input_layer_size: usize, output_layer_size: usize, activation: Activation) -> Self {
        FullyConnected {
            weight: ndarray::Array2::zeros((input_layer_size, output_layer_size)),
            bias:   ndarray::Array1::zeros(output_layer_size),
            activation,
            activated: None,
            forwarded: None,
        }
    }
}
