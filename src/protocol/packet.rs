use super::{DnsHeader, DnsQuestion, DnsRecord};
use bytes::{Buf, BufMut, Bytes, BytesMut};

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

impl From<&mut Bytes> for DnsPacket {
    fn from(bytes: &mut Bytes) -> Self {
        let header = DnsHeader::try_from(bytes.slice(0..12)).unwrap();
        bytes.advance(12);

        let mut questions: Vec<DnsQuestion> = Vec::with_capacity(header.question_count() as usize);
        for _ in 0..header.question_count() {
            questions.push(DnsQuestion::from(&mut *bytes));
        }

        let mut answers: Vec<DnsRecord> = Vec::with_capacity(header.answer_count() as usize);
        for _ in 0..header.answer_count() {
            answers.push(DnsRecord::from(&mut *bytes));
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
}
