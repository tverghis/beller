//! These submodules are the "implementation" layer of the CLI commands.
//! Items in this module may combine several steps/requests from the `api` submodule
//! to make interaction with them more ergonomic.

mod api;

pub mod crypto;
pub mod did;
pub mod labeler;
pub mod plc;
pub mod repo;
pub mod session;
