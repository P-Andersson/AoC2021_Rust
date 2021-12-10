use std::fs;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day10/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

fn parse(data: &String) -> Vec<&str> {
    return data.lines().collect();
}

fn main() {
    let inp = read_input();
    let lines = parse(&inp);

    let mut syntax_errors: Vec<char> = Vec::new();
    for line in lines {
        let mut terminator_stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' => terminator_stack.push(')'),
                '[' => terminator_stack.push(']'),
                '{' => terminator_stack.push('}'),
                '<' => terminator_stack.push('>'),
                _ => {
                    let expected = terminator_stack.pop();
                    if expected == None {
                        break;
                    } else if expected != Some(c) {
                        syntax_errors.push(c);
                        break;
                    }
                }
            }            
        }
    }
    
    println!("Error score: {}", syntax_errors.iter().map(| c | match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Weird error"),
    }).sum::<i64>());    
}