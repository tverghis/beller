//! These submodules invoke XRPC APIs, and on failure, exit the process with an error code.
//! This makes them easier to work with at the highest "layer" of this application, since
//! all the error handling is performed internally.

pub mod did;
pub mod plc;
pub mod repo;
pub mod session;
