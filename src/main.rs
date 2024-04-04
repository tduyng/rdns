use anyhow::{Error, Result};
use bytes::Bytes;
use dns_starter_rust::protocol::DnsPacket;
use std::net::UdpSocket;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let response_bytes: Bytes = DnsPacket::parse_response(Bytes::copy_from_slice(&buf[..size]));

                udp_socket
                    .send_to(&response_bytes, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                return Err(Error::from(e));
            }
        }
    }
}
