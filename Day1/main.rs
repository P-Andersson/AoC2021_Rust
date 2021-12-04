use std::fs;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day1/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

fn parse(data: String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for elem in data.split("\n") {
        result.push(elem.parse::<i32>().expect(
            &format!("Unparsable:  {}", elem)));
    }
    return result;
}

fn main() {
    let inp = read_input();
    let parsed = parse(inp);

    let mut increments: i32 = 0;

    for i in 1..parsed.len() {
        if parsed[i-1] < parsed[i] {
            increments += 1;
        }
    }

    println!("{:?}", parsed);    
    println!("Increments: {}", increments);
}