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

fn encode_dns_name(domain: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    
    for part in domain.split('.') {
        encoded.push(part.len() as u8);
        encoded.extend(part.bytes());
    }
    encoded.push(0);
    
    encoded
}

fn header_to_bytes(header: &DNSHeader) -> Vec<u8> {
    let mut bytes = Vec::new();
    
    bytes.extend(&header.id.to_be_bytes());
    bytes.extend(&header.flags.to_be_bytes());
    bytes.extend(&header.num_questions.to_be_bytes());
    bytes.extend(&header.num_answers.to_be_bytes());
    bytes.extend(&header.num_authorities.to_be_bytes());
    bytes.extend(&header.num_additionals.to_be_bytes());
    
    bytes
}

fn question_to_bytes(question: &DNSQuestion) -> Vec<u8> {
    let mut bytes = Vec::new();
    
    bytes.extend(&question.name);
    bytes.extend(&question.type_.to_be_bytes());
    bytes.extend(&question.class.to_be_bytes());
    
    bytes
}

fn build_query(domain: &str, record_type: u16) -> Vec<u8> {
    let id = rand::random::<u16>();
    
    let header = DNSHeader {
        id,
        flags: 0x0100, // Standard query with recursion desired
        num_questions: 1,
        num_answers: 0,
        num_authorities: 0,
        num_additionals: 0,
    };
    
    let question = DNSQuestion {
        name: encode_dns_name(domain),
        type_: record_type,
        class: 1, // IN (Internet)
    };
    
    let mut query = Vec::new();
    query.extend(header_to_bytes(&header));
    query.extend(question_to_bytes(&question));
    
    query
}

fn main() {
    println!("DNS Resolver starting...");
    
    let domain = "example.com";
    let query = build_query(domain, 1); // Type A record
    println!("Built DNS query of {} bytes", query.len());
}