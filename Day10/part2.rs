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

    let mut scores: Vec<i64> = Vec::new();
    'lineloop : for line in lines {
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
                        continue 'lineloop;
                    } else if expected != Some(c) {
                        continue 'lineloop;
                    }
                }
            } 
        }
        let mut score = 0;
        while terminator_stack.len() > 0 {
            let missing = terminator_stack.pop().unwrap();
            score = score * 5 + match missing {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Weird error"),
            }
        } 
        scores.push(score);
    }
    
    scores.sort();

    println!("Error score: {}", scores[scores.len()/2]);    
}