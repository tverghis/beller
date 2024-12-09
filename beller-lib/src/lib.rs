pub mod identity;
pub mod repo;
pub mod session;

use serde::{de::DeserializeOwned, Serialize};

pub type XRPCResult<T> = Result<T, ureq::Error>;

/// Represents an XRPC operation.
pub trait XRPC: Serialize {
    /// The NSID for the XRPC operation.
    ///
    /// See [the ATProto docs](https://atproto.com/specs/nsid) for more details.
    const NSID: &'static str;
    /// The type of the result of the XRPC operation.
    type Return: DeserializeOwned;

    /// Returns the URL for the XRPC endpoint.
    fn url(&self, pds: &str) -> String {
        format!("{}/xrpc/{}", pds, Self::NSID)
    }

    /// Performs the requested operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails. Inspect the returned `ureq::Error` for more details.
    #[allow(clippy::result_large_err)]
    fn apply(&self, pds: &str) -> XRPCResult<Self::Return>;
}
