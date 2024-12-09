//! These submodules are thin wrappers around XRPC requests, and on failure, simply panic.
//! This makes them easier to work with at the highest "layer" of this application, since
//! all the error handling is performed internally.
//! The `crypto` submodule is the exception in that it involves no XRPC requests; all operations are
//! performed locally.

pub(super) mod crypto;
pub(super) mod did;
pub(super) mod plc;
pub(super) mod repo;
pub(super) mod session;
