
use super::network::Network;

impl Network {
    pub fn new() -> Self {
        Network {
            conv_layer: Vec::new(),
            fully_connected: Vec::new(),
        }
    }
}

