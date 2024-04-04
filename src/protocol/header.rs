use anyhow::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug)]
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

impl TryFrom<Bytes> for DnsHeader {
    type Error = anyhow::Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let len = bytes.len();

        if len != 12 {
            return Err(anyhow::anyhow!(
                "Expected 12 bytes for header, but got {}",
                len
            ));
        }

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
        Self::extract_bits(self.bytes[2], 0, 1)
    }

    pub fn op_code(&self) -> u8 {
        Self::extract_bits(self.bytes[2], 1, 4)
    }

    pub fn authoritative_answer(&self) -> u8 {
        Self::extract_bits(self.bytes[2], 5, 1)
    }

    pub fn truncated_message(&self) -> u8 {
        Self::extract_bits(self.bytes[2], 7, 1)
    }

    pub fn available_recursion(&self) -> u8 {
        Self::extract_bits(self.bytes[3], 0, 1)
    }

    pub fn response_code(&self) -> u8 {
        Self::extract_bits(self.bytes[3], 4, 4)
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

    // Extracts the specified number of bits starting from the given offset
    fn extract_bits(byte: u8, offset: u8, num_bits: u8) -> u8 {
        (byte >> (8 - offset - num_bits)) & ((1 << num_bits) - 1)
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
                | (op_code << 4)
                | (authoritative_answer << 2)
                | (truncated_message << 1)
                | desired_recursion,
        );
        bytes_mut.put_u8((available_recursion << 7) | (response_code));
        bytes_mut.put_u16(question_count);
        bytes_mut.put_u16(answer_count);
        bytes_mut.put_u16(authority_count);
        bytes_mut.put_u16(additional_count);

        Ok(DnsHeader {
            bytes: bytes_mut.freeze(),
        })
    }
}
