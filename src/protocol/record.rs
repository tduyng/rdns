use std::net::UdpSocket;

use super::{DnsHeader, DnsPacket, DnsQuestion};
use crate::utils::encode;
use bytes::{BufMut, Bytes, BytesMut};
use nom::{
    bytes::complete::take as take_bytes,
    multi,
    number::complete::{be_u16, be_u32},
    IResult,
};

#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl From<&DnsRecord> for Bytes {
    fn from(record: &DnsRecord) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put(encode(&record.name));
        bytes.put_u16(record.record_type);
        bytes.put_u16(record.class);
        bytes.put_u32(record.ttl);
        bytes.put_u16(record.length);
        bytes.put(Bytes::from(record.data.clone()));

        bytes.freeze()
    }
}

impl DnsRecord {
    pub fn parse_request<'a>(
        (input, message): (&'a [u8], &'a [u8]),
        count: usize,
    ) -> IResult<&'a [u8], Vec<DnsRecord>> {
        multi::count(
            |_i| {
                let (input, names) = DnsQuestion::parse_name((input, message))?;
                let (input, record_type) = be_u16(input)?;
                let (input, class) = be_u16(input)?;
                let (input, ttl) = be_u32(input)?;
                let (input, length) = be_u16(input)?;
                let (input, data) = take_bytes(length)(input)?;
                let name = names.join(".");

                Ok((
                    input,
                    DnsRecord {
                        name,
                        record_type,
                        class,
                        ttl,
                        length,
                        data: data.to_owned(),
                    },
                ))
            },
            count,
        )(input)
    }

    pub fn to_response(packet: &DnsPacket, ip_address: &String) -> Vec<DnsRecord> {
        let header = packet.header();
        let questions = packet.questions();

        let mut answers = Vec::with_capacity(header.answer_count() as usize);
        for i in 0..header.question_count() as usize {
            let record = Self::forward_dns(questions.get(i).unwrap().to_owned(), ip_address);

            answers.push(record);
        }
        answers
    }

    fn forward_dns(question: DnsQuestion, ip_address: &String) -> DnsRecord {
        let socket = UdpSocket::bind("localhost:0").expect("Failed to bind to resolver");
        let header = DnsHeader {
            id: 123,
            query_response: false,
            op_code: 0,
            authoritative_answer: false,
            truncated_message: false,
            desired_recursion: true,
            available_recursion: false,
            reserved_bits: 0,
            response_code: 0,
            question_count: 1,
            answer_count: 0,
            authority_count: 0,
            additional_count: 0,
        };

        let packet = DnsPacket {
            header,
            questions: vec![question],
            answers: vec![],
        };
        socket
            .send_to(&Bytes::from(packet), ip_address)
            .expect("Unable to send message");

        let mut buf = [0; 512];

        socket.recv_from(&mut buf).expect("Recieved message");
        let (_, packet) = DnsPacket::parse_request(&buf).expect("Unable to parse dns response");

        packet.answers().first().unwrap().to_owned()
    }
}
