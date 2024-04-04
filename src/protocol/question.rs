use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::utils::{decode, encode};

#[derive(Debug)]
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

impl From<&mut Bytes> for DnsQuestion {
    fn from(bytes: &mut Bytes) -> Self {
        let name = decode(bytes);
        Self {
            name,
            record_type: bytes.get_u16(),
            class: bytes.get_u16(),
        }
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
}
