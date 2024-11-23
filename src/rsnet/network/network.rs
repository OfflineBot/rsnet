
use crate::{
    ConvLayer,
    FullyConnected,
};

#[allow(unused)]
/// The main entry point for AI
/// Hierachy:
///  - Convolutional Layer
///  - Fully Connected Layer
pub struct Network {
    pub(crate) conv_layer:      Vec<ConvLayer>,
    pub(crate) fully_connected: Vec<FullyConnected>,
}

