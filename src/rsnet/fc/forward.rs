
use ndarray::Array2;

use super::fully_connected::FullyConnected;
use crate::Activation as Act;
use crate::functions::{sigmoid, relu};

impl FullyConnected {

    pub fn forward(&mut self, x: Array2<f32>) -> Array2<f32> {
        let z1 = x.dot(&self.weight) + &self.bias;
        self.forwarded = Some(z1.clone());
        match self.activation {
            Act::ReLU => {
                let a1 = relu(z1);
                self.activated = Some(a1.clone());
                a1
            },
            Act::Sigmoid => {
                let a1 = sigmoid(z1);
                self.activated = Some(a1.clone());
                a1
            },
            Act::None => {
                self.activated = None;
                z1
            },
            Act::Custom(ref activ, _) => {
                let a1 = activ(z1);
                self.activated = Some(a1.clone());
                a1
            },
        }
    }

}
