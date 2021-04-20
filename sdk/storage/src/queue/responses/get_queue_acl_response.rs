use crate::QueueStoredAccessPolicy;
use azure_core::errors::AzureError;
use azure_core::errors::PermissionError;
use azure_core::headers::CommonStorageResponseHeaders;
use azure_core::prelude::*;
use azure_core::util::to_str_without_bom;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub stored_access_policies: Vec<QueueStoredAccessPolicy>,
}

impl std::convert::TryFrom<&Response<Bytes>> for GetQueueACLResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = to_str_without_bom(response.body())?;

        debug!("headers == {:?}", headers);

        let a: Result<Vec<QueueStoredAccessPolicy>, PermissionError> =
            StoredAccessPolicyList::from_xml(body)?
                .stored_access
                .into_iter()
                .map(|sap| sap.try_into())
                .collect();

        Ok(GetQueueACLResponse {
            common_storage_response_headers: headers.try_into()?,
            stored_access_policies: a?,
        })
    }
}
