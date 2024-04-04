use super::{header::DnsHeader, DnsAnswer, DnsQuestion};
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answer: DnsAnswer,
}

impl From<DnsPacket> for Bytes {
    fn from(packet: DnsPacket) -> Self {
        let mut bytes = BytesMut::with_capacity(12);
        bytes.put(packet.header().into());
        for i in 0..packet.header().question_count() as usize {
            if let Some(question) = packet.questions().get(i) {
                bytes.put::<Bytes>(question.into());
            }
        }
        bytes.put::<Bytes>(packet.answer().clone().into());
        bytes.freeze()
    }
}

impl DnsPacket {
    pub fn new(packet: DnsPacket) -> Self {
        Self {
            header: packet.header,
            questions: packet.questions,
            answer: packet.answer,
        }
    }

    pub fn header(&self) -> &DnsHeader {
        &self.header
    }

    pub fn questions(&self) -> &Vec<DnsQuestion> {
        &self.questions
    }

    pub fn answer(&self) -> &DnsAnswer {
        &self.answer
    }
}
