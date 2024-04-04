use bytes::{BufMut, Bytes, BytesMut};

pub fn encode(name: &str) -> Bytes {
    let mut bytes = BytesMut::new();
    for label in name.split('.') {
        bytes.put_u8(label.len() as u8);
        bytes.put(label.as_bytes());
    }
    bytes.put_u8(0x00); // Null terminator
    bytes.freeze()
}
