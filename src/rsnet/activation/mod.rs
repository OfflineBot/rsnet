
pub mod activation;

mod relu;
mod sigmoid;

#[allow(unused)]
pub mod functions {
    pub use super::relu::*;
    pub use super::sigmoid::*;
}
