use std::fs;
use std::collections::HashMap;

use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day14/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

type Polymer = Vec<char>;

#[derive(Debug)]
struct PairInsertionRule {
    pair: (char, char),
    insertee: char,
}

impl PairInsertionRule {
    pub fn apply(&self, polymer: &mut Polymer, index: &mut usize) -> bool {
        if polymer[*index] == self.pair.0 && polymer[*index + 1] == self.pair.1 {
            polymer.insert(*index+1, self.insertee);
            *index += 1;
            return true;
        }
        return false;
    }
}


fn parse(data: &String) -> (Polymer, Vec<PairInsertionRule>) {
    let rule_re = Regex::new(r"^(\w\w) -> (\w)$").unwrap();
    
    let mut polymer: Polymer = Polymer::new();    
    let mut rules: Vec<PairInsertionRule> = Vec::new();   
    
    let mut parsing_template = true;
    for l in data.lines() {
        if l.len() > 0 {
            if parsing_template {
                for c in l.chars() {
                    polymer.push(c);
                }
            } else {
                for cap in rule_re.captures_iter(l) {
                    let (pair, insertee) =  (&cap[1], &cap[2]); 
                    rules.push(PairInsertionRule{pair: (pair.chars().nth(0).unwrap(), 
                                                        pair.chars().nth(1).unwrap()), 
                                                        insertee: insertee.chars().nth(0).unwrap()});
                }
            }
        } else {
            parsing_template = false;
        }
    }
    return (polymer, rules);
}

fn main() {
    let inp = read_input();
    let (mut polymer, rules) = parse(&inp);

    for step in 0 .. 10 {
        let mut index = 0;
        while index + 1 < polymer.len() {
            for rule in &rules {
                if rule.apply(&mut polymer, &mut index) {
                    break;
                }
            }
            index += 1;
        }
    }

    let mut quanitities: HashMap<char, i64> = HashMap::new();
    for c in polymer {
        quanitities.insert(c, quanitities.get(&c).unwrap_or(&0) + 1);
    }
    
    let least = quanitities.values().min().unwrap();
    let most = quanitities.values().max().unwrap();
    
    println!("Most common - least common = {}", most - least);
}