use super::{DnsHeader, DnsHeaderBuilder, DnsQuestion, DnsRecord};
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
}

impl From<DnsPacket> for Bytes {
    fn from(packet: DnsPacket) -> Self {
        let mut bytes = BytesMut::with_capacity(12);
        bytes.put(packet.header().bytes().clone());
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

impl From<Bytes> for DnsPacket {
    fn from(mut bytes: Bytes) -> Self {
        let original = bytes.clone();
        let header = DnsHeader::try_from(&mut bytes).unwrap();

        let mut questions: Vec<DnsQuestion> = Vec::with_capacity(header.question_count() as usize);
        for _ in 0..header.question_count() {
            questions.push(DnsQuestion::from_bytes(&mut bytes, &original));
        }

        let mut answers: Vec<DnsRecord> = Vec::with_capacity(header.answer_count() as usize);
        for _ in 0..header.answer_count() {
            answers.push(DnsRecord::from_bytes(&mut bytes, &original));
        }

        Self {
            header,
            questions,
            answers,
        }
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

    pub fn parse_response(bytes: Bytes) -> Bytes {
        let request = DnsPacket::from(bytes);

        let request_header = request.header();
        let op_code = request_header.op_code();
        let response_code = if op_code == 0 { 0 } else { 4 };

        let header = DnsHeaderBuilder::new()
            .packet_id(request_header.packet_id())
            .query_response(1)
            .op_code(op_code)
            .desired_recursion(request_header.desired_recursion())
            .response_code(response_code)
            .question_count(request_header.question_count())
            .answer_count(request_header.question_count())
            .authority_count(request_header.question_count())
            .additional_count(request_header.question_count())
            .build()
            .unwrap();

        let mut questions: Vec<DnsQuestion> =
            Vec::with_capacity(request.header().question_count() as usize);
        for i in 0..request.header().question_count() as usize {
            questions.push(request.questions().get(i).unwrap().to_owned());
        }

        let mut answers = Vec::with_capacity(request.header().answer_count() as usize);
        for _ in 0..request.header().question_count() as usize {
            let answer = DnsRecord {
                name: request.questions().first().unwrap().name().clone(),
                record_type: 1,
                class: 1,
                ttl: 60,
                length: 4,
                data: vec![0x8, 0x8, 0x8, 0x8],
            };
            answers.push(answer);
        }

        let dns_packet = DnsPacket {
            header,
            questions,
            answers,
        };

        dns_packet.into()
    }
}
