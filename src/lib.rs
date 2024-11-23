
mod rsnet;


pub use rsnet::{
    network::network::Network,
    fc::fully_connected::FullyConnected,
    conv::conv::ConvLayer,
    ndarray_trait::flatten_to_twod::FlattenTo2D,
    activation::activation::Activation,
};

