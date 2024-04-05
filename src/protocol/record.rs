use super::{DnsPacket, DnsQuestion};
use crate::utils::encode;
use bytes::{BufMut, Bytes, BytesMut};
use nom::{
    bytes::complete::take as take_bytes,
    multi,
    number::complete::{be_u16, be_u32},
    IResult,
};

#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
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

impl DnsRecord {
    pub fn parse_request<'a>(
        (input, message): (&'a [u8], &'a [u8]),
        count: usize,
    ) -> IResult<&'a [u8], Vec<DnsRecord>> {
        multi::count(
            |_i| {
                let (input, names) = DnsQuestion::parse_name((input, message))?;
                let (input, record_type) = be_u16(input)?;
                let (input, class) = be_u16(input)?;
                let (input, ttl) = be_u32(input)?;
                let (input, length) = be_u16(input)?;
                let (input, data) = take_bytes(length)(input)?;
                let name = names.join(".");

                Ok((
                    input,
                    DnsRecord {
                        name,
                        record_type,
                        class,
                        ttl,
                        length,
                        data: data.to_owned(),
                    },
                ))
            },
            count,
        )(input)
    }

    pub fn to_response(packet_response: &DnsPacket) -> Vec<DnsRecord> {
        let header = packet_response.header();
        let questions = packet_response.questions();

        let mut answers = Vec::with_capacity(header.answer_count() as usize);
        for _ in 0..header.question_count() as usize {
            let answer = DnsRecord {
                name: questions.first().unwrap().name().clone(),
                record_type: 1,
                class: 1,
                ttl: 60,
                length: 4,
                data: vec![0x8, 0x8, 0x8, 0x8],
            };
            answers.push(answer);
        }
        answers
    }
}
