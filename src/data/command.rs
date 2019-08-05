pub enum Command {
    Join(String),
    Part(String),
    Ping(String),
    Pong(String)
}
