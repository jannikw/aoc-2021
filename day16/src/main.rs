use std::fmt;

fn main() {
    let packet = parse_packet(&mut get_input());
    let version_sum = sum_versions(&packet);
    let result = evaluate_packet(&packet);

    println!("task 1: sum of versions = {}", version_sum);
    println!("task 2: result of evaluation = {}", result);
}

struct Reader {
    blocks: Vec<u8>,
    pos: usize,
}

impl Reader {
    fn new(hex_string: &str) -> Self {
        let blocks = hex_string
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .collect();

        Reader { blocks, pos: 0 }
    }

    fn consume(&mut self) -> bool {
        let block = self.pos / 4;
        let shift = 3 - self.pos % 4;
        if block < self.blocks.len() {
            self.pos += 1;
            return ((self.blocks[block] >> shift) & 1) == 1;
        }

        panic!("read after end");
    }

    fn read_u8(&mut self, length: u8) -> u8 {
        let mut result = 0;
        for _ in 0..length {
            result <<= 1;
            result |= if self.consume() { 1 } else { 0 };
        }
        return result;
    }

    fn read_u64(&mut self, length: u8) -> u64 {
        let mut result = 0;
        for _ in 0..length {
            result <<= 1;
            result |= if self.consume() { 1 } else { 0 };
        }
        return result;
    }
}

impl fmt::Display for Reader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in &self.blocks {
            write!(f, "{}{}{}{} ", b >> 3 & 1, b >> 2 & 1, b >> 1 & 1, b & 1)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Packet(u8, PacketData);

#[derive(Debug, Eq, PartialEq)]
enum PacketData {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Box<Packet>, Box<Packet>),
    LessThan(Box<Packet>, Box<Packet>),
    EqualTo(Box<Packet>, Box<Packet>),
}

fn parse_packet(mut reader: &mut Reader) -> Packet {
    let version = reader.read_u8(3);
    let type_ = reader.read_u8(3);

    let data = match type_ {
        0 => PacketData::Sum(parse_packet_list(&mut reader)),
        1 => PacketData::Product(parse_packet_list(&mut reader)),
        2 => PacketData::Minimum(parse_packet_list(&mut reader)),
        3 => PacketData::Maximum(parse_packet_list(&mut reader)),
        4 => {
            let mut value: u64 = 0;
            let mut last_done = false;
            while !last_done {
                last_done = reader.consume() == false; // check first bit of group
                let block = reader.read_u8(4);
                value = value << 4 | block as u64;
            }

            PacketData::Literal(value)
        }
        5 => {
            let mut packets = parse_packet_list(&mut reader);
            assert_eq!(packets.len(), 2);
            let b = packets.pop().unwrap();
            let a = packets.pop().unwrap();
            PacketData::GreaterThan(Box::new(a), Box::new(b))
        }
        6 => {
            let mut packets = parse_packet_list(&mut reader);
            assert_eq!(packets.len(), 2);
            let b = packets.pop().unwrap();
            let a = packets.pop().unwrap();
            PacketData::LessThan(Box::new(a), Box::new(b))
        }
        7 => {
            let mut packets = parse_packet_list(&mut reader);
            assert_eq!(packets.len(), 2);
            let b = packets.pop().unwrap();
            let a = packets.pop().unwrap();
            PacketData::EqualTo(Box::new(a), Box::new(b))
        }
        _ => {
            panic!("invalid packet type: {}", type_);
        }
    };

    Packet(version, data)
}

fn parse_packet_list(mut reader: &mut Reader) -> Vec<Packet> {
    let length_type = reader.consume();
    if length_type == true {
        // read 11 bit as number of sub packets
        let count = reader.read_u64(11);
        (0..count).map(|_| parse_packet(&mut reader)).collect()
    } else {
        // read 15 bit as length of sub packets
        let length = reader.read_u64(15) as usize;
        let start = reader.pos;
        let mut packets = Vec::new();
        while reader.pos - start < length {
            packets.push(parse_packet(&mut reader));
        }
        packets
    }
}

fn sum_versions(Packet(version, data): &Packet) -> i32 {
    match data {
        PacketData::Literal(_) => *version as i32,
        PacketData::Sum(ps)
        | PacketData::Product(ps)
        | PacketData::Minimum(ps)
        | PacketData::Maximum(ps) => {
            *version as i32 + ps.iter().map(|p| sum_versions(p)).sum::<i32>()
        }
        PacketData::GreaterThan(a, b) | PacketData::LessThan(a, b) | PacketData::EqualTo(a, b) => {
            *version as i32 + sum_versions(a) + sum_versions(b)
        }
    }
}

fn evaluate_packet(Packet(_, data): &Packet) -> u64 {
    match data {
        PacketData::Literal(value) => *value,
        PacketData::Sum(ps) => ps.iter().map(|p| evaluate_packet(p)).sum(),
        PacketData::Product(ps) => ps.iter().fold(1, |product, p| product * evaluate_packet(p)),
        PacketData::Minimum(ps) => ps.iter().map(evaluate_packet).min().unwrap(),
        PacketData::Maximum(ps) => ps.iter().map(evaluate_packet).max().unwrap(),
        PacketData::GreaterThan(a, b) => {
            if evaluate_packet(a) > evaluate_packet(b) {
                1
            } else {
                0
            }
        }
        PacketData::LessThan(a, b) => {
            if evaluate_packet(a) < evaluate_packet(b) {
                1
            } else {
                0
            }
        }
        PacketData::EqualTo(a, b) => {
            if evaluate_packet(a) == evaluate_packet(b) {
                1
            } else {
                0
            }
        }
    }
}

fn get_input() -> Reader {
    Reader::new(include_str!("../../inputs/day16.txt").trim())
}
