use super::DnsPacket;
use bytes::{BufMut, Bytes, BytesMut};
use nom::{
    complete::{bool as take_bool, take},
    IResult,
};

#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,
    pub query_response: bool,
    pub op_code: u8,
    pub authoritative_answer: bool,
    pub truncated_message: bool,
    pub desired_recursion: bool,
    pub available_recursion: bool,
    pub reserved_bits: u8,
    pub response_code: u8,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

impl From<&DnsHeader> for Bytes {
    fn from(value: &DnsHeader) -> Self {
        let mut bytes = BytesMut::with_capacity(12);
        bytes.put_u16(value.id);
        let flags: u16 = ((value.query_response as u16) << 15)
            | ((value.op_code as u16) << 11)
            | ((value.authoritative_answer as u16) << 10)
            | ((value.truncated_message as u16) << 9)
            | ((value.desired_recursion as u16) << 8)
            | ((value.available_recursion as u16) << 7)
            | ((value.reserved_bits as u16) << 4)
            | value.response_code as u16;
        bytes.put_u16(flags);
        bytes.put_u16(value.question_count);
        bytes.put_u16(value.answer_count);
        bytes.put_u16(value.authority_count);
        bytes.put_u16(value.additional_count);
        bytes.freeze()
    }
}

impl DnsHeader {
    pub fn new(header: DnsHeader) -> Self {
        header
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn query_response(&self) -> bool {
        self.query_response
    }

    pub fn op_code(&self) -> u8 {
        self.op_code
    }

    pub fn authoritative_answer(&self) -> bool {
        self.authoritative_answer
    }

    pub fn truncated_message(&self) -> bool {
        self.truncated_message
    }

    pub fn desired_recursion(&self) -> bool {
        self.desired_recursion
    }

    pub fn available_recursion(&self) -> bool {
        self.available_recursion
    }

    pub fn reserved_bits(&self) -> u8 {
        self.reserved_bits
    }

    pub fn response_code(&self) -> u8 {
        self.response_code
    }

    pub fn question_count(&self) -> u16 {
        self.question_count
    }

    pub fn answer_count(&self) -> u16 {
        self.answer_count
    }

    pub fn authority_count(&self) -> u16 {
        self.authority_count
    }

    pub fn additional_count(&self) -> u16 {
        self.additional_count
    }

    pub fn parse_request(buf: (&[u8], usize)) -> IResult<(&[u8], usize), DnsHeader> {
        let (buf, id): (_, u16) = take(16_usize)(buf)?;
        let (buf, query_response): (_, bool) = take_bool(buf)?;
        let (buf, op_code): (_, u8) = take(4_usize)(buf)?;
        let (buf, authoritative_answer): (_, bool) = take_bool(buf)?;
        let (buf, truncated_message): (_, bool) = take_bool(buf)?;
        let (buf, desired_recursion): (_, bool) = take_bool(buf)?;
        let (buf, available_recursion): (_, bool) = take_bool(buf)?;
        let (buf, reserved_bits): (_, u8) = take(3_usize)(buf)?;
        let (buf, response_code): (_, u8) = take(4_usize)(buf)?;
        let (buf, question_count): (_, u16) = take(16_usize)(buf)?;
        let (buf, answer_count): (_, u16) = take(16_usize)(buf)?;
        let (buf, authority_count): (_, u16) = take(16_usize)(buf)?;
        let (buf, additional_count): (_, u16) = take(16_usize)(buf)?;

        let header = DnsHeader {
            id,
            query_response,
            op_code,
            authoritative_answer,
            truncated_message,
            desired_recursion,
            available_recursion,
            reserved_bits,
            response_code,
            question_count,
            answer_count,
            authority_count,
            additional_count,
        };

        Ok((buf, header))
    }

    pub fn to_response(request: &DnsPacket) -> DnsHeader {
        let request_header = request.header();
        let op_code = request_header.op_code();
        let response_code = if op_code == 0 { 0 } else { 4 };

        DnsHeader {
            id: request_header.id(),
            query_response: true,
            op_code,
            authoritative_answer: false,
            truncated_message: false,
            desired_recursion: request_header.desired_recursion(),
            available_recursion: false,
            reserved_bits: 0,
            response_code,
            question_count: request_header.question_count(),
            answer_count: request_header.question_count(),
            authority_count: request_header.question_count(),
            additional_count: request_header.question_count(),
        }
    }
}
