use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day13/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

type Position = (i32, i32);

#[derive(Debug)]
struct FoldInstruction {
    axis: u8,
    coordinate: i32,
}

fn parse(data: &String) -> (HashSet<Position>, Vec<FoldInstruction>) {
    let pos_re = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let instr_re = Regex::new(r"^fold along (\w)=(\d+)$").unwrap();
    
    let mut positions: HashSet<Position> = HashSet::new();    
    let mut instructions: Vec<FoldInstruction> = Vec::new();   
    
    let mut parsing_positions = true;
    for l in data.lines() {
        if l.len() > 0 {
            if parsing_positions {
                for cap in pos_re.captures_iter(l) {
                    positions.insert(
                        (cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()), 
                    );
                }
            } else {
                for cap in instr_re.captures_iter(l) {
                    let (axis, coordinate) =  (&cap[1], cap[2].parse::<i32>().unwrap()); 
                    let axis_no = match axis {
                        "x" => 0,
                        "y" => 1,
                        _ => panic!("Unknown axis: {}", axis),
                    };
                    instructions.push(FoldInstruction{axis: axis_no, coordinate: coordinate});
                }
            }
        } else {
            parsing_positions = false;
        }
    }
    return (positions, instructions);
}

fn do_fold(positions: &HashSet<Position>, instruction: &FoldInstruction) -> HashSet<Position> {
    let mut new_positions: HashSet<Position> = HashSet::new();    
    for pos in positions {
        let mut new_pos = pos.clone();
        let mut new_axis = match instruction.axis {
            0 => &mut new_pos.0,
            1 => &mut new_pos.1,
            _ => panic!(),
        };
        if *new_axis >= instruction.coordinate {
            *new_axis =  *new_axis - 2*(*new_axis - instruction.coordinate);
        }
        new_positions.insert(new_pos);
    }
    return new_positions;
}

fn render(positions: &HashSet<Position>) {
    let min_x = positions.iter().map(|p| p.0 ).min().unwrap();
    let max_x = positions.iter().map(|p| p.0 ).max().unwrap();
    let min_y = positions.iter().map(|p| p.1 ).min().unwrap();
    let max_y = positions.iter().map(|p| p.1 ).max().unwrap();

    for y in min_y ..= max_y  { 
        for x in min_x ..= max_x {
            if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let inp = read_input();
    let (positions, instructions) = parse(&inp);

    //render(&positions);

    let mut folded = positions;
    for insr in &instructions {
        folded = do_fold(&folded, &insr);   
        //render(&folded);
        println!("Points: {}", folded.len());
        println!("");
    }
    render(&folded);
    
}