
use super::conv::ConvLayer;

impl ConvLayer {
    pub fn new(kernel_size: usize, in_channel: usize, out_channel: usize, padding: usize, stride: usize) -> Self {
        ConvLayer {
            kernel: ndarray::Array4::zeros((out_channel, in_channel, kernel_size, kernel_size)),
            in_channel,
            out_channel,
            kernel_size,
            padding, 
            stride,
        }
    }
}
