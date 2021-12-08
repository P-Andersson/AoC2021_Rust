use std::fs;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day6/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

fn parse(data: String) -> Vec<u64> {
    let mut cycles: Vec<u64> = Vec::new();
    for elem in data.split(",") {
        cycles.push(elem.parse::<u64>().expect(&format!("Unparsable: {}", elem)));
    }
    return cycles;
}

fn main() {
    let MAX_CYCLES = 8;

    let inp = read_input();
    let parsed = parse(inp);

    let mut cycles: Vec<u64> = Vec::new();
    for i in 0 .. MAX_CYCLES+1 {
        cycles.push(0);
    }
    for cycles_left in parsed {
        cycles[cycles_left as usize] += 1;
    }

    for day in 0 .. 80 {
        let resets = cycles[0];
        for i in 1 .. cycles.len() {
            cycles[i - 1] = cycles[i];
        }
        cycles[6] += resets;
        cycles[8] = resets;
    }

    println!("Fish: {}", cycles.iter().sum::<u64>());    
}