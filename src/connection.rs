use nanomsg::{Socket, Protocol, Endpoint, Error as NanoError};

use std::borrow::Cow;
use std::io::{Error as IOError};
use std::fmt::{Display, Result as FormatResult, Formatter};

pub enum ConnectionStateError {
    Uninitialized(NanoError),
    Failed(NanoError),
    ReadFailure(IOError),
    WriteFailure(IOError),
    AddressReuse(IOError),
    Malformed
}

pub enum Transport {
    Inproc,
    Ipc,
    Tcp,
    Udp,
    Pgm
}

impl Display for Transport {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        let inner = match *self {
            Transport::Inproc => "inproc",
            Transport::Ipc => "ipc",
            Transport::Tcp => "tcp",
            Transport::Udp => "udp",
            Transport::Pgm => "pgm"
        };
        write!(f, "{}", inner)
    }
}

impl Display for ConnectionStateError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        let inner = match *self {
            ConnectionStateError::Uninitialized(ref nanoerr) => format!("{}", nanoerr),
            ConnectionStateError::Failed(ref nanoerr) => format!("{}", nanoerr),
            ConnectionStateError::ReadFailure(ref nanoerr) => format!("{}", nanoerr),
            ConnectionStateError::WriteFailure(ref nanoerr) => format!("{}", nanoerr),
            ConnectionStateError::AddressReuse(ref ioerr) => format!("{}", ioerr),
            ConnectionStateError::Malformed => format!("Malformed")
        };
        write!(f, "{}", inner)
    }
}

// TODO: Hide protocol to this module?
type SocketResult = Result<Socket, NanoError>; // TODO: rename?
type PortAvailableResult = Result<u16, IOError>;
pub type ConnectionResult<'a> = Result<Connection<'a>, ConnectionStateError>;

pub struct Connection<'a> {
    pub address: Cow<'a, str>,
    pub socket: Socket,
    pub endpoint: Endpoint
}

impl<'a> Connection<'a> {
    fn new(address: Cow<'a, str>, socket: Socket, endpoint: Endpoint) -> Connection<'a> {
        Connection {
            address: address,
            socket: socket,
            endpoint: endpoint
        }
    }

    pub fn from_connect_using_protocol(connect_address: &str, transport: Transport, protocol: Protocol) -> ConnectionResult<'a> {
        match Self::create_socket(protocol) {
            Ok(mut socket) => {
                let address = Cow::from(format!("{}://{}", transport, connect_address));
                match socket.connect(&address) {
                    Ok(endpoint) => Ok(Self::new(address.to_owned(), socket, endpoint)),
                    Err(err) => Err(ConnectionStateError::Failed(err))
                }
            },
            Err(err) => Err(ConnectionStateError::Uninitialized(err))
        }
    }

    pub fn from_bind_address(bind_address: &str, transport: Transport, protocol: Protocol) -> ConnectionResult<'a> {
        match Self::create_socket(protocol) {
            Ok(mut socket) => {
                match Self::find_free_port() {
                    Ok(port) => {
                        let address: Cow<str> = Cow::from(format!("{}://{}:{}", transport, bind_address, port));
                        match socket.bind(&address) {
                            Ok(endpoint) => Ok(Self::new(address.to_owned(), socket, endpoint)),
                            Err(err) => Err(ConnectionStateError::Failed(err))
                        }
                    },
                    Err(err) => Err(ConnectionStateError::AddressReuse(err))
                }
            },
            Err(err) => Err(ConnectionStateError::Uninitialized(err))
        }
    }

    pub fn from_bind(transport: Transport, protocol: Protocol) -> ConnectionResult<'a> {
        Self::from_bind_address("0.0.0.0", transport, protocol)
    }

    fn create_socket(protocol: Protocol) -> SocketResult {
        Ok(try!(Socket::new(protocol)))
    }

    fn find_free_port() -> PortAvailableResult {
        use std::net::TcpListener;
        
        let random_sock = try!(TcpListener::bind("0.0.0.0:0"));
        let local_addr = try!(random_sock.local_addr());
        Ok(local_addr.port())
    }
}
