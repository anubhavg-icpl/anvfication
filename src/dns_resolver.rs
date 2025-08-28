use std::net::{UdpSocket, SocketAddr};
use std::io;

const DNS_PORT: u16 = 53;
const ROOT_NAME_SERVER: &str = "198.41.0.4";

#[derive(Debug, Clone)]
struct DNSHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

#[derive(Debug, Clone)]
struct DNSQuestion {
    name: Vec<u8>,
    type_: u16,
    class: u16,
}

fn main() {
    println!("DNS Resolver starting...");
}