use std::fs;
use std::cmp;
use std::collections::HashMap;
use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day5/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

type Point = (i32, i32);

type VentRange = (Point, Point);

fn parse(data: String) -> Vec<VentRange> {
    let mut result: Vec<VentRange> = Vec::new();
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    for elem in data.split("\n") {
        for cap in re.captures_iter(elem) {
            result.push((
                (cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()), 
                (cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap()), 
            ));
        }
    }
    return result;
}

fn build_map(ranges: &Vec<VentRange>) -> HashMap<Point, u32> {
    let mut map: HashMap<Point, u32> = HashMap::new();
    
    let mut add_point = |point: Point| {
        if map.contains_key(&point) {
            map.insert(point, map[&point] + 1);
        } else 
        {
            map.insert(point, 1);
        }
    };
    for range in ranges {
        if range.0.1 == range.1.1 {
            for x in cmp::min(range.0.0, range.1.0) .. cmp::max(range.0.0, range.1.0) + 1 {
                add_point((x, range.0.1));
            }
        } else if range.0.0 == range.1.0 {
            for y in cmp::min(range.0.1, range.1.1) .. cmp::max(range.0.1, range.1.1) + 1 {
                add_point((range.0.0, y));
            }
        }
    }
    return map;
}

fn main() {
    let inp = read_input();
    let parsed = parse(inp);

    let map = build_map(&parsed);

    let mut overlaps = 0;
    for danger in map.values() {
        if *danger >= 2 {
            overlaps += 1;
        } 
    }

    //println!("{:?}", map);
    println!("Overlaps: {}", overlaps);    
}