use std::net::{TcpStream};
use std::io::{Read, Write, Error};
use std::str::from_utf8;


pub struct Client {
    stream: TcpStream,
}

impl Client {
    fn new(ip: &str) -> Result<Self, Error> {
        let mut stream = TcpStream::connect(ip)?;

        Ok(Self {
            stream,
        })
    }

    pub fn send_data(&mut self, msg: &str) -> Result<(), Error> {
        self.stream.write(msg.as_bytes())?;
        Ok(())
    }
}

impl Default for Client{
    fn default() -> Self { Client::new("localhost:1111").expect("Failed to create default Client") }
}