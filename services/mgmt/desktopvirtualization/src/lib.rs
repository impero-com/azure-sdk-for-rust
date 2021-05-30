#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2019-01-23-preview")]
mod package_2019_01_23_preview;
#[cfg(feature = "package-2019-01-23-preview")]
pub use package_2019_01_23_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-09-24-preview")]
mod package_2019_09_24_preview;
#[cfg(feature = "package-2019-09-24-preview")]
pub use package_2019_09_24_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-12-10-preview")]
mod package_2019_12_10_preview;
#[cfg(feature = "package-2019-12-10-preview")]
pub use package_2019_12_10_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-09-21-preview")]
mod package_2020_09_21_preview;
#[cfg(feature = "package-2020-09-21-preview")]
pub use package_2020_09_21_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-10-19-preview")]
mod package_2020_10_19_preview;
#[cfg(feature = "package-2020-10-19-preview")]
pub use package_2020_10_19_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-11-02-preview")]
mod package_2020_11_02_preview;
#[cfg(feature = "package-2020-11-02-preview")]
pub use package_2020_11_02_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-11-10-preview")]
mod package_2020_11_10_preview;
#[cfg(feature = "package-2020-11-10-preview")]
pub use package_2020_11_10_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-01-14-preview")]
mod package_2021_01_14_preview;
#[cfg(feature = "package-2021-01-14-preview")]
pub use package_2021_01_14_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-02-01-preview")]
mod package_2021_02_01_preview;
#[cfg(feature = "package-2021-02-01-preview")]
pub use package_2021_02_01_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-03-09-preview")]
mod package_2021_03_09_preview;
#[cfg(feature = "package-2021-03-09-preview")]
pub use package_2021_03_09_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2021-04-01-preview")]
mod package_2021_04_01_preview;
use azure_core::setters;
#[cfg(feature = "package-2021-04-01-preview")]
pub use package_2021_04_01_preview::{models, operations, API_VERSION};
pub fn config(
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
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
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
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
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn api_version(&self) -> &str {
        self.api_version.as_str()
    }
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref().as_ref()
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
