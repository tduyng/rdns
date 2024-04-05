use super::DnsPacket;
use crate::utils::{decode, encode};
use bytes::{Buf, BufMut, Bytes, BytesMut};

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
    pub fn from_bytes(bytes: &mut Bytes, orig: &Bytes) -> Self {
        let name = decode(bytes, orig);
        let record_type = bytes.get_u16();
        let class = bytes.get_u16();
        let ttl = bytes.get_u32();
        let length = bytes.get_u16();
        let data = bytes.copy_to_bytes(length as usize);

        Self {
            name,
            record_type,
            class,
            ttl,
            length,
            data: Vec::from(data.chunk()),
        }
    }

    pub fn to_response(packet_response: &DnsPacket) -> Vec<DnsRecord> {
        let header = packet_response.header();
        let questions = packet_response.questions();

        let mut answers = Vec::with_capacity(header.answer_count() as usize);
        for _ in 0..header.question_count() as usize {
            let answer = DnsRecord {
                name: questions.first().unwrap().name().clone(),
                record_type: 1,
                class: 1,
                ttl: 60,
                length: 4,
                data: vec![0x8, 0x8, 0x8, 0x8],
            };
            answers.push(answer);
        }
        answers
    }
}
