use std::fs;
use std::cmp;

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
    TooFastX,
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

        if prev_pos.0 < target.start.0 && pos.0 > target.end.0 {
            return Outcome::TooFastX;
        } else if prev_pos.1 > target.start.1 && pos.1 < target.end.1 {
            return Outcome::TooFastY;
        } else if pos.1 < target.end.1 {
            if pos.0 < target.start.0 {
                return Outcome::Undershoot;
            } else {
                return Outcome::Overshoot;
            }
        } else if pos.0 > target.end.0 {
            return Outcome::Overshoot;
        } else if pos.0 < target.start.0 && vt.0 == 0 {
            return Outcome::Undershoot;
        }

        if pos.0 >= target.start.0 && pos.0 <= target.end.0 && 
           pos.1 <= target.start.1 && pos.1 >= target.end.1 {
               return Outcome::Hit((pos, peak));
           }
    }
}

fn find_horizontal_solution(vertical_velocity: i32, target: &Target) -> Option<i32> {
    let mut lowest = 0;
    let mut highest = 300;
    let mut any_too_fast_y = false;
    loop {
        let hv0 = (highest+lowest)/2;
        if hv0 == lowest {
            return None;
        }
        let res = compute_trajectory((hv0, vertical_velocity), &target);
        match res {
            Outcome::Undershoot => lowest = hv0,
            Outcome::Overshoot => highest = hv0,
            Outcome::TooFastY => { any_too_fast_y = true; highest = hv0; },
            Outcome::TooFastX => highest = hv0,
            Outcome::Hit((_pos, peak)) => return Some(peak),
        }
    }
}

fn find_highest_vertical_solution(target: &Target) -> Option<i32> {
    let lowest = 0;
    let highest = (target.start.1 - target.start.0).abs();

    for vv0 in (lowest ..= highest).rev() {
        let res = find_horizontal_solution(vv0, &target);
        if res.is_some() {
            return res;
        }
        
    }
    return None;
}

fn main() {
    let inp = read_input();
    let target = parse(&inp);
    
    println!("{:?}", find_highest_vertical_solution(&target));
}