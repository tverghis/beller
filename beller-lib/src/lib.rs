use atrium_api::com;
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

#[derive(Debug, Clone, Serialize)]
pub struct RequestPlcOperationSignature {
    pub auth_token: String,
}

impl RequestPlcOperationSignature {
    #[must_use]
    pub fn new(auth_token: String) -> Self {
        Self { auth_token }
    }
}

impl XRPC for RequestPlcOperationSignature {
    const NSID: &'static str = com::atproto::identity::request_plc_operation_signature::NSID;
    type Return = ();

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.auth_token))
            .call()
            .map(|_| ())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSession {
    pub identifier: String,
    pub password: String,
}

impl CreateSession {
    #[must_use]
    pub fn new(identifier: String, password: String) -> Self {
        Self {
            identifier,
            password,
        }
    }
}

impl XRPC for CreateSession {
    const NSID: &'static str = com::atproto::server::create_session::NSID;
    type Return = atrium_api::com::atproto::server::create_session::OutputData;

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        ureq::post(&url)
            .send_json(self)?
            .into_json()
            .map_err(ureq::Error::from)
    }
}
