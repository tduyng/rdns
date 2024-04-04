use anyhow::{Error, Result};
use bytes::Bytes;
use dns_starter_rust::protocol::{DnsHeader, DnsHeaderBuilder, DnsPacket, DnsQuestion};
use std::net::UdpSocket;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                // Try to create a DnsHeader from the first 12 bytes of the buffer.
                let buf = Bytes::copy_from_slice(&buf[..]);
                let _ = DnsHeader::try_from(buf.slice(0..12))?;

                let header_builder = DnsHeaderBuilder::new()
                    .packet_id(1234)
                    .query_response(1)
                    .question_count(1)
                    .build()?;

                let question = DnsQuestion::new("codecrafters.io", 1, 1);
                let response_bytes: Bytes = DnsPacket::new(header_builder, question).into();

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
