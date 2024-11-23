
use crate::{FlattenTo2D, Network};
use ndarray::{Array2, Array4};


impl Network {

    pub fn forward_all(&mut self, input_img: Array4<f32>) -> Array2<f32> {
        let out_img = self.forward_conv(input_img);
        self.forward_fc(out_img.flatten_to_2d())
    }

    pub fn forward_conv(&mut self, input_img: Array4<f32>) -> Array4<f32> {
        input_img
    }

    pub fn forward_fc(&mut self, input: Array2<f32>) -> Array2<f32> {
        input
    }
}
