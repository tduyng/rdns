use anyhow::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct DnsHeader {
    bytes: Bytes,
}

#[derive(Debug, Default)]
pub struct DnsHeaderBuilder {
    packet_id: Option<u16>,
    query_response: Option<u8>,
    op_code: Option<u8>,
    authoritative_answer: Option<u8>,
    truncated_message: Option<u8>,
    desired_recursion: Option<u8>,
    available_recursion: Option<u8>,
    response_code: Option<u8>,
    question_count: Option<u16>,
    answer_count: Option<u16>,
    authority_count: Option<u16>,
    additional_count: Option<u16>,
}

impl TryFrom<&mut Bytes> for DnsHeader {
    type Error = anyhow::Error;

    fn try_from(buf: &mut Bytes) -> Result<Self, Self::Error> {
        let bytes = buf.slice(0..12);
        buf.advance(12);

        Ok(DnsHeader { bytes })
    }
}

impl From<DnsHeader> for Bytes {
    fn from(header: DnsHeader) -> Self {
        header.bytes
    }
}

impl DnsHeader {
    pub fn new(builder: DnsHeaderBuilder) -> Result<Self> {
        builder.build()
    }

    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    pub fn into(&self) -> Bytes {
        self.bytes.slice(..)
    }

    pub fn packet_id(&self) -> u16 {
        self.bytes.slice(0..2).get_u16()
    }

    pub fn query_response(&self) -> u8 {
        // Extract the QR bit (1st bit of the 3rd byte)
        (self.bytes[2] & 0b1000_0000) >> 7
    }

    pub fn op_code(&self) -> u8 {
        // Extract the OPCODE bits (2nd to 5th bits of the 3rd byte)
        (self.bytes[2] & 0b0111_1000) >> 3
    }

    pub fn authoritative_answer(&self) -> u8 {
        // Extract the AA bit (3rd bit of the 3rd byte)
        (self.bytes[2] & 0b0000_0100) >> 2
    }

    pub fn truncated_message(&self) -> u8 {
        // Extract the TC bit (2nd bit of the 3rd byte)
        (self.bytes[2] & 0b0000_0010) >> 1
    }

    pub fn desired_recursion(&self) -> u8 {
        // Extract the RD bit (1st bit of the 3rd byte)
        self.bytes[2] & 0b0000_0001
    }

    pub fn available_recursion(&self) -> u8 {
        // Extract the RA bit (8th bit of the 4th byte)
        (self.bytes[3] & 0b1000_0000) >> 7
    }

    pub fn response_code(&self) -> u8 {
        // Extract the RCODE bits (1st to 4th bits of the 4th byte)
        self.bytes[3] & 0b0000_1111
    }

    pub fn question_count(&self) -> u16 {
        self.bytes.slice(4..6).get_u16()
    }

    pub fn answer_count(&self) -> u16 {
        self.bytes.slice(6..8).get_u16()
    }

    pub fn authority_count(&self) -> u16 {
        self.bytes.slice(8..10).get_u16()
    }

    pub fn additional_count(&self) -> u16 {
        self.bytes.slice(10..12).get_u16()
    }
}

impl DnsHeaderBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn packet_id(mut self, packet_id: u16) -> Self {
        self.packet_id = Some(packet_id);
        self
    }

    pub fn query_response(mut self, query_response: u8) -> Self {
        self.query_response = Some(query_response);
        self
    }

    pub fn op_code(mut self, op_code: u8) -> Self {
        self.op_code = Some(op_code);
        self
    }

    pub fn authoritative_answer(mut self, authoritative_answer: u8) -> Self {
        self.authoritative_answer = Some(authoritative_answer);
        self
    }

    pub fn truncated_message(mut self, truncated_message: u8) -> Self {
        self.truncated_message = Some(truncated_message);
        self
    }

    pub fn desired_recursion(mut self, desired_recursion: u8) -> Self {
        self.desired_recursion = Some(desired_recursion);
        self
    }

    pub fn available_recursion(mut self, available_recursion: u8) -> Self {
        self.available_recursion = Some(available_recursion);
        self
    }

    pub fn response_code(mut self, response_code: u8) -> Self {
        self.response_code = Some(response_code);
        self
    }

    pub fn question_count(mut self, question_count: u16) -> Self {
        self.question_count = Some(question_count);
        self
    }

    pub fn answer_count(mut self, answer_count: u16) -> Self {
        self.answer_count = Some(answer_count);
        self
    }

    pub fn authority_count(mut self, authority_count: u16) -> Self {
        self.authority_count = Some(authority_count);
        self
    }

    pub fn additional_count(mut self, additional_count: u16) -> Self {
        self.additional_count = Some(additional_count);
        self
    }

    pub fn build(self) -> Result<DnsHeader> {
        let packet_id = self.packet_id.ok_or("Packet ID is required").unwrap();
        let query_response = self.query_response.unwrap_or_default();
        let op_code = self.op_code.unwrap_or_default();
        let authoritative_answer = self.authoritative_answer.unwrap_or_default();
        let truncated_message = self.truncated_message.unwrap_or_default();
        let desired_recursion = self.desired_recursion.unwrap_or_default();
        let available_recursion = self.available_recursion.unwrap_or_default();
        let response_code = self.response_code.unwrap_or_default();
        let question_count = self.question_count.unwrap_or_default();
        let answer_count = self.answer_count.unwrap_or_default();
        let authority_count = self.authority_count.unwrap_or_default();
        let additional_count = self.additional_count.unwrap_or_default();

        let mut bytes_mut = BytesMut::with_capacity(12);
        bytes_mut.put_u16(packet_id);
        bytes_mut.put_u8(
            (query_response << 7)
                | (op_code << 3)
                | (authoritative_answer << 2)
                | (truncated_message << 1)
                | desired_recursion,
        );
        bytes_mut.put_u8((available_recursion << 7) | response_code);
        bytes_mut.put_u16(question_count);
        bytes_mut.put_u16(answer_count);
        bytes_mut.put_u16(authority_count);
        bytes_mut.put_u16(additional_count);

        Ok(DnsHeader {
            bytes: bytes_mut.freeze(),
        })
    }
}
