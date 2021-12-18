use std::fs;
use std::cmp;
use std::collections::HashSet;

use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day17/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

type Point = (i32, i32);
type Velocity = (i32, i32);

#[derive(Debug)]
struct Target {
    start: Point,
    end: Point,
}


fn parse(data: &String) -> Target {
    let target_re = Regex::new(r"^target area: x=([-\d]+)\.\.([-\d]+), y=([-\d]+)\.\.([-\d]+)$").unwrap();

    for cap in target_re.captures_iter(data) {
        return Target{start: (cap.get(1).unwrap().as_str().parse:: <i32>().unwrap(),
                              cap.get(4).unwrap().as_str().parse:: <i32>().unwrap()),
                      end:   (cap.get(2).unwrap().as_str().parse:: <i32>().unwrap(),
                              cap.get(3).unwrap().as_str().parse:: <i32>().unwrap())};
    }
    panic!("No matches!");
}

#[derive(Debug)]
enum Outcome{
    Overshoot,
    Undershoot,
    TooFastY,
    Hit((Point, i32)),
}

fn compute_trajectory(v0: Velocity, target: &Target) -> Outcome {
    let mut pos = (0, 0);
    let mut vt = v0;
    let mut peak = 0;

    loop {
        let prev_pos = pos;

        pos.0 += vt.0;
        pos.1 += vt.1;

        peak = cmp::max(peak, pos.1);

        vt.0 += if vt.0 > 0 {-1} else if vt.0 < 0 {1} else {0};
        vt.1 -= 1;

        if prev_pos.1 > target.start.1 && pos.1 < target.end.1 {
            return Outcome::TooFastY;
        } else if pos.1 < target.end.1 {
            if pos.0 < target.start.0 {
                return Outcome::Undershoot;
            } else {
                return Outcome::Overshoot;
            }
        } else if pos.0 < target.start.0 && vt.0 == 0 {
            return Outcome::Undershoot;
        }

        if pos.0 >= target.start.0 && pos.0 <= target.end.0 && 
           pos.1 <= target.start.1 && pos.1 >= target.end.1 {
               return Outcome::Hit((pos, peak));
           }
    }
}

fn find_horizontal_solution(vertical_velocity: i32, target: &Target) -> Vec<i32> {
    let mut hits = Vec::new();
    for cand_hv0 in 0 .. target.end.0 + 1 {
        let res = compute_trajectory((cand_hv0, vertical_velocity), &target);
        match res {
            Outcome::Hit((_pos, _peak)) => hits.push(cand_hv0), 
            _ => {},
        }
    }
    return hits;
}

fn find_solutions(target: &Target) -> HashSet<Velocity> {
    let lowest = cmp::min(target.start.1, target.end.1) - 1;
    let highest =  cmp::min(target.start.1, target.end.1).abs();
    let mut all_solutions = HashSet::new();

    for vv0 in lowest ..= highest {
        let solutions = find_horizontal_solution(vv0, &target);
        for solution in solutions {
            all_solutions.insert((solution, vv0));
        }
    }
    return all_solutions;
}

fn main() {
    let inp = read_input();
    let target = parse(&inp);
    
    println!("Solutions: {:?}", find_solutions(&target).len());
}