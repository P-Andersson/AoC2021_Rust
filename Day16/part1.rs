use std::fs;
use std::u8;
use std::mem;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day16/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Debug)]
struct BitData {
    data : Vec<u8>,
}

impl BitData {
    pub fn get(&self, bit_index: usize, bits: usize) -> u64 {
        let mut res = 0;
        
        for bit in bit_index .. (bit_index + bits) {
            let byte_index = bit / 8;
            let subindex =  7 - (bit % 8);

            let bit_value = (self.data[byte_index] >> subindex) & 1;
            res |= (bit_value as u64) << ((bits - (bit - bit_index)) - 1);
        }

        return res;
    }
}

fn parse_hex(s: &str) -> BitData{
    let mut data = s.as_bytes().iter().map( |b| match b {
        b'0' ..= b'9' => b - b'0',
        b'A' ..= b'F' => b - b'A' + 10,
        _ => panic!(),
    });

    let mut result = Vec::new();
    loop {
        let (high, low) = (data.next(), data.next());
        if high.is_none() {
            break;
        }

        result.push(high.unwrap() << 4 | low.unwrap() << 0);
    }     
    return BitData{data: result};
}

fn parse(data: &String) -> BitData {
     return parse_hex(data);
}

#[derive(Debug)] #[derive(Clone)]
enum PacketData {
    Literal(u64),
    Operator(usize),
}

#[derive(Debug)] #[derive(Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

fn parse_literal(data: &BitData, index: &mut usize) -> PacketData {
    let mut last_group = false;
    let mut read = 0;

    let mut res: u64 = 0;
    while !last_group {
        last_group = data.get(*index, 1) == 0;
        *index += 1;
        let subval = data.get(*index, 4);
        *index += 4;
        read += 4;
        res <<= 4;
        res |= subval;
    }
    if read as usize > mem::size_of::<u64>() * 8 {
        panic!();
    } 
    *index += read % 4;

    return PacketData::Literal(res);
}

const LENGTH_MODE_0_SIZE: usize = 15;
const LENGTH_MODE_1_SIZE: usize = 11;

fn parse_operator(data: &BitData, index: &mut usize) -> (PacketData, Vec<Packet>) {
    let length_type_id = data.get(*index, 1);
    *index += 1;

    let mut immediate_subpackets = 0;
    let mut subpackets = Vec::new();
    if length_type_id == 0 {
        let terminate_index = data.get(*index, LENGTH_MODE_0_SIZE) as usize + *index + LENGTH_MODE_0_SIZE;
        *index += LENGTH_MODE_0_SIZE;
        while *index < terminate_index {
            subpackets.append(&mut parse_packet(data, index));
            immediate_subpackets += 1;
        }
        if *index > terminate_index {
            panic!();
        }
    }
    else if length_type_id == 1 {
        let package_count = data.get(*index, LENGTH_MODE_1_SIZE) as usize;
        *index += LENGTH_MODE_1_SIZE;
        for _package in 0 .. package_count {
            subpackets.append(&mut parse_packet(data, index));
            immediate_subpackets += 1;
        }
    } else {
        panic!();
    }

    return (PacketData::Operator(immediate_subpackets), subpackets);
}

fn parse_packet(data: &BitData, index: &mut usize) -> Vec<Packet> {
    let version = data.get(*index, 3);
    *index += 3;
    let type_id = data.get(*index, 3);
    *index += 3;

    return match type_id {
        4 => {
            let data = parse_literal(data, index);
            [Packet{version: version as u8, type_id: type_id as u8, data: data}].to_vec()
        },
        _ => {
            let (data, mut subpackets) = parse_operator(data, index);
            let mut res = [Packet{version: version as u8, type_id: type_id as u8, data: data}].to_vec();
            res.append(&mut subpackets);
            res
        },
        //_ => panic!("Unknown Type ID: {}", type_id)
    }
}

fn main() {
    let inp = read_input();
    let data = parse(&inp);
    
    let packets = parse_packet(&data, &mut 0);
    //println!("{:?}", packets);

    println!("Version Number Sum: {}", packets.iter().map(|p| p.version as u64).sum::<u64>());
}