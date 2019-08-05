use service::Service;
use connection::{Connection, Transport, ConnectionStateError, ConnectionResult};

use nanomsg::{Protocol, Error as NanoError};

pub struct Router;

type RouterResult = Result<Router, ConnectionStateError>;

impl Service for Router {
    fn process_incoming_message() {}
}

impl Router {
    pub fn new() -> RouterResult {
        let router_conn = try!(Self::init_pull_connection());
    }

    fn init_pull_connection() -> ConnectionResult {
        Ok(try!(Connection::bind(Transport::Tcp, Protocol::Pull)))
    }

    fn from_connection(conn: Connection) -> Router {
        Router {}
    }
}
