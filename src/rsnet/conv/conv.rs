

use ndarray;


#[allow(unused)]
pub struct ConvLayer {
    pub(crate) kernel:      ndarray::Array4<f32>,
    pub(crate) kernel_size: usize,
    pub(crate) in_channel:  usize,
    pub(crate) out_channel: usize,
    pub(crate) padding:     usize,
    pub(crate) stride:      usize,
}

