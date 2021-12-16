use std::fs;
use std::u8;
use std::mem;
use std::cmp;

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
enum OpType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)] #[derive(Clone)]
enum PacketData {
    Literal(u64),
    Operator((OpType, usize)),
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

fn parse_operator(type_id: u64, data: &BitData, index: &mut usize) -> (PacketData, Vec<Packet>) {
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

    let op_type = match type_id {
        0 => OpType::Sum,
        1 => OpType::Product,
        2 => OpType::Minimum,
        3 => OpType::Maximum,
        5 => OpType::GreaterThan,
        6 => OpType::LessThan,
        7 => OpType::EqualTo,
        _ => panic!(),
    };

    return (PacketData::Operator((op_type, immediate_subpackets)), subpackets);
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
        0 | 1 | 2 | 3 | 5 | 6 | 7 => {
            let (data, mut subpackets) = parse_operator(type_id, data, index);
            let mut res = [Packet{version: version as u8, type_id: type_id as u8, data: data}].to_vec();
            res.append(&mut subpackets);
            res
        },
        _ => panic!("Unknown Type ID: {}", type_id)
    }
}

fn crunch_packets(packets: &Vec<Packet>) -> i64 {
    let mut packet_stack = packets.clone();
    let mut work_stack = Vec::new();
    
    while packet_stack.len() > 0 {
        let packet = packet_stack.pop().unwrap();
        match packet.data {
            PacketData::Literal(val) => {
                work_stack.push(val as i64);
            }
            PacketData::Operator((op_type, packet_count)) => {
                match op_type {
                    OpType::Sum => {
                        let mut sum = 0;
                        for _ in 0 .. packet_count {
                            sum += work_stack.pop().unwrap();
                        }
                        work_stack.push(sum);
                    }
                    OpType::Product => {
                        let mut prod = 1;
                        for _ in 0 .. packet_count {
                            prod *= work_stack.pop().unwrap();
                        }
                        work_stack.push(prod);
                    }
                    OpType::Minimum => {
                        let mut min = i64::MAX;
                        for _ in 0 .. packet_count {
                            min = cmp::min(min, work_stack.pop().unwrap());
                        }
                        work_stack.push(min);
                    }
                    OpType::Maximum => {
                        let mut max = i64::MIN;
                        for _ in 0 .. packet_count {
                            max = cmp::max(max, work_stack.pop().unwrap());
                        }
                        work_stack.push(max);
                    }
                    OpType::GreaterThan => {
                        let first = work_stack.pop().unwrap();
                        let second = work_stack.pop().unwrap();
                        work_stack.push( if first > second {1} else {0});
                    }
                    OpType::LessThan => {
                        let first = work_stack.pop().unwrap();
                        let second = work_stack.pop().unwrap();
                        work_stack.push( if first < second {1} else {0});
                    }
                    OpType::EqualTo => {
                        let first = work_stack.pop().unwrap();
                        let second = work_stack.pop().unwrap();
                        work_stack.push( if first == second {1} else {0});
                    }
                }
            }
        }

    }
    return work_stack.pop().unwrap();
}


fn main() {
    let inp = read_input();
    let data = parse(&inp);
    
    let packets = parse_packet(&data, &mut 0);
    //println!("{:?}", packets);

    println!("Outer Value: {}", crunch_packets(&packets));
}