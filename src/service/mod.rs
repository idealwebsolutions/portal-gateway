// use data::message::Message;
use connection::Transport;

use std::io::ErrorKind;

pub type Callback = fn();

pub struct ServiceOptions {
    pub transport: Option<Transport>,
    pub bind_address: Option<String>
}

pub trait Service { 
    fn cycle(&mut self) -> Result<String, ErrorKind>;
    // fn handle_incoming_message<T>(message: T); 
}

// mod discovery;
pub mod health;
