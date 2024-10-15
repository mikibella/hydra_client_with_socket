use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    jwk_api: Box<dyn crate::apis::JwkApi>,
    metadata_api: Box<dyn crate::apis::MetadataApi>,
    o_auth2_api: Box<dyn crate::apis::OAuth2Api>,
    oidc_api: Box<dyn crate::apis::OidcApi>,
    wellknown_api: Box<dyn crate::apis::WellknownApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            jwk_api: Box::new(crate::apis::JwkApiClient::new(rc.clone())),
            metadata_api: Box::new(crate::apis::MetadataApiClient::new(rc.clone())),
            o_auth2_api: Box::new(crate::apis::OAuth2ApiClient::new(rc.clone())),
            oidc_api: Box::new(crate::apis::OidcApiClient::new(rc.clone())),
            wellknown_api: Box::new(crate::apis::WellknownApiClient::new(rc.clone())),
        }
    }

    pub fn jwk_api(&self) -> &dyn crate::apis::JwkApi{
        self.jwk_api.as_ref()
    }

    pub fn metadata_api(&self) -> &dyn crate::apis::MetadataApi{
        self.metadata_api.as_ref()
    }

    pub fn o_auth2_api(&self) -> &dyn crate::apis::OAuth2Api{
        self.o_auth2_api.as_ref()
    }

    pub fn oidc_api(&self) -> &dyn crate::apis::OidcApi{
        self.oidc_api.as_ref()
    }

    pub fn wellknown_api(&self) -> &dyn crate::apis::WellknownApi{
        self.wellknown_api.as_ref()
    }

}
