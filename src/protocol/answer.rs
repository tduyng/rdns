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

impl DnsRecord {
    pub fn new(record: DnsRecord) -> Self {
        record
    }
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

impl From<&mut Bytes> for DnsRecord {
    fn from(value: &mut Bytes) -> Self {
        let length = value.get_u16();
        let data = value.copy_to_bytes(length as usize);

        Self {
            name: decode(value),
            record_type: length,
            class: length,
            ttl: value.get_u32(),
            length,
            data: Vec::from(data.chunk()),
        }
    }
}
