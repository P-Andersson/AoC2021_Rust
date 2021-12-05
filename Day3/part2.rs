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

fn compute_most_common(data: &Vec<Vec<u8>>, bitcount: usize) -> Vec<Option<u8>> {
    let mut balances: Vec<i32> = Vec::new();
    balances.resize(bitcount, 0);
   
    for bit_string in data {
        for i in 0 .. bitcount {
            if bit_string[i] == 1 {
                balances[i] += 1;
            } else {
                balances[i] -= 1;
            }
        }
    }

    let mut result: Vec<Option<u8>> = Vec::new();
    result.resize(bitcount, None);

    for i in 0 .. bitcount {
        if balances[i] > 0 {
            result[i] = Some(1);
        }
        else if balances[i] < 0 {
            result[i] = Some(0);
        }
    }

    return result;
}

type CriteriaF = fn(val: u8, most_common: Option<u8>) -> bool;

fn compute_with_criteria(data: &Vec<Vec<u8>>, bitindex: usize, criteria: CriteriaF) -> u32 {
    let bitcount = data[0].len();

    if data.len() == 1 {
        let mut result = 0u32;
        for i in 0 .. bitcount {
            if data[0][i] == 1 {
                result |= 1 << bitcount - i - 1;
            }
        }
        return result;
    }

    let mut new_candidates: Vec<Vec<u8>> = Vec::new();

    let most_common = compute_most_common(data, bitcount);
    for candidate in data {
        if criteria(candidate[bitindex], most_common[bitindex]) {
            new_candidates.push(candidate.clone());
        }
    }
    
    return compute_with_criteria(&new_candidates, bitindex + 1, criteria);
}

fn main() {
    let inp = read_input();
    let parsed = parse(inp);
    let strings = parsed.0;
    let bitcount = parsed.1;
    
    let most_common = compute_most_common(&strings, bitcount);

    let oxygen_rating = compute_with_criteria(&strings, 0, 
        |val, most_common| Some(val) == most_common || (most_common == None && val == 1) 
    );
    let scrubber_rating = compute_with_criteria(&strings, 0, 
        |val, most_common|(most_common != None && Some(val) != most_common) || (most_common == None && val == 0) 
    );


    let mut gamma = 0u32;
    for i in 0 .. bitcount {
        if most_common[i] == Some(1) {
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
    println!("Oxy. Rating: {}, Scrubber Rating: {}", oxygen_rating, scrubber_rating);
    println!("Life Support Rating: {}", oxygen_rating * scrubber_rating);
}