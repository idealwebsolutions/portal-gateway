use data::message::{Message, FromJson};
use connection::{Connection, Transport, ConnectionStateError, ConnectionResult};

use nanomsg::{Protocol, Error as NanoError};
use time::Duration;
use futures::stream::Stream;
use futures::{Poll, Async};
use crossbeam::scope;

use std::thread::sleep;
use std::borrow::Cow;
use std::io::{Read, Write, Error as IOError};

/// Provider: Provides two connections for read/write events
/// Additional socket is allocated for health
/// 1. Health: Connect
/// 2. Push: Connect
/// 3. Subscribe: Connect

/*struct EventCycle;

impl EventCycle {
    fn handle_message_event() {
    }

    fn handle_health_event() {
    }
}*/

struct Incoming {
    inner: Provider
}

struct Provider;/* {
    sub_conn: Connection,
    push_conn: Connection
}*/

type Address = String;
type AddressTuple = (Address, Address, Address);
type ProviderResult = Result<Provider, ConnectionStateError>;
type RegisterResult = Result<AddressTuple, ConnectionStateError>;

impl Stream for Incoming {
    type Item = ();
    type Error = IOError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, IOError> {
        Ok(Async::Ready(Some(try!(self.inner.process_incoming()))))
    }
}

impl Provider {
    // Default connect function to discovery service
    pub fn connect_all(discovery_address: &str) -> ProviderResult {
        /*let discover_conn = try!(Self::init_new_connection(&discovery_address, Protocol::Req));
        
        let addresses = try!(Self::register_pipeline(discover_conn));
        
        let push_address = addresses.0;
        let health_address = addresses.1;
        let publish_address = addresses.2;
        
        let sub_conn = try!(Self::init_new_connection(&publish_address, Protocol::Sub));
        let push_conn = try!(Self::init_new_connection(&push_address, Protocol::Push));*/
        let health_address = "";
        let health_conn = try!(Self::init_new_connection(&health_address, Transport::Tcp, Protocol::Respondent));
    
        Ok(Self::from_connections(health_conn))
    }

    pub fn incoming(self) -> Incoming {
        Incoming { inner: self }
    }

    pub fn disconnect(&self) {}
    
    fn process_incoming(&mut self) -> Result<(), IOError> {
        Ok(())
    }

    // Send a register message to service 
    /*fn register_pipeline(discovery_connection: Connection) -> RegisterResult {
        let mut socket = discovery_connection.socket;
        let mut endpoint = discovery_connection.endpoint;
        
        /*
        let register_frame = Frame {
            from: "".to_owned(),
            body: Message {
                target: "REGISTER".to_owned(),
                source: pull_address.to_owned(),
                payload: "".to_owned()
            }
        };*/
        
        // Ok(("".to_owned(), "".to_owned(), "".to_owned()))
        
        match Message::encode() {
            Ok(encoded) => {
                match socket.write_all(&encoded) {
                    Ok(_) => {
                        let mut reply = String::new();
                        match socket.read_to_string(&mut reply) {
                            Ok(_) => {
                                match Frame::decode(reply.into_bytes()) {
                                    Ok(decoded) => {
                                        let payload = decoded.body.payload;
                                        let _ = endpoint.shutdown();
                                        
                                        // payload.split("").collect(); 
                                        Ok(("".to_owned(), "".to_owned()))
                                    },
                                    Err(_) => Err(ConnectionStateError::Malformed)
                                }
                            },
                            Err(err) => Err(ConnectionStateError::ReadFailure(err))
                        }
                    },
                    Err(err) => Err(ConnectionStateError::WriteFailure(err))
                }
            },
            Err(_) => Err(ConnectionStateError::Malformed)
        }
    }*/

    fn init_new_connection(address: &str, transport: Transport, protocol: Protocol) -> ConnectionResult {
        Ok(try!(Connection::from_connect_using_protocol(&address, transport, protocol)))
    }

    // Creates a new provider
    fn from_connections(health_connection: Connection) -> Provider {
        let mut health_socket = health_connection.socket;
        let mut health_endpoint = health_connection.endpoint;
        
        let mut buffer: Cow<Vec<u8>> = Cow::Owned(Vec::<u8>::new());

        scope(|scope| {
            let scoped_join_handle = scope.spawn(|| -> Result<(), IOError> {
                loop {
                    match health_socket.nb_read_to_end(&mut buffer.to_mut()) {
                        Ok(_) => {
                            let encoded = String::from_utf8_lossy(&buffer);
                            if let Some(decoded) = Message::decode(&encoded).ok() {
                                // 
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

            let thread_result: Result<(), IOError> = scoped_join_handle.join();
    
            if thread_result.is_err() {
                let _ = health_endpoint.shutdown();
                info!("Endpoints have shutdown");
            }
        });

        Provider { }
    }
}
