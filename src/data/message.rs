use serde_json::{to_vec, from_str};
use serde_json::Error;

use std::borrow::Cow;
use std::collections::HashMap;

// HEALTH: source (heartbeat), target (matrix-1341441), command (PING), payload (1920302030 (timesamp)
// HEALTH: source (matrix-1341441), target (heartbeat), command (PONG), payload (1003919399 (timestamp)
// DISCOVERY: source (matrix-1341441), target (discovery), command (REGISTER), payload (matrix-1341441)
// DISCOVERY: source (discovery), target (matrix-1341441), command (SET), payload ({"health":"ip:port"}...address_table)
// 
// PROVIDER: source  

#[derive(Serialize, Deserialize)]
pub struct Header {
    source: String,
    target: String
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub command: String,
    pub payload: String
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub body: Body
}

type EncodeResult = Result<Vec<u8>, Error>;
type DecodeResult = Result<Message, Error>;

pub trait ToJson {
    fn encode(message: Self) -> EncodeResult
        where Self: Sized;
}

pub trait FromJson<'a> {
    fn decode(buffer: &Cow<'a, str>) -> DecodeResult;
}

impl ToJson for Message {
    fn encode(message: Self) -> EncodeResult {
        to_vec(&message)
    }
}

impl<'a> FromJson<'a> for Message {
    fn decode(buffer: &Cow<'a, str>) -> DecodeResult {
        from_str(&buffer)
    }
}

pub struct MessageBuilder {
    header: HashMap<String, String>,
    body: HashMap<String, String>
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            header: HashMap::new(),
            body: HashMap::new()
        }
    }
    
    /*
    fn named(&mut self, name: &str) -> MessageBuilder {
        self.header.insert(name,      
    }

    fn with_attribute(&mut self) -> MessageBuilder {
    
    }

    fn pack(&self) -> Message {
        // Message::new();   
    }*/
}
