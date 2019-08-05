use data::message::{Message, FromJson};
use connection::{Connection, Transport, ConnectionStateError, ConnectionResult};
use service::{Service, Callback};

use nanomsg::{Protocol, Error as NanoError};
use time::{PreciseTime, Duration};
use crossbeam::scope;

use std::thread::sleep;
use std::borrow::Cow;
use std::io::Error as IOError;

struct Registry;

impl Registry {
    pub fn new() -> Registry {
        Registry {}
    }

    fn on_register_new_entry(&self, c: Callback) {
        (c)()
    }

    fn on_unregister_existing_entry(&self, c: Callback) {
        (c)()
    }
}

pub struct Discovery;

type DiscoveryResult = Result<Discovery, ConnectionStateError>;

impl Service for Discovery {
    fn listen(&mut self) {}
    fn handle_incoming_message() {}
}

impl Discovery {
    pub fn new() -> DiscoveryResult {
        let discovery_conn = try!(Self::init_reply_connection());

        Ok(Self::from_connection(discovery_conn))
    }

    fn init_reply_connection() -> ConnectionResult {
        Ok(try!(Connection::from_bind(Transport::Tcp, Protocol::Rep)))
    }

    fn from_connection(discovery_conn: Connection) -> Discovery {
        let mut discovery_socket = discovery_conn.socket;
        let mut discovery_endpoint = discovery_conn.endpoint;

        let mut buffer: Cow<Vec<u8>> = Cow::Owned(Vec::<u8>::new()); 
        
        scope(|scope| {        
            let scoped_join_handle = scope.spawn(|| -> Result<(), IOError> {
                loop {
                    match discovery_socket.nb_read_to_end(&mut buffer.to_mut()) {
                        Ok(_) => {
                            let encoded = String::from_utf8_lossy(&buffer);
                            if let Some(decoded) = Message::decode(&encoded).ok() {
                                // Self::handle_incoming_message()
                            }
                        },
                        Err(NanoError::TryAgain) => warn!("Resource busy read error"),
                        Err(err) => panic!("Critical error while reading: {}", err)
                    }
                    
                    buffer.to_mut().clear();
                    
                    if let Some(duration) = Duration::milliseconds(10).to_std().ok() {
                        sleep(duration);
                    }
                }
            });

            let result: Result<(), IOError> = scoped_join_handle.join();

            if result.is_err() {
                let _ = discovery_endpoint.shutdown();
            }
        });
        
        Discovery { }
    }
}
