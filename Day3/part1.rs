use std::fs;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day3/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

fn parse(data: String) -> (Vec<Vec<u8>>, usize) {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for elem in data.split("\n") {
        let mut loc_result: Vec<u8> = Vec::new();
        for c in elem.chars() {
            if c == '1' {
                loc_result.push(1);
            } else
            {
                loc_result.push(0);
            }
        }
        result.push(loc_result);
    }
    let length = result[0].len();
    return (result, length);
}

fn main() {
    let inp = read_input();
    let parsed = parse(inp);
    let strings = parsed.0;
    let bitcount = parsed.1;
    
    let mut ones_count: Vec<usize> = Vec::new();
    ones_count.resize(bitcount, 0);

    for bit_string in &strings {
        for i in 0 .. bitcount {
            if bit_string[i] == 1 {
                ones_count[i] += 1;
            }
        }
    }

    let mut gamma = 0u32;
    for i in 0 .. bitcount {
        if ones_count[i] >= strings.len()/2 {
            gamma |= 1 << bitcount - i - 1;
        }
    }
    let mut mask = 0u32;
    for i in 0 .. bitcount {
        mask |= 1 << i;
    }
    let epsilon = !gamma & mask;
    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    println!("Power Consumption: {}", gamma * epsilon);
}