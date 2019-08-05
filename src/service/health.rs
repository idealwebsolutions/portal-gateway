use service::{Service, ServiceOptions};
use data::command::Command;
use data::message::{Message, MessageBuilder, FromJson};
use connection::{Connection, Transport, ConnectionStateError};

use nanomsg::{Protocol, Error as NanoError};
use time::{SteadyTime, Duration};
use futures::stream::Stream;
use futures::{Future, Poll, Async};

use std::borrow::Cow;
// use std::time::Duration as StdDuration;
// use std::thread::sleep;
use std::io::ErrorKind;

const HEALTH_TAG: &'static str = "HEALTH";

pub struct HealthStateStream {

}

pub struct Incoming<'a> {
    inner: Health<'a>
}

pub struct Health<'a> {
    conn: Connection<'a>,
    last: SteadyTime
}

type HealthResult<'a> = Result<Health<'a>, ConnectionStateError>;

const PING_INTERVAL: isize = 30; // 30 seconds

// impl<'a> Hooks for Health<'a> {}

impl<'a> Stream for Incoming<'a> {
    type Item = String;
    type Error = ErrorKind;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(Some(try!(self.inner.cycle()))))
    }
}

impl<'a> Service for Health<'a> {
    /*fn handle_incoming_message(message: Message) {
        match message.command {
            Command::Pong(ref timestamp) => {

            }
        }
    }*/

    fn cycle(&mut self) -> Result<String, ErrorKind> {
        let ref mut socket = self.conn.socket;
        let ref mut endpoint = self.conn.endpoint;

        let mut buffer = Cow::from(Vec::<u8>::new());
        
        loop {
            let now = SteadyTime::now();

            if (now - self.last) > Duration::seconds(PING_INTERVAL as i64) {
                self.last = now;

                let bytes = Cow::from(format!("PING {}", now).into_bytes());

                match socket.nb_write(&bytes) {
                    Ok(_) => {
                        debug!("{}: Successful write dispatch", HEALTH_TAG);
                        loop {
                            match socket.nb_read_to_end(&mut buffer.to_mut()) {
                                Ok(_) => {

                                    debug!("{}: Read success", HEALTH_TAG);
                                    let cloned = buffer.into_owned();
                                    let bytes = String::from_utf8_lossy(&cloned);
                                    match Message::decode(&bytes) {
                                        Ok(decoded) => {
                                            debug!("{}: Message successfully decoded", HEALTH_TAG);
                                            // Self::handle_incoming_message(decoded)
                                            return Ok(decoded.body.command)
                                        },
                                        Err(err) => {
                                            error!("{}: Invalid data {}", HEALTH_TAG, err);
                                            return Err(ErrorKind::InvalidData)
                                        }
                                    }
                                },
                                Err(NanoError::TryAgain) => {
                                    error!("{}: Resource read busy: Try again error", HEALTH_TAG);
                                    return Err(ErrorKind::WouldBlock)
                                },
                                Err(err) => {
                                    error!("{}: {}", HEALTH_TAG, err);
                                    return Err(ErrorKind::TimedOut)
                                }
                            }           
                            buffer.to_mut().clear();
                        }
                    },
                    Err(NanoError::TryAgain) => {
                        debug!("{}: Resource write busy: Try again later", HEALTH_TAG);
                        return Err(ErrorKind::WouldBlock)
                    },
                    Err(err) => {
                        error!("{}: {}", HEALTH_TAG, err);
                        break;
                    }
                }
            }
        }

        let _ = endpoint.shutdown();
        error!("{}: endpoint has unexpectedly shutdown", HEALTH_TAG);

        return Err(ErrorKind::UnexpectedEof)
    }
}

impl<'a> Health<'a> {
    pub fn with_options(options: ServiceOptions) -> HealthResult<'a> {
        let transport = options.transport.unwrap_or(Transport::Tcp);
        let mut connection;

        if let Some(address) = options.bind_address {
            connection = try!(Connection::from_bind_address(&address, transport, Protocol::Surveyor));
        } else {
            connection = try!(Connection::from_bind(transport, Protocol::Surveyor));
        }

        let deadline: isize = (PING_INTERVAL * 1000) / 2;
        
        if let Err(e) = connection.socket.set_survey_deadline(deadline) {
            return Err(ConnectionStateError::Failed(e))
        }

        Ok(Health {
            conn: connection,
            last: SteadyTime::now()
        })
    }

    pub fn take(self) -> Incoming<'a> {
        Incoming { 
            inner: self 
        }
    }

    pub fn new() -> HealthResult<'a> {
        Ok(Health { 
            conn: try!(Connection::from_bind(Transport::Tcp, Protocol::Surveyor)),
            last: SteadyTime::now()
        })
    }
}

impl Future for HealthStateStream {
}

impl HealthStateStream {
}
