use super::header::DnsHeader;
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct DnsPacket {
    header: DnsHeader,
}

impl From<DnsPacket> for Bytes {
    fn from(packet: DnsPacket) -> Self {
        let header_bytes: Bytes = packet.header().bytes().clone();
        let mut bytes = BytesMut::with_capacity(header_bytes.len());
        bytes.put(header_bytes);
        bytes.freeze()
    }
}

impl DnsPacket {
    pub fn new(header: DnsHeader) -> Self {
        Self { header }
    }

    pub fn header(&self) -> &DnsHeader {
        &self.header
    }
}
