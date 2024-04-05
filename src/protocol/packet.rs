use super::{DnsHeader, DnsQuestion, DnsRecord};
use bytes::{BufMut, Bytes, BytesMut};
use nom::{bits, IResult};

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
}

impl From<DnsPacket> for Bytes {
    fn from(packet: DnsPacket) -> Self {
        let mut bytes = BytesMut::with_capacity(12);
        bytes.put::<Bytes>(packet.header().into());

        for i in 0..packet.header().question_count() as usize {
            if let Some(question) = packet.questions().get(i) {
                bytes.put::<Bytes>(question.into());
            }
        }
        for i in 0..packet.header().answer_count() as usize {
            if let Some(answer) = packet.answers().get(i) {
                bytes.put::<Bytes>(answer.into());
            }
        }
        bytes.freeze()
    }
}

impl DnsPacket {
    pub fn header(&self) -> &DnsHeader {
        &self.header
    }

    pub fn questions(&self) -> &Vec<DnsQuestion> {
        &self.questions
    }

    pub fn answers(&self) -> &Vec<DnsRecord> {
        &self.answers
    }

    pub fn parse_request(buf: &[u8]) -> IResult<&[u8], DnsPacket> {
        let message = buf;
        let (buf, header) = bits(DnsHeader::parse_request)(buf)?;

        let (buf, questions) =
            DnsQuestion::parse_request((buf, message), header.question_count() as usize)?;

        let (buf, answers) =
            DnsRecord::parse_request((buf, message), header.answer_count() as usize)?;

        Ok((
            buf,
            DnsPacket {
                header,
                questions,
                answers,
            },
        ))
    }

    pub fn to_response(buf: &[u8], ip_address: &String) -> Bytes {
        let (_, packet) = Self::parse_request(buf).unwrap();

        let dns_packet = DnsPacket {
            header: DnsHeader::to_response(&packet),
            questions: DnsQuestion::to_response(&packet),
            answers: DnsRecord::to_response(&packet, ip_address),
        };

        dns_packet.into()
    }
}
