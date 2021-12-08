use std::fs;
use std::cmp;
use std::collections::HashMap;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day7/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

fn parse(data: String) -> Vec<i64> {
    let mut poses: Vec<i64> = Vec::new();
    for elem in data.split(",") {
        poses.push(elem.parse::<i64>().expect(&format!("Unparsable: {}", elem)));
    }
    return poses;
}

struct TargetCandidate {
    crabs : i64,
    l_fuel : i64,
    r_fuel : i64,
}

fn main() {
    let inp = read_input();
    let parsed = parse(inp);

    let mut target_pos_info: HashMap<i64, TargetCandidate> = HashMap::new();

    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for pos in &parsed {
        min = cmp::min(min, *pos);
        max = cmp::max(max, *pos);
    }

    for pos in parsed{
        if target_pos_info.contains_key(&pos) {
            target_pos_info.get_mut(&pos).unwrap().crabs += 1;
        } else {
            target_pos_info.insert(pos, TargetCandidate{crabs: 1, l_fuel: 0, r_fuel: 0});
        }
    }

    let mut l_fuel = 0;
    let mut l_crabs = 0;
    for pos in min .. max + 1{
        l_fuel += l_crabs;
        if target_pos_info.contains_key(&pos) {
            let mut target = target_pos_info.get_mut(&pos).unwrap();
            target.l_fuel = l_fuel;
            l_crabs += target.crabs;
        } else {
            target_pos_info.insert(pos, TargetCandidate{crabs: 0, l_fuel: l_fuel, r_fuel: 0});
        }
    }

    let mut r_fuel = 0;
    let mut r_crabs = 0;
    for pos in (min .. max + 1).rev(){
        r_fuel += r_crabs;
        let mut target = target_pos_info.get_mut(&pos).unwrap();
        target.r_fuel = r_fuel;
        r_crabs += target.crabs;
    }

    let mut best = i64::MAX;
    for val in target_pos_info.values() {
        best = cmp::min(best, val.l_fuel + val.r_fuel);
    }
    
    println!("Best Fuel: {}", best);    
}