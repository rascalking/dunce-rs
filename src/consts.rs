pub const HEADER_LENGTH: usize = 12;

pub const ID_OFFSET: usize = 0;
pub const FLAGS_OFFSET: usize = 2;
pub const QDCOUNT_OFFSET: usize = 4;
pub const ANCOUNT_OFFSET: usize = 6;
pub const NSCOUNT_OFFSET: usize = 8;
pub const ARCOUNT_OFFSET: usize = 10;

pub const QR: u16 = 1 << 15;
pub const AA: u16 = 1 << 10;
pub const TC: u16 = 1 << 9;
pub const RD: u16 = 1 << 8;
pub const RA: u16 = 1 << 7;

pub const OPCODE_SHIFT: u8 = 11;
pub const OPCODE_MASK: u16 = 15 << OPCODE_SHIFT;
pub const RCODE_MASK: u16 = 15;
