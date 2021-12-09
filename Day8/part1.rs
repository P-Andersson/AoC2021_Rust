use std::fs;

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
    pub fn count_vals(&self, val: u8) -> u32 {
        match val {
            1 => self.ouputs.iter().filter(| outp | outp.len() == 2).map(| outp| 1u32).sum(),
            4 => self.ouputs.iter().filter(| outp | outp.len() == 4).map(| outp| 1u32).sum(),
            7 => self.ouputs.iter().filter(| outp | outp.len() == 3).map(| outp| 1u32).sum(),
            8 => self.ouputs.iter().filter(| outp | outp.len() == 7).map(| outp| 1u32).sum(),
            _ => 0
        }
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

    let mut easy_digits = 0;
    for entry in parsed {
        easy_digits += entry.count_vals(1);
        easy_digits += entry.count_vals(4);
        easy_digits += entry.count_vals(7);
        easy_digits += entry.count_vals(8);
    }   
    println!("Easy Digits: {}", easy_digits);    
}