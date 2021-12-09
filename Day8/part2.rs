use std::fs;
use std::collections::HashMap;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day8/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}


struct Entry7SegSignals<'a> {
    inputs : Vec<&'a[u8]>,
    ouputs : Vec<&'a[u8]>,
}

impl Entry7SegSignals<'_> {
    pub fn compute_outputs(&self) -> u32{
        
        let structures = Entry7SegSignals::identify_digits(&self.inputs);

        let mut sum = 0u32;
        let mut factor = 1u32;
        for output in self.ouputs.iter().rev() {
            for (val, structure) in structures.clone() {
                if output.len() == structure.len() && 
                   Entry7SegSignals::contains_all(&output.to_vec(), &structure) {
                    sum += (val as u32) * factor;
                    factor *= 10;
                    break;
                }
            }
        }
       
        return sum;
    }

    fn difference(left: &Vec<u8>, right: &[u8]) -> Vec<u8> {
        return left.iter().filter(| signal | !right.contains(signal)).cloned().collect();
    }

    fn contains_all(l: &Vec<u8>, r: &Vec<u8>) -> bool {
        for c in r {
            if !l.contains(c) {
                return false;
            }
        }
        return true;
    }

    fn identify_digits(inputs: &Vec<&[u8]>) -> HashMap<u8, Vec<u8>> {
        let mut structures: HashMap<u8, Vec<u8>> = HashMap::new();
        
        // Basic ones
        for input in inputs {
            if input.len() == 2 {
                structures.insert(1, input.to_vec());
            } else if input.len() == 4 {
                structures.insert(4, input.to_vec());
            } else if input.len() == 3 {
                structures.insert(7, input.to_vec());
            } else if input.len() == 7 {
                structures.insert(8, input.to_vec());
            }
        }

        // Five segments
        for input in inputs {
            if input.len() == 5 {
                if Entry7SegSignals::contains_all(&input.to_vec(), &structures[&1]) {
                    structures.insert(3, input.to_vec());
                } else if Entry7SegSignals::contains_all(&input.to_vec(), 
                            &Entry7SegSignals::difference(&structures[&4], &structures[&1])) {
                    structures.insert(5, input.to_vec());
                } else {
                    structures.insert(2, input.to_vec());
                }
            }
        }
        // Six segments
        for input in inputs {
            if input.len() == 6 {
                if Entry7SegSignals::contains_all(&input.to_vec(), &structures[&4]) {
                    structures.insert(9, input.to_vec());
                } else if Entry7SegSignals::contains_all(&input.to_vec(), &structures[&7]) {
                    structures.insert(0, input.to_vec());
                } else {
                    structures.insert(6, input.to_vec());
                }
            }
        }

        return structures;
    }
}

fn parse<'a>(data: &'a String) -> Vec<Entry7SegSignals<'a>> {
    let mut result: Vec<Entry7SegSignals> = Vec::new();
    for elem in data.split("\n") {
        let inputs_ouputs = elem.split_once('|').unwrap();
        let inputs = inputs_ouputs.0.trim().split(' ').map(|s| s.as_bytes()).collect();
        let outputs = inputs_ouputs.1.trim().split(' ').map(|s| s.as_bytes()).collect();
        result.push(Entry7SegSignals{inputs: inputs, ouputs: outputs});
    }
    return result;
}


fn main() {
    let inp = read_input();
    let parsed = parse(&inp);

    let mut sum = 0;
    for entry in parsed {
        sum += entry.compute_outputs();
    }   
    println!("Output sum: {}", sum);    
}