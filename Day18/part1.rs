use std::fs;
use std::iter;

use std::fmt;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day18/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Eq)] #[derive(PartialEq)] #[derive(Clone)]
enum SnailfishNumber {
    Number(i32),
    Pair(Box<NumberPair>),
}

impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailfishNumber::Number(val) => f.write_str(format!("{}", val).as_str()),
            SnailfishNumber::Pair(pair) => pair.fmt(f),
        }
    }
}


struct Explosion {
    left: i32,
    right: i32,
}

impl SnailfishNumber {
    fn propogate_from_right(&mut self, numb: i32) -> i32{
        match self {
            SnailfishNumber::Number(v) => {
                *self = SnailfishNumber::Number(*v + numb);
                return 0;
            },
            SnailfishNumber::Pair(pair) => {
                return pair.right.propogate_from_right(numb);
            }  
        };
    }

    fn propogate_from_left(&mut self, numb: i32) -> i32 {
        match self {
            SnailfishNumber::Number(v) => {
                *self = SnailfishNumber::Number(*v + numb);
                return 0;
            },
            SnailfishNumber::Pair(pair) => {
                return pair.left.propogate_from_left(numb);
            }  
        };
    }

    fn explode(&mut self, depth: i32) -> Option<Explosion> {
        match self {
            SnailfishNumber::Pair(pair) => {
                if depth >= 4 {
                    let left = match pair.left { SnailfishNumber::Number(val) => Some(val), _ => None };
                    let right = match pair.right { SnailfishNumber::Number(val) => Some(val), _ => None };
                    
                    if [left, right].iter().all(|v| v.is_some() ) {
                        *self = SnailfishNumber::Number(0);
                        return Some(Explosion{left: left.unwrap(), right: right.unwrap()});
                    }
                }
                return pair.explode(depth + 1);
            }
            _ => { return None; }
        }

    }

    fn split(&mut self) -> bool {
        match self {
            SnailfishNumber::Pair(pair) => {
                return pair.split();
            }
            SnailfishNumber::Number(val) => {
                if *val >= 10 {
                    *self = SnailfishNumber::Pair(NumberPair::new(
                        SnailfishNumber::Number((*val)/2),
                        SnailfishNumber::Number((*val)/2 + (*val)%2)));
                    return true;
                }
                return false;
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            SnailfishNumber::Pair(pair) => {
                return pair.left.magnitude() * 3 + pair.right.magnitude() * 2; 
            }
            SnailfishNumber::Number(val) => {
                return *val as i64;
            }
        }
    }


    fn reduce(&self) -> SnailfishNumber {
        let mut new_numb = self.clone();

        let mut any_action = true;
        //println!("Reducing: {:?}", new_numb);
        while any_action {
            any_action = false;
            while new_numb.explode(0).is_some() {
                any_action = true;
                //println!("After Explode: {:?}", new_numb);
            }
            if new_numb.split() {
                any_action = true;
                //println!("After Split: {:?}", new_numb);
            }
        }

        return new_numb;
    }

    fn add(&self, other: &SnailfishNumber) -> SnailfishNumber {
        return SnailfishNumber::Pair(NumberPair::new(
            self.clone(), 
            other.clone()));
    }
}

#[derive(Eq)] #[derive(PartialEq)] #[derive(Clone)]
struct NumberPair {
    left: SnailfishNumber,
    right: SnailfishNumber,
}

impl fmt::Debug for NumberPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[").unwrap();
        self.left.fmt(f).unwrap();
        f.write_str(",").unwrap();
        self.right.fmt(f).unwrap();
        f.write_str("]").unwrap();
        return fmt::Result::Ok(());
    }
}

impl NumberPair {
    fn new(left: SnailfishNumber, right: SnailfishNumber) -> Box<NumberPair> {
        return Box::new(NumberPair{left: left, right: right});
    }

    fn explode(&mut self, depth: i32) -> Option<Explosion> {
        let lresult = self.left.explode(depth);
        if lresult.is_some() {
            let mut exp = lresult.unwrap();
            exp.right = self.right.propogate_from_left(exp.right);
            return Some(exp);
        }

        let rresult = self.right.explode(depth);
        if rresult.is_some() {
            let mut exp = rresult.unwrap();
            exp.left = self.left.propogate_from_right(exp.left);
            return Some(exp);
        }

        return None;
    }

    fn split(&mut self) -> bool {
        let lresult = self.left.split();
        if lresult { return lresult };
        let rresult = self.right.split();
        return rresult;

    }
}

fn parse_pair<I>(iter: &mut iter::Peekable<I>) -> SnailfishNumber
where
    I: Iterator<Item = char>,
{
    let numb1 = parse_snailfish_number(iter);
    assert!(iter.next().unwrap() == ',');
    let numb2 = parse_snailfish_number(iter);
    assert!(iter.next().unwrap() == ']');
    return SnailfishNumber::Pair(NumberPair::new(numb1, numb2));
}

fn parse_single<I>(iter: &mut iter::Peekable<I>) -> SnailfishNumber 
where
    I: Iterator<Item = char>,
{
    let mut result = 0;
    while ('0' ..= '9').contains(iter.peek().unwrap_or(&' ')) {
        result *= 10;
        result += ((iter.next().unwrap() as u8) - '0' as u8) as i32;
    }
    return SnailfishNumber::Number(result);
}

fn parse_snailfish_number<I>(iter: &mut iter::Peekable<I>) -> SnailfishNumber 
where
    I: Iterator<Item = char>,
{
    if *iter.peek().unwrap() == '[' {
        iter.next();
        return parse_pair(iter);
    }
    return parse_single(iter);
}

fn parse(data: &String) -> Vec<SnailfishNumber> {
    let mut result = Vec::new();
    for line in data.lines() {
        result.push(parse_snailfish_number(&mut line.chars().peekable()));
    }
    return result;
}

fn main() {
    let inp = read_input();
    let numbers = parse(&inp);
    
    let mut ite = numbers.iter();
    let mut cur = ite.next().unwrap().clone().reduce();
    for number in ite {
        let reduced = number.reduce();
        let orig = cur.clone();
        let addee = cur.add(&reduced);
        cur = addee.reduce();
        //println!("{:?} + {:?} = \n{:?}", orig, number, cur);
    }
    println!("End: {:?}", cur);
    println!("Masgnitude: {:?}", cur.magnitude());

}
