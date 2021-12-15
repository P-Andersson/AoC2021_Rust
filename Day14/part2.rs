use std::fs;
use std::cmp;
use std::collections::HashMap;

use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day14/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

type Pairs = (char, char);

type Elements = HashMap<Pairs, i64>;

#[derive(Debug)]
struct PairInsertionRule {
    pair: (char, char),
    insertee: char,
}

impl PairInsertionRule {
    pub fn apply(&self, source: &Elements, dest: &mut Elements) -> bool {
        let quant = *source.get(&self.pair).unwrap_or(&0);
        if  quant > 0 {
            dest.insert(self.pair, cmp::max(0, dest.get(&self.pair).unwrap_or(&1) - quant));
            let new1 = (self.pair.0, self.insertee);
            dest.insert(new1, dest.get(&new1).unwrap_or(&0) + quant);
            let new2 = (self.insertee, self.pair.1);
            dest.insert(new2, dest.get(&new2).unwrap_or(&0) + quant);
           
            return true;
        }
        return false;
    }
}

fn parse(data: &String) -> (Elements, Vec<PairInsertionRule>) {
    let rule_re = Regex::new(r"^(\w\w) -> (\w)$").unwrap();
    
    let mut elements: Elements = Elements::new();    
    let mut rules: Vec<PairInsertionRule> = Vec::new();   
    
    let mut parsing_template = true;

    let mut last_c: Option<char> = None;
    for l in data.lines() {
        if l.len() > 0 {
            if parsing_template {
                for c in l.chars() {
                    if last_c.is_some() {
                        let key = (last_c.unwrap(), c);
                        elements.insert(key, elements.get(&key).unwrap_or(&0) + 1);
                    }
                    last_c = Some(c);
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
    return (elements, rules);
}

fn main() {
    let inp = read_input();
    let (mut polymer, rules) = parse(&inp);
    

    println!("Template: {:?}", polymer);

    for step in 0 .. 40 {
        println!("Step: {}", step + 1);
        let mut next_polymer = polymer.clone();
        for rule in &rules {
            rule.apply(&polymer, &mut next_polymer);
        }
        println!("State: {:?}", next_polymer);
        polymer = next_polymer.clone();

    }


    let mut quanitities: HashMap<char, i64> = HashMap::new();

    {
        for (key, val) in polymer {
            quanitities.insert(key.0, quanitities.get(&key.0).unwrap_or(&0) + val);
        }
    }

    println!("Quants: {:?}", quanitities);

    
    let least = quanitities.values().min().unwrap();
    let most = quanitities.values().max().unwrap();
    
    println!("Most common - least common (+/- 1)= {}", most - least);
}