use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Hello,
}

#[derive(Serialize, Deserialize)]
pub struct SqsPayload<'a> {
    pub msg_type: MessageType,
    pub msg: &'a str,
    pub request_id: Uuid,
    pub service_id: Uuid,
}
