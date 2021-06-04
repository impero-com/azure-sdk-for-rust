#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-05")]
mod package_2018_05;
#[cfg(feature = "package-2018-05")]
pub use package_2018_05::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-03-preview")]
mod package_2018_03_preview;
#[cfg(feature = "package-2018-03-preview")]
pub use package_2018_03_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-10")]
mod package_2017_10;
#[cfg(feature = "package-2017-10")]
pub use package_2017_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-09")]
mod package_2017_09;
#[cfg(feature = "package-2017-09")]
pub use package_2017_09::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-04")]
mod package_2016_04;
#[cfg(feature = "package-2016-04")]
pub use package_2016_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2015-05-preview")]
mod package_2015_05_preview;
#[cfg(feature = "package-2015-05-preview")]
pub use package_2015_05_preview::{models, operations, API_VERSION};
#[cfg(feature = "profile-hybrid-2020-09-01")]
mod profile_hybrid_2020_09_01;
use azure_core::setters;
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub use profile_hybrid_2020_09_01::{models, operations, API_VERSION};
pub fn config(
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        api_version: None,
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    api_version: Option<String>,
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { api_version : String => Some (api_version) , base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            api_version: self.api_version.unwrap_or(API_VERSION.to_owned()),
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or("https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self.token_credential_resource.unwrap_or("https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    api_version: String,
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn api_version(&self) -> &str {
        self.api_version.as_str()
    }
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}
