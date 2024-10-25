// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![cfg_attr(test, deny(rust_2018_idioms))]
//! # pqkd
//!
//! pqkd is a client implementation in and for Rust for pQKD from
//! [QuantumBlockchains](https://www.quantumblockchains.io/pqkd/). 
//! This library is designed
//! to work with the pQKD device, which means that it makes
//! no sense to use the code without this device.  
//! 
//! ## Features
//! 
//! pqkd allows you to send keys to other pQKD devices,
//! receive them and also receive random values from the
//! pQKD device in hex, bytes, base64 format. 
pub use crate::pqkd::BuilderPqkdClient;
pub use crate::pqkd::PqkdClient;
pub use crate::pqkd::PqkdStatus;
pub use crate::request_pqkd::PqkdRequestBuilder;
pub use crate::request_pqkd::PqkdResponse;

pub mod qrng;
pub mod error;
pub mod pqkd;
pub mod request_pqkd;
