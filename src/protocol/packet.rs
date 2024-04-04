use super::{header::DnsHeader, DnsQuestion};
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct DnsPacket {
    header: DnsHeader,
    question: DnsQuestion,
}

impl From<DnsPacket> for Bytes {
    fn from(packet: DnsPacket) -> Self {
        let header_bytes: Bytes = packet.header().bytes().clone();
        let question_bytes: Bytes = packet.question().into();
        let mut bytes = BytesMut::with_capacity(header_bytes.len() + question_bytes.len());
        bytes.put(header_bytes);
        bytes.put(question_bytes);
        bytes.freeze()
    }
}

impl DnsPacket {
    pub fn new(header: DnsHeader, question: DnsQuestion) -> Self {
        Self { header, question }
    }

    pub fn header(&self) -> &DnsHeader {
        &self.header
    }

    pub fn question(&self) -> &DnsQuestion {
        &self.question
    }
}
