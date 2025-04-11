use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Hello,
    Url,
}

#[derive(Serialize, Deserialize)]
pub struct SqsPayload<'a> {
    pub msg_type: MessageType,
    pub msg: &'a str,
    pub request_id: Uuid,
    pub service_id: Uuid,
}

impl SqsPayload<'_> {
    pub fn mock() -> SqsPayload<'static> {
        let msg = "Foobar";
        SqsPayload {
            msg_type: MessageType::Hello,
            msg: &msg,
            request_id: Uuid::new_v4(),
            service_id: Uuid::new_v4(),
        }
    }
}
