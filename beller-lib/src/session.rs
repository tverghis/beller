use atrium_api::com;
use serde::Serialize;

use crate::{XRPCResult, XRPC};

#[derive(Debug, Clone, Serialize)]
pub struct Create {
    pub identifier: String,
    pub password: String,
}

impl Create {
    #[must_use]
    pub fn new(identifier: String, password: String) -> Self {
        Self {
            identifier,
            password,
        }
    }
}

impl XRPC for Create {
    const NSID: &'static str = com::atproto::server::create_session::NSID;
    type Return = com::atproto::server::create_session::OutputData;

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        ureq::post(&url)
            .send_json(self)?
            .into_json()
            .map_err(ureq::Error::from)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Get {
    pub auth_token: String,
}

impl Get {
    #[must_use]
    pub fn new(auth_token: String) -> Self {
        Self { auth_token }
    }
}

impl XRPC for Get {
    const NSID: &'static str = com::atproto::server::get_session::NSID;
    type Return = com::atproto::server::get_session::OutputData;

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        ureq::get(&url)
            .set("Authorization", &format!("Bearer {}", self.auth_token))
            .call()?
            .into_json()
            .map_err(ureq::Error::from)
    }
}
