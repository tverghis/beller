use atrium_api::com;
use serde::Serialize;

use crate::{XRPCResult, XRPC};

#[derive(Debug, Clone, Serialize)]
pub struct DescribeRepo {
    pub did: String,
}

impl DescribeRepo {
    #[must_use]
    pub fn new(did: String) -> Self {
        Self { did }
    }
}

impl XRPC for DescribeRepo {
    const NSID: &'static str = com::atproto::repo::describe_repo::NSID;
    type Return = com::atproto::repo::describe_repo::OutputData;

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        ureq::get(&url)
            .query("repo", &self.did)
            .call()?
            .into_json()
            .map_err(ureq::Error::from)
    }
}
