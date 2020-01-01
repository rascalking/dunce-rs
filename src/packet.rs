use std::fmt;

use byteorder::{ByteOrder, NetworkEndian};
use rand::prelude::*;

use super::consts::*;
use super::question::Question;

pub struct Packet {
    buf: Option<Vec<u8>>,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "{{id: {}, qr: {}, aa: {}, tc: {}, rd: {}, ra: {}, opcode: {}, qdcount: {}, ancount: {}, nscount: {}, arcount: {}, length: {}}}",
               self.id(), self.is_query(), self.is_authoritative(),
               self.is_truncated(), self.is_recursion_desired(),
               self.is_recursion_available(), self.opcode(), self.qdcount(),
               self.ancount(), self.nscount(), self.arcount(),
               self.buf().len())
    }
}

impl Packet {
    fn new() -> Packet {
        let buf = Some(vec![0 as u8; HEADER_LENGTH]);
        Packet { buf }
    }

    pub fn new_query(query: &str) -> Packet {
        let mut packet = Packet::new();

        // set up headers
        packet.set_id(rand::thread_rng().gen::<u16>());
        packet.set_query(true);
        packet.set_opcode(0);
        packet.set_recursion_desired(true);

        // set up the question
        let question = Question::new(query);
        packet.add_question(question);

        // return the finished packet
        packet
    }

    pub fn buf(&self) -> &[u8] {
        self.buf.as_ref().unwrap()
    }

    pub fn mut_buf(&mut self) -> &mut Vec<u8> {
        self.buf.as_mut().unwrap()
    }

    pub fn add_question(&mut self, question: Question) {
        // yes, this assumes we only have questions in the packet.  shaddup.
        let bytes = question.as_bytes();
        self.mut_buf().extend_from_slice(bytes.as_ref().unwrap());
        self.set_qdcount(self.qdcount()+1);
    }

    // https://www.ietf.org/rfc/rfc1035.html#section-4.1.1
    /*
                                        1  1  1  1  1  1
          0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                      ID                       |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                    QDCOUNT                    |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                    ANCOUNT                    |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                    NSCOUNT                    |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
        |                    ARCOUNT                    |
        +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
     */

    fn id(&self) -> u16 {
        NetworkEndian::read_u16(&self.buf()[ID_OFFSET..])
    }

    fn set_id(&mut self, id: u16) {
        NetworkEndian::write_u16(&mut self.mut_buf()[ID_OFFSET..], id)
    }

    fn flags(&self) -> u16 {
        NetworkEndian::read_u16(&self.buf()[FLAGS_OFFSET..])
    }

    fn set_flags(&mut self, flags: u16) {
        NetworkEndian::write_u16(&mut self.mut_buf()[FLAGS_OFFSET..], flags)
    }

    fn is_query(&self) -> bool {
        // it's a query when QR == 0
        !(self.flags() & QR == QR)
    }

    fn set_query(&mut self, is_query: bool) {
        let mut flags = self.flags();
        if is_query {
            // it's a query when QR == 0
            flags &= !QR;
        }
        else {
            flags |= QR;
        }
        self.set_flags(flags);
    }

    fn opcode(&self) -> u8 {
        ((self.flags() & OPCODE_MASK) >> OPCODE_SHIFT) as u8
    }

    fn set_opcode(&mut self, opcode: u8) {
        let mut flags = self.flags();
        flags &= !OPCODE_MASK; // zero out the existing opcode
        flags |= (opcode as u16) << OPCODE_SHIFT;
        self.set_flags(flags);
    }

    fn is_authoritative(&self) -> bool {
        self.flags() & AA == AA
    }

    fn set_authoritative(&mut self, is_authoritative: bool) {
        let mut flags = self.flags();
        if is_authoritative {
            flags |= AA;
        }
        else {
            flags &= !AA;
        }
        self.set_flags(flags);
    }

    fn is_truncated(&self) -> bool {
        self.flags() & TC == TC
    }

    fn set_truncated(&mut self, is_truncated: bool) {
        let mut flags = self.flags();
        if is_truncated {
            flags |= TC;
        }
        else {
            flags &= !TC;
        }
        self.set_flags(flags);
    }

    fn is_recursion_desired(&self) -> bool {
        self.flags() & RD == RD
    }

    fn set_recursion_desired(&mut self, is_recursion_desired: bool) {
        let mut flags = self.flags();
        if is_recursion_desired {
            flags |= RD;
        }
        else {
            flags &= !RD;
        }
        self.set_flags(flags);
    }

    fn is_recursion_available(&self) -> bool {
        self.flags() & RA == RA
    }

    fn set_recursion_available(&mut self, is_recursion_available: bool) {
        let mut flags = self.flags();
        if is_recursion_available {
            flags |= RA;
        }
        else {
            flags &= !RA;
        }
        self.set_flags(flags);
    }

    fn rcode(&self) -> u8 {
        (self.flags() & RCODE_MASK) as u8
    }

    fn set_rcode(&mut self, rcode: u8) {
        let mut flags = self.flags();
        flags &= !RCODE_MASK; // zero out the existing rcode
        flags |= rcode as u16;
        self.set_flags(flags);
    }

    fn qdcount(&self) -> u16 {
        NetworkEndian::read_u16(&self.buf()[QDCOUNT_OFFSET..])
    }

    fn set_qdcount(&mut self, qdcount: u16) {
        NetworkEndian::write_u16(&mut self.mut_buf()[QDCOUNT_OFFSET..], qdcount)
    }

    fn ancount(&self) -> u16 {
        NetworkEndian::read_u16(&self.buf()[ANCOUNT_OFFSET..])
    }

    fn set_ancount(&mut self, ancount: u16) {
        NetworkEndian::write_u16(&mut self.mut_buf()[ANCOUNT_OFFSET..], ancount)
    }

    fn nscount(&self) -> u16 {
        NetworkEndian::read_u16(&self.buf()[NSCOUNT_OFFSET..])
    }

    fn set_nscount(&mut self, nscount: u16) {
        NetworkEndian::write_u16(&mut self.mut_buf()[NSCOUNT_OFFSET..], nscount)
    }

    fn arcount(&self) -> u16 {
        NetworkEndian::read_u16(&self.buf()[ARCOUNT_OFFSET..])
    }

    fn set_arcount(&mut self, arcount: u16) {
        NetworkEndian::write_u16(&mut self.mut_buf()[ARCOUNT_OFFSET..], arcount)
    }
/*
                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                                               |
    /                     QNAME                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QTYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QCLASS                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                                               |
    /                                               /
    /                      NAME                     /
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     CLASS                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TTL                      |
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                   RDLENGTH                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
    /                     RDATA                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
 */
}
