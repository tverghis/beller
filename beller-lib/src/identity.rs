use atrium_api::com;
use serde::Serialize;

use crate::{XRPCResult, XRPC};

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
pub struct GetRecommendedDidCredentials {
    pub auth_token: String,
}

impl GetRecommendedDidCredentials {
    #[must_use]
    pub fn new(auth_token: String) -> Self {
        Self { auth_token }
    }
}

impl XRPC for GetRecommendedDidCredentials {
    const NSID: &'static str = com::atproto::identity::get_recommended_did_credentials::NSID;
    type Return = com::atproto::identity::get_recommended_did_credentials::OutputData;

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        ureq::get(&url)
            .set("Authorization", &format!("Bearer {}", self.auth_token))
            .call()?
            .into_json()
            .map_err(ureq::Error::from)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SignPlcOperation {
    pub auth_token: String,
    pub input: com::atproto::identity::sign_plc_operation::InputData,
}

impl SignPlcOperation {
    #[must_use]
    pub fn new(
        auth_token: String,
        input: com::atproto::identity::sign_plc_operation::InputData,
    ) -> Self {
        Self { auth_token, input }
    }
}

impl XRPC for SignPlcOperation {
    const NSID: &'static str = com::atproto::identity::sign_plc_operation::NSID;
    type Return = com::atproto::identity::sign_plc_operation::OutputData;

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        let input = com::atproto::identity::sign_plc_operation::Input::from(self.input.clone());

        ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.auth_token))
            .send_json(input)?
            .into_json()
            .map_err(ureq::Error::from)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitPlcOperation {
    pub auth_token: String,
    pub input: com::atproto::identity::submit_plc_operation::InputData,
}

impl SubmitPlcOperation {
    #[must_use]
    pub fn new(
        auth_token: String,
        input: com::atproto::identity::submit_plc_operation::InputData,
    ) -> Self {
        Self { auth_token, input }
    }
}

impl XRPC for SubmitPlcOperation {
    const NSID: &'static str = com::atproto::identity::submit_plc_operation::NSID;
    type Return = ();

    fn apply(&self, pds: &str) -> XRPCResult<Self::Return> {
        let url = self.url(pds);
        ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.auth_token))
            .send_json(&self.input)?;

        Ok(())
    }
}
