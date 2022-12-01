use bitstream_io::{BitReader, BitRead, BigEndian};
use std::io::{self, stdin, Stdin};

#[derive(Debug)]
enum Opcode {
    Sum,
    Multiply,
    Min,
    Max,
    GreaterThan = 5,
    LowerThan,
    Equals
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::Sum,
            1 => Opcode::Multiply,
            2 => Opcode::Min,
            3 => Opcode::Max,
            5 => Opcode::GreaterThan,
            6 => Opcode::LowerThan,
            7 => Opcode::Equals,
            _ => panic!("Unknown opcode: {}", v)
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    payload: Payload
}

#[derive(Debug)]
enum Payload {
    Literal(u64),
    Operator(Opcode, Vec<Packet>)
}

fn read_literal<T,U>(reader: &mut BitReader<T,U>) -> io::Result<(Payload, usize)> where T: io::Read, U: bitstream_io::Endianness {
    let mut value = 0;
    let mut bits_read = 0;
    loop {
        let has_more = reader.read_bit()?;
        value = (value << 4) | reader.read::<u64>(4)?;
        bits_read += 5;
        if !has_more {
            break;
        }
    }

    Ok((Payload::Literal(value), bits_read))
}

fn read_operator<T,U>(opcode: Opcode, reader: &mut BitReader<T,U>) -> io::Result<(Payload, usize)> where T: io::Read, U: bitstream_io::Endianness {
    let has_number_of_subpackets = reader.read_bit()?;
    let mut packets = vec![];
    let mut bits_read = 1;

    if has_number_of_subpackets {
        // the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet
        let packets_to_read: u16 = reader.read(11)?;
        bits_read += 11;
        for _ in 0..packets_to_read {
            let next = read_packet(reader)?;
            packets.push(next.0);
            bits_read += next.1;
        }
    } else {
        // the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet
        let mut subpackets_length: usize = reader.read::<u16>(15)? as usize;
        bits_read += 15;
        while subpackets_length > 0 {
            let next = read_packet(reader)?;
            packets.push(next.0);
            bits_read += next.1;
            subpackets_length -= next.1; 
        }

    }
    Ok((Payload::Operator(opcode, packets), bits_read))
}

fn read_packet<T,U>(reader: &mut BitReader<T,U>) -> io::Result<(Packet, usize)> where T: io::Read, U: bitstream_io::Endianness {
    let version = reader.read::<u8>(3)?;
    let type_id = reader.read::<u8>(3)?;
    let (payload, bits_read) = match type_id {
        4 => read_literal(reader)?,
        x => read_operator(Opcode::from(x), reader)?,
    };

    Ok((Packet{
        version: version,
        payload: payload
    }, 6 + bits_read))
}

fn read_input() -> Vec<Packet> {
    let mut reader: BitReader<Stdin, BigEndian> = BitReader::new(stdin());
    let mut packets = vec![];

    loop {
        let packet = read_packet(&mut reader);
        if packet.is_err() {
            println!("Error: {}", packet.err().unwrap());
            break;
        }
        packets.push(packet.unwrap().0);
        break; // input is only one packet
    }

    packets
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match &self.payload {
            Payload::Literal(n) => write!(f, "{}", n)?,
            Payload::Operator(opcode, packets) => {
                let (head, sep)= match opcode {
                    Opcode::Sum => ("(", "+"),
                    Opcode::Multiply => ("(", "*"),
                    Opcode::Min => ("min(", ","),
                    Opcode::Max => ("max(", ","),
                    Opcode::GreaterThan => ("(", ">"),
                    Opcode::LowerThan => ("(", "<"),
                    Opcode::Equals => ("(", "=="),
                    _ => panic!("unknown opcode with packets")
                };
                write!(f, "{}", head)?;
                for (i,p) in packets.iter().enumerate() {
                    write!(f, "{}{}", p, if i == packets.len()-1 { ")" } else { sep })?;
                }
            }
        }

        Ok(())
    }
}

impl Packet {
    fn version_sum(&self) -> u32 {
        let mut sum = self.version as u32;
        match &self.payload {
            Payload::Literal(_) => (),
            Payload::Operator(_, packets) => {
                for packet in packets {
                    sum += packet.version_sum();
                }
            }
        }
        sum
    }

    fn value(&self) -> u64 {
        match &self.payload {
            Payload::Literal(n) => *n,
            Payload::Operator(opcode, packets) => {
                let values = packets.iter().map(Packet::value).collect::<Vec<_>>();
                match opcode {
                    Opcode::Sum => values.iter().sum(),
                    Opcode::Multiply => values.iter().fold(1, |acc,p| acc * p),
                    Opcode::Min => *values.iter().min().unwrap(),
                    Opcode::Max => *values.iter().max().unwrap(),
                    Opcode::GreaterThan => if values[0] > values[1] { 1 } else { 0 }
                    Opcode::LowerThan => if values[0] < values[1] { 1 } else { 0 }
                    Opcode::Equals => if values[0] == values[1] { 1 } else { 0 }
                }
            }
        }
    }
}

fn main() {
    let packets = read_input();
    dbg!(&packets);
    
    println!("Expr: {}", packets[0]);
    println!("Version sum: {}", packets[0].version_sum());
    println!("Value: {}", packets[0].value());
}
