use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct DnsQuestion {
    name: String,
    record_type: u16,
    class: u16,
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

fn encode(name: &str) -> Bytes {
    let mut bytes = BytesMut::new();
    for label in name.split('.') {
        bytes.put_u8(label.len() as u8);
        bytes.put(label.as_bytes());
    }
    bytes.put_u8(0x00); // Null terminator
    bytes.freeze()
}

impl DnsQuestion {
    pub fn new(name: impl ToString, record_type: u16, class: u16) -> DnsQuestion {
        Self {
            name: name.to_string(),
            record_type,
            class,
        }
    }
}
