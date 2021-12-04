use std::fs;

struct SubmarineState {
    x: i32,
    y: i32,    
}

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day2/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}



fn parse(data: String) -> SubmarineState{
    let mut state = SubmarineState{x: 0, y: 0};
    for elem in data.split("\n") {
        let res: Vec<&str> = elem.split(" ").collect();
        let operation = res[0];
        let mag = res[1].parse::<i32>().expect(
            &format!("Unparsable Magnitude:  \"{}\"", res[1]));
        
        match operation {
            "forward" => state.x += mag,
            "down" => state.y += mag,
            "up" => state.y -= mag,
            _ => panic!("Unknown Op: \"{}\"", operation),

        }

    }
    return state;
}

fn main() {
    let inp = read_input();
    let parsed = parse(inp);
    
    println!("Final State: X: {}, Y: {}", parsed.x, parsed.y);
    println!("Res: {}", parsed.x * parsed.y);
}