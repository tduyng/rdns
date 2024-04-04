use bytes::{Buf, BufMut, Bytes, BytesMut};

pub fn encode(name: &str) -> Bytes {
    let mut bytes = BytesMut::new();
    for content in name.split('.') {
        bytes.put_u8(content.len() as u8);
        bytes.put(content.as_bytes());
    }
    bytes.put_u8(0x00);
    bytes.freeze()
}

pub fn decode(bytes: &mut Bytes) -> String {
    let mut label = String::new();
    loop {
        let len = bytes.get_u8();
        if len == 0 {
            break;
        }
        let content = bytes.copy_to_bytes(len as usize);
        label.push_str(std::str::from_utf8(&content[..]).unwrap());
        label.push('.');
    }
    label.pop();
    label
}
