#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub mod endpoints {
    use crate::models::*;
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        endpoint_type: &str,
        endpoint_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<Endpoint, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name,
            endpoint_type,
            endpoint_name
        );
        let mut url = url::Url::parse(url_str).map_err(get::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(get::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Endpoint =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(get::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        endpoint_type: &str,
        endpoint_name: &str,
        parameters: &Endpoint,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name,
            endpoint_type,
            endpoint_name
        );
        let mut url = url::Url::parse(url_str).map_err(create_or_update::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(create_or_update::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).map_err(create_or_update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create_or_update::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(create_or_update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Endpoint = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: Endpoint = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(create_or_update::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(Endpoint),
            Created201(Endpoint),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        endpoint_type: &str,
        endpoint_name: &str,
        parameters: &Endpoint,
        subscription_id: &str,
    ) -> std::result::Result<Endpoint, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name,
            endpoint_type,
            endpoint_name
        );
        let mut url = url::Url::parse(url_str).map_err(update::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(update::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).map_err(update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(update::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Endpoint =
                    serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(update::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        endpoint_type: &str,
        endpoint_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name,
            endpoint_type,
            endpoint_name
        );
        let mut url = url::Url::parse(url_str).map_err(delete::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(delete::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(delete::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(delete::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                Err(delete::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}
pub mod profiles {
    use crate::models::*;
    pub async fn check_traffic_manager_relative_dns_name_availability(
        operation_config: &crate::OperationConfig,
        parameters: &CheckTrafficManagerRelativeDnsNameAvailabilityParameters,
    ) -> std::result::Result<TrafficManagerNameAvailability, check_traffic_manager_relative_dns_name_availability::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Network/checkTrafficManagerNameAvailability",
            operation_config.base_path(),
        );
        let mut url = url::Url::parse(url_str).map_err(check_traffic_manager_relative_dns_name_availability::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(check_traffic_manager_relative_dns_name_availability::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body =
            azure_core::to_json(parameters).map_err(check_traffic_manager_relative_dns_name_availability::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(check_traffic_manager_relative_dns_name_availability::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(check_traffic_manager_relative_dns_name_availability::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: TrafficManagerNameAvailability = serde_json::from_slice(rsp_body).map_err(|source| {
                    check_traffic_manager_relative_dns_name_availability::Error::DeserializeError(source, rsp_body.clone())
                })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(check_traffic_manager_relative_dns_name_availability::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod check_traffic_manager_relative_dns_name_availability {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn list_all_in_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<ProfileListResult, list_all_in_resource_group::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
        );
        let mut url = url::Url::parse(url_str).map_err(list_all_in_resource_group::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_all_in_resource_group::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(list_all_in_resource_group::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_all_in_resource_group::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ProfileListResult = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_all_in_resource_group::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(list_all_in_resource_group::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod list_all_in_resource_group {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn list_all(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<ProfileListResult, list_all::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Network/trafficmanagerprofiles",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(list_all::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_all::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list_all::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_all::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ProfileListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list_all::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(list_all::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod list_all {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<Profile, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name
        );
        let mut url = url::Url::parse(url_str).map_err(get::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(get::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Profile =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(get::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        parameters: &Profile,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name
        );
        let mut url = url::Url::parse(url_str).map_err(create_or_update::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(create_or_update::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).map_err(create_or_update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create_or_update::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(create_or_update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Profile = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: Profile = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(create_or_update::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(Profile),
            Created201(Profile),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        parameters: &Profile,
        subscription_id: &str,
    ) -> std::result::Result<Profile, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name
        );
        let mut url = url::Url::parse(url_str).map_err(update::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(update::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).map_err(update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(update::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Profile =
                    serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(update::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        profile_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            profile_name
        );
        let mut url = url::Url::parse(url_str).map_err(delete::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(delete::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(delete::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(delete::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                Err(delete::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}
