
use crate::Network;
use crate::{
    rsnet::activation::activation::Activation, ConvLayer, FullyConnected
};

impl Network {

    pub fn add_fc(&mut self, input_layer_size: usize, output_layer_size: usize, activation: Activation) {
        self.fully_connected.push(
            FullyConnected::new(input_layer_size, output_layer_size, activation)
        );
    }


    pub fn add_conv(&mut self, kernel_size: usize, out_channel: usize, in_channel: usize, padding: usize, stride: usize) {
        self.conv_layer.push(
            ConvLayer::new(kernel_size, in_channel, out_channel, padding, stride)
        );
    }

}
