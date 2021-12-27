use std::fs;
use std::fmt;
use std::cmp;

use std::collections::HashMap;
use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day24/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Debug)] #[derive(Hash)] #[derive(PartialEq)] #[derive(Eq)] #[derive(Copy)] #[derive(Clone)]
enum Variable {
    X,
    Y,
    Z,
    W,
}

type AluInteger = i64;

#[derive(Debug)] #[derive(Clone)] #[derive(Copy)]
enum AluDatum {
    Var(Variable),
    Literal(AluInteger),
}

#[derive(Debug)] #[derive(Clone)] #[derive(Copy)]
enum Instruction {
    Inp(Variable),
    Add(Variable, AluDatum),
    Mul(Variable, AluDatum),
    Div(Variable, AluDatum),
    Mod(Variable, AluDatum),
    Eql(Variable, AluDatum),
}

type Program = Vec<Instruction>;

type Inspector = fn (Instruction, &Alu);

struct Alu {
    variables : HashMap<Variable, AluInteger>,
    inspectors : Vec<Inspector>,
}

impl fmt::Debug for Alu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Alu")
         .field("x: ", &self.variables.get(&Variable::X))
         .field("y: ", &self.variables.get(&Variable::Y))
         .field("z: ", &self.variables.get(&Variable::Z))
         .field("w: ", &self.variables.get(&Variable::W))
         .finish()
    }
}

impl Alu {
    fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert(Variable::X, 0);
        vars.insert(Variable::Y, 0);
        vars.insert(Variable::Z, 0);
        vars.insert(Variable::W, 0);

        return Alu{variables: vars, inspectors: Vec::new()};
    }

    fn run(&mut self, program: &Program, inputs: Vec<i64>, verbose: bool) -> Result<(),()>{
        fn read(alu: &Alu, var: &Variable) -> AluInteger {
            *alu.variables.get(&var).unwrap()
        }
        fn value(alu: &Alu, datum: &AluDatum) -> AluInteger {
            match datum {
                AluDatum::Literal(val) => *val,
                AluDatum::Var(var) => read(alu, var),
            }            
        }
        

        let mut input_index = 0;

        for instr in program {
            for i in &self.inspectors {
                i(*instr, self);
            }
            match instr {
                Instruction::Inp(var) => { 
                    if input_index >= inputs.len() {
                        return Err(());
                    }
                    self.variables.insert(*var, inputs[input_index]); input_index += 1; 
                    if verbose {
                        println!("Got Input: {} ({})", input_index, inputs[input_index-1]);
                    }
                },
                Instruction::Add(var, datum) => { self.variables.insert(*var, read(self, var) + value(self, datum)); },
                Instruction::Mul(var, datum) => { self.variables.insert(*var, read(self, var) * value(self, datum)); },
                Instruction::Div(var, datum) => { self.variables.insert(*var, read(self, var) / value(self, datum)); },
                Instruction::Mod(var, datum) => { self.variables.insert(*var, read(self, var) % value(self, datum)); },
                Instruction::Eql(var, datum) => { self.variables.insert(*var, if read(self, var) == value(self, datum) {1} else {0}); },
            };

        }
        return Ok(());
    }
}


fn parse(data: &String) -> Program {
    let instruction_re = Regex::new(r"(?m)^(\S+) (\S+)\s*(\S+)?$").unwrap();

    fn as_var(i: &str) -> Result<Variable, ()> {
        match i {
            "x" => Ok(Variable::X),
            "y" => Ok(Variable::Y),
            "z" => Ok(Variable::Z),
            "w" => Ok(Variable::W),
            _ => Err(()),
        }
    }

    fn as_datum(i: &str) -> Result<AluDatum, ()> {
        let var = as_var(i);
        if var.is_ok() {
            return Ok(AluDatum::Var(var.unwrap()));
        }
        let parsed = i.parse::<AluInteger>();
        if parsed.is_ok() {
            return Ok(AluDatum::Literal(parsed.unwrap()));
        }
        return Err(());
    }

    let mut program = Program::new();

    for cap in instruction_re.captures_iter(data) {
        let instr = cap.get(1).unwrap().as_str();
        match instr {
            "inp" => { program.push(Instruction::Inp(as_var(cap.get(2).unwrap().as_str()).unwrap())); },
            "add" => { program.push(Instruction::Add(as_var(cap.get(2).unwrap().as_str()).unwrap(),
                                                     as_datum(cap.get(3).unwrap().as_str()).unwrap()));
                                                    },
            "mul" => { program.push(Instruction::Mul(as_var(cap.get(2).unwrap().as_str()).unwrap(),
                                                    as_datum(cap.get(3).unwrap().as_str()).unwrap()));
                                                    },
            "div" => { program.push(Instruction::Div(as_var(cap.get(2).unwrap().as_str()).unwrap(),
                                                    as_datum(cap.get(3).unwrap().as_str()).unwrap()));
                                                    },
            "mod" => { program.push(Instruction::Mod(as_var(cap.get(2).unwrap().as_str()).unwrap(),
                                                    as_datum(cap.get(3).unwrap().as_str()).unwrap()));
                                                    },
            "eql" => { program.push(Instruction::Eql(as_var(cap.get(2).unwrap().as_str()).unwrap(),
                                                    as_datum(cap.get(3).unwrap().as_str()).unwrap()));
                                                    },
            _ => panic!(),
                     
        };
    }
    return program;
}
// Note: Symbols go from 1-9, not 0-8. 1 is treated as 0 internally
type Base9Integer = i64;

fn from_string(s: &str) -> Base9Integer {
    let mut val: Base9Integer = 0;
    for c in s.chars() {
        let digit = (c as i64) - '1' as i64;
        val *= 9;
        val += digit;
    }
    return val;
}

fn to_string(v: Base9Integer) -> String {
    let mut string = String::new();
    let mut divisor = 1;
    while divisor == 1 || divisor <= v {
        let val = (v / divisor) % 9;
        string.insert(0, (('1' as u8) + val as u8) as char);
        divisor *= 9;
    }
    return string;
}

fn to_digits(v: Base9Integer, pad_to: usize) -> Vec<i64> {
    let mut digits = Vec::new();
    let mut divisor = 1;
    while divisor == 1 || divisor <= v {
        let val = (v / divisor) % 9;
        digits.insert(0, 1 + val);
        divisor *= 9;
    }
    while digits.len() < pad_to {
        digits.insert(0, 1);
    }
    return digits;
}



fn peeker(instr: Instruction, alu: &Alu) {
    match instr {
        Instruction::Add(Variable::Z, AluDatum::Var(var)) => { 
            println!("z += {:?}", alu.variables.get(&var).unwrap()); 
            //println!("Pushed = {:?}", alu.variables.get(&Variable::Z).unwrap() % 26); 
        },
        Instruction::Add(Variable::X, AluDatum::Literal(val)) => { 
            println!("Constant: {:?}", val); 
            //println!("Pushed = {:?}", alu.variables.get(&Variable::Z).unwrap() % 26); 
        },
        Instruction::Mod(Variable::X, AluDatum::Literal(26)) => { 
            println!("Peeked: {:?}", alu.variables.get(&Variable::X).unwrap() % 26); 
            //println!("Pushed = {:?}", alu.variables.get(&Variable::Z).unwrap() % 26); 
        },
        Instruction::Div(Variable::Z, AluDatum::Literal(26)) => { 
            println!("Pop!"); 
            //println!("Pushed = {:?}", alu.variables.get(&Variable::Z).unwrap() % 26); 
        },
        Instruction::Eql(Variable::X, AluDatum::Var(Variable::W)) => { 
            let x = alu.variables.get(&Variable::X).unwrap();
            let w = alu.variables.get(&Variable::W).unwrap();
            println!("x {:?} == w {:?}", x, w); 
            println!("Multipl: {}", if x == w {0} else {1} ); 
        }
        _ => {}
    }
}


/* Hand decompoliation
for digit in 1 ..= 14 {
    let inp = read();
    let x = peek_or_0();
    match digit {
        1 =>  { if inp != x + 11  { push(inp + 6} } }
        2 =>  { if inp != x + 11  { push(inp + 12} } }
        3 =>  { if inp != x + 15  { push(inp + 8} } }
        4 =>  { pop() if inp != x + -11  { push(inp + 7 } } }       
        5 =>  { if inp != x + 15  { push(inp + 7} } }
        6 =>  { if inp != x + 15  { push(inp + 12} } }
        7 =>  { if inp != x + 14  { push(inp + 2} } }
        8 =>  { pop()  if inp != x + -7  { push(inp + 15) } }
        9 =>  { if inp != x + 12  { push(inp + 4} } }
        10 =>  { pop() if inp != x + -6  { push(inp + 5} } }
        11 =>  { pop() if inp != x + -10  { push(inp + 12} } }
        12 =>  { pop() if inp != x + -15  { push(inp + 11} } }
        13 =>  { pop() if inp != x + -9  { push(inp + 13} } }
        14 =>  { pop() if inp != x + 0  { push(inp + 7} } }
    }    

    Only digit 4, 8, 10, 11, 12, 13 and 14 can be conditionally disabled
    Depends (Given pops):
    4 - 3 (disabling range for Dig 3: 4 - 9) (Matches 1 to 6)
    8 - 7 (disabling range for Dig 7: 6 - 9) (Matches 1 to 4)
    10 - 9  (disabling range for Dig 9: 3 - 9) (Matches 1 to 7)
    11 - 6  (disabling range for Dig 6: 1 - 7) (Matches 3 to 9)
    12 - 5 (diabling range for Dig 5: 9) (matches 1)
    13 - 2 (disabling range for Dig 2: 1 to 6) (matches 4 to 9)
    14 - 1 (disabling range for Dig 1: 1 to 3) (matches 7 to 9)


    Last 5 are a mess due to pops

*/


fn main() {
    let inp = read_input();
    let program = parse(&inp);


    // Ranges constructed manually above, manually build small number and evaluate
    for d1 in [from_string("11419161313147")] {
        let mut alu = Alu::new();
        alu.inspectors.push(peeker);
        alu.run(&program, to_digits(d1, 14), true);
        println!("Digit: {} Vars: {:?}", to_string(d1), alu);  
        // Disect stack
        while *alu.variables.get(&Variable::Z).unwrap() != 0 {
            let z = *alu.variables.get(&Variable::Z).unwrap();
            println!("Stack: {}", z % 26);
            alu.variables.insert(Variable::Z, z / 26);

        }
      
    }
    //search_spans(&program);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base9conversion() {
        assert_eq!(to_string(from_string("3214")), "3214");

        assert_eq!(to_string(from_string("99999999999999")), "99999999999999");
    }

    #[test]
    fn test_base9conversion_to_digits() {
        assert_eq!(to_digits(from_string("3214"), 4), [3, 2, 1, 4]);
    }
}