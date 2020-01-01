use byteorder::{ByteOrder, NetworkEndian};

pub struct Question<'a> {
    labels: Option<Vec<&'a str>>,
}

impl Question<'_> {
    pub fn new(query: &str) -> Question {
        let labels: Option<Vec<&str>> = Some(query.split('.').rev().collect());
        Question { labels }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.labels {
            Some(labels) => {
                let mut buf = vec![];
                for label in labels {
                    buf.push(label.len() as u8);
                    buf.extend_from_slice(label.as_bytes());
                }
                buf.push(0);
                let mut offset = buf.len();
                buf.resize(offset+4, 0);
                NetworkEndian::write_u16(&mut buf[offset..], 1 as u16); // QTYPE A
                offset += 2;
                NetworkEndian::write_u16(&mut buf[offset..], 1 as u16); // QCLASS IN
                Some(buf)
            },
            None => None
        }
    }
}
