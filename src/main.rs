use anyhow::{Error, Result};
use bytes::Bytes;
use clap::Parser;
use dns_starter_rust::protocol::DnsPacket;
use std::net::UdpSocket;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(short, long, help = "Ip address <ip>:<port>")]
    resolver: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let ip_address = cli.resolver;

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let buf = &buf[..size].to_owned();
                let response_bytes: Bytes = DnsPacket::to_response(buf, ip_address.clone());

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
