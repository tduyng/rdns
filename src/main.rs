use anyhow::{Error, Result};
use bytes::Bytes;
use dns_starter_rust::protocol::{DnsHeaderBuilder, DnsPacket, DnsQuestion, DnsRecord};
use std::net::UdpSocket;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut buf = Bytes::copy_from_slice(&buf[..size]);
                let request = DnsPacket::from(&mut buf);

                let request_header = request.header();
                let op_code = request_header.op_code();
                let response_code = if op_code == 0 { 0 } else { 4 };

                let header = DnsHeaderBuilder::new()
                    .packet_id(request_header.packet_id())
                    .query_response(1)
                    .op_code(op_code)
                    .desired_recursion(request_header.desired_recursion())
                    .response_code(response_code)
                    .question_count(1)
                    .answer_count(1)
                    .build()?;
                let question_name = request.questions().first().unwrap().name.clone();
                let question = DnsQuestion {
                    name: question_name,
                    record_type: 1,
                    class: 1,
                };
                let answer = DnsRecord::new(DnsRecord {
                    name: request.questions().first().unwrap().name.clone(),
                    record_type: 1,
                    class: 1,
                    ttl: 60,
                    length: 4,
                    data: vec![0x8, 0x8, 0x8, 0x8],
                });

                let dns_packet = DnsPacket::new(DnsPacket {
                    header,
                    questions: vec![question],
                    answers: vec![answer],
                });

                let response_bytes: Bytes = (&dns_packet).into();

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
