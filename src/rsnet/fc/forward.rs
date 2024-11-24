
use ndarray::Array2;

use super::fully_connected::FullyConnected;
use crate::Activation as Act;

impl FullyConnected {

    // TODO! Activations
    pub fn forward(&mut self, x: Array2<f32>) -> Array2<f32> {
        let z1 = x.dot(&self.weight) + &self.bias;
        self.forwarded = Some(z1.clone());
        println!("NOT DONE YET!! Activations missin!");
        match self.activation {
            Act::ReLU => {
                z1
            },
            Act::Sigmoid => {
                z1
            },
            Act::None => {
                z1
            },
            Act::Custom(_, _) => {
                z1
            }
        }
    }
}
