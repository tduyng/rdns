use crate::utils::encode;
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct DnsAnswer {
    records: Vec<Record>,
}
#[derive(Debug, Clone)]
pub struct Record {
    pub name: String,
    pub record_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl DnsAnswer {
    pub fn new(record: Record) -> Self {
        Self {
            records: vec![Record {
                name: record.name,
                record_type: record.record_type,
                class: record.class,
                ttl: record.ttl,
                length: record.length,
                data: record.data,
            }],
        }
    }
}

impl From<DnsAnswer> for Bytes {
    fn from(answer: DnsAnswer) -> Self {
        let mut bytes = BytesMut::new();
        for record in answer.records {
            bytes.put(encode(&record.name));
            bytes.put_u16(record.record_type);
            bytes.put_u16(record.class);
            bytes.put_u32(record.ttl);
            bytes.put_u16(record.length);
            bytes.put(Bytes::from(record.data));
        }

        bytes.freeze()
    }
}
