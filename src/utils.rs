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

pub fn decode(bytes: &mut Bytes, original: &Bytes) -> String {
    let mut label = String::new();
    loop {
        let len = bytes.get_u8();
        if len == 0 {
            break;
        } else if len >> 6 == 0b11 {
            let byte_two = bytes.get_u8();
            let offset: usize = ((((len & 0b0011_1111) as u16) << 8) | byte_two as u16) as usize;
            let name = decode(&mut original.clone().slice(offset..), original);
            label.push_str(name.as_str());
            label.push('.');
        } else {
            let content = bytes.copy_to_bytes(len as usize);
            label.push_str(std::str::from_utf8(&content[..]).unwrap());
            label.push('.');
        }
    }
    label.pop();
    label
}
