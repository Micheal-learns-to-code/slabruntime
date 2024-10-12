//! This crate provides a mechanism to work with CP210x USB to UART bridge devices at runtime.
pub use crate::{
    cp210x_runtime::Cp210xRuntime,
    errors::{CP210xError, Result},
};

mod constants;
mod cp210x_runtime;
mod errors;
