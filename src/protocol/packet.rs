use std::net::UdpSocket;

use super::{DnsHeader, DnsQuestion, DnsRecord};
use bytes::{BufMut, Bytes, BytesMut};
use nom::{bits, IResult};

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
}

impl From<DnsPacket> for Bytes {
    fn from(packet: DnsPacket) -> Self {
        let mut bytes = BytesMut::with_capacity(12);
        bytes.put::<Bytes>(packet.header().into());

        for i in 0..packet.header().question_count() as usize {
            if let Some(question) = packet.questions().get(i) {
                bytes.put::<Bytes>(question.into());
            }
        }
        for i in 0..packet.header().answer_count() as usize {
            if let Some(answer) = packet.answers().get(i) {
                bytes.put::<Bytes>(answer.into());
            }
        }
        bytes.freeze()
    }
}

impl DnsPacket {
    pub fn header(&self) -> &DnsHeader {
        &self.header
    }

    pub fn questions(&self) -> &Vec<DnsQuestion> {
        &self.questions
    }

    pub fn answers(&self) -> &Vec<DnsRecord> {
        &self.answers
    }

    pub fn parse_request(buf: &[u8]) -> IResult<&[u8], DnsPacket> {
        let message = buf;
        let (buf, header) = bits(DnsHeader::parse_request)(buf)?;

        let (buf, questions) =
            DnsQuestion::parse_request((buf, message), header.question_count() as usize)?;

        let (buf, answers) =
            DnsRecord::parse_request((buf, message), header.answer_count() as usize)?;

        Ok((
            buf,
            DnsPacket {
                header,
                questions,
                answers,
            },
        ))
    }

    pub fn to_response(buf: &[u8], ip_address: String) -> Bytes {
        let (_, request) = Self::parse_request(buf).unwrap();

        let mut questions: Vec<DnsQuestion> =
            Vec::with_capacity(request.header().question_count() as usize);
        for i in 0..request.header().question_count() as usize {
            questions.push(request.questions().get(i).unwrap().to_owned());
        }

        let mut answers: Vec<DnsRecord> =
            Vec::with_capacity(request.header().answer_count() as usize);
        for i in 0..request.header().answer_count() as usize {
            let record = Self::forward_dns(questions.get(i).unwrap().to_owned(), &ip_address);
            answers.push(record);
        }

        let dns_packet = DnsPacket {
            header: DnsHeader::to_response(&request),
            questions: DnsQuestion::to_response(&request),
            answers: DnsRecord::to_response(&request),
        };

        dns_packet.into()
    }

    fn forward_dns(question: DnsQuestion, resolver_addr: &String) -> DnsRecord {
        let socket = UdpSocket::bind("localhost:0").expect("failed to bind to resolver");
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
            .send_to(&Bytes::from(packet), resolver_addr)
            .expect("unable to send message");

        let mut buf = [0; 512];

        socket
            .recv_from(&mut buf)
            .expect("should have recieved message");
        let (_, packet) = DnsPacket::parse_request(&buf).expect("unable to parse dns response");

        packet.answers().first().unwrap().to_owned()
    }
}
