use super::DnsPacket;
use crate::utils::{decode, encode};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use nom::{
    bytes::complete::take as take_bytes,
    error::{Error, FromExternalError},
    multi,
    number::complete::be_u16,
    IResult,
};

#[derive(Debug, Clone)]
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

    pub fn from_bytes(bytes: &mut Bytes, original: &Bytes) -> Self {
        let name = decode(bytes, original);
        let record_type = if bytes.remaining() >= 2 {
            bytes.get_u16()
        } else {
            1
        };

        let class = if bytes.remaining() >= 2 {
            bytes.get_u16()
        } else {
            1
        };

        Self {
            name,
            record_type,
            class,
        }
    }

    pub fn parse_request<'a>(
        (buf, message): (&'a [u8], &'a [u8]),
        count: usize,
    ) -> IResult<&'a [u8], Vec<DnsQuestion>> {
        let (buf, questions) = multi::count(
            |i| {
                let (i, names) = Self::parse_name((i, message))?;
                let (i, (record_type, class)) = Self::parse_type_class(i)?;
                Ok((
                    i,
                    DnsQuestion {
                        name: names.join("."),
                        record_type,
                        class,
                    },
                ))
            },
            count,
        )(buf)?;
        Ok((buf, questions))
    }

    pub fn to_response(packet_response: &DnsPacket) -> Vec<DnsQuestion> {
        let header = packet_response.header();
        let mut questions: Vec<DnsQuestion> = Vec::with_capacity(header.question_count() as usize);
        for i in 0..header.question_count() as usize {
            questions.push(packet_response.questions().get(i).unwrap().to_owned());
        }
        questions
    }

    pub fn parse_name<'a>((buf, message): (&'a [u8], &'a [u8])) -> IResult<&'a [u8], Vec<&'a str>> {
        let mut labels = Vec::new();
        let mut rest = buf;
        loop {
            if rest[0] == 0 {
                break;
            }
            let (new_rest, label) = Self::parse_dns_label((rest, message))?;
            labels.push(label);
            rest = new_rest;
        }
        let (rest, _) = take_bytes(1usize)(rest)?;
        Ok((rest, labels))
    }

    /* ---------------------------- Private methods ---------------------*/
    fn parse_dns_label<'a>((buf, message): (&'a [u8], &'a [u8])) -> IResult<&'a [u8], &'a str> {
        let start_buf = buf;
        let (buf, length) = take_bytes(1usize)(buf)?;
        let length = length[0] as usize;

        if length & 0xC0 == 0xC0 {
            return Self::parse_pointer((start_buf, message));
        }
        let (buf, label_bytes) = take_bytes(length)(buf)?;

        match std::str::from_utf8(label_bytes) {
            Ok(label) => Ok((buf, label)),
            Err(e) => Err(nom::Err::Failure(Error::from_external_error(
                buf,
                nom::error::ErrorKind::Fail,
                e,
            ))),
        }
    }

    fn parse_pointer<'a>((buf, message): (&'a [u8], &'a [u8])) -> IResult<&'a [u8], &'a str> {
        let (_buf, val) = be_u16(buf)?;
        let message_offset = (val ^ 0xC000) as usize;
        Self::parse_dns_label((&message[message_offset..], message))
    }

    fn parse_type_class(buf: &[u8]) -> IResult<&[u8], (u16, u16)> {
        let (buf, record_type) = be_u16(buf)?;
        let (buf, class) = be_u16(buf)?;
        Ok((buf, (record_type, class)))
    }
}
