use crate::queue::PopReceipt;
use azure_core::errors::AzureError;
use azure_core::headers::{utc_date_from_rfc2822, CommonStorageResponseHeaders};
use azure_core::util::to_str_without_bom;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub pop_receipt: PopReceipt,
    pub insertion_time: DateTime<Utc>,
    pub expiration_time: DateTime<Utc>,
    pub time_next_visible: DateTime<Utc>,
    pub dequeue_count: u64,
    pub message_text: String,
}

impl From<Message> for PopReceipt {
    fn from(message: Message) -> Self {
        message.pop_receipt
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageInternal {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "PopReceipt")]
    pub pop_receipt: String,
    #[serde(rename = "TimeNextVisible")]
    pub time_next_visible: String,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: u64,
    #[serde(rename = "MessageText")]
    pub message_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessagesInternal {
    #[serde(rename = "QueueMessage")]
    pub messages: Option<Vec<MessageInternal>>,
}

impl std::convert::TryFrom<&Response<Bytes>> for GetMessagesResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = to_str_without_bom(response.body())?;

        debug!("headers == {:?}", headers);

        let response: MessagesInternal = serde_xml_rs::from_str(body)?;
        debug!("response == {:?}", response);

        let mut messages = Vec::new();
        for message in response.messages.unwrap_or_default().into_iter() {
            messages.push(Message {
                pop_receipt: PopReceipt::new(message.message_id, message.pop_receipt),
                insertion_time: utc_date_from_rfc2822(&message.insertion_time)?,
                expiration_time: utc_date_from_rfc2822(&message.expiration_time)?,
                time_next_visible: utc_date_from_rfc2822(&message.time_next_visible)?,
                dequeue_count: message.dequeue_count,
                message_text: message.message_text,
            })
        }

        Ok(GetMessagesResponse {
            common_storage_response_headers: headers.try_into()?,
            messages,
        })
    }
}
