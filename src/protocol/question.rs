use super::DnsPacket;
use crate::utils::{decode, encode};
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: String,
    pub record_type: u16,
    pub class: u16,
}

impl From<&DnsQuestion> for Bytes {
    fn from(value: &DnsQuestion) -> Self {
        let mut bytes = BytesMut::new();
        bytes.put(encode(&value.name));
        bytes.put_u16(value.record_type);
        bytes.put_u16(value.class);
        bytes.freeze()
    }
}

impl DnsQuestion {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn record_type(&self) -> u16 {
        self.record_type
    }

    pub fn class(&self) -> u16 {
        self.class
    }

    pub fn from_bytes(bytes: &mut Bytes, original: &Bytes) -> Self {
        let name = decode(bytes, original);
        let record_type = if bytes.remaining() >= 2 {
            bytes.get_u16()
        } else {
            1
        };

        let class = if bytes.remaining() >= 2 {
            bytes.get_u16()
        } else {
            1
        };

        Self {
            name,
            record_type,
            class,
        }
    }

    pub fn to_response(packet_response: &DnsPacket) -> Vec<DnsQuestion> {
        let header = packet_response.header();
        let mut questions: Vec<DnsQuestion> = Vec::with_capacity(header.question_count() as usize);
        for i in 0..header.question_count() as usize {
            questions.push(packet_response.questions().get(i).unwrap().to_owned());
        }
        questions
    }
}
