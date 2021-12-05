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
            // 45 degree lines are allowed
        } else {
            //println!("{:?}", range);
            let mut cur_x = range.0.0;
            let mut cur_y = range.0.1;
            let end_x = range.1.0;
            let end_y = range.1.1;

            let step_x = if cur_x < end_x {1} else {-1};
            let step_y = if cur_y < end_y {1} else {-1};

            while cur_x != end_x + step_x && cur_y != end_y + step_y {
                add_point((cur_x, cur_y));
                cur_x += step_x;
                cur_y += step_y;
            }
        }
    }
    return map;
}

fn visualize_map(map: &HashMap<Point, u32>) {
    let mut max_x = 0;
    let mut max_y = 0;
    for point in map.keys() {
        max_x = cmp::max(point.0, max_x);
        max_y = cmp::max(point.1, max_y);
    }
    
    for y in 0 .. max_y + 1 {
        for x in 0 .. max_x + 1 {
            print!("{:2}", map.get(&(x, y)).unwrap_or(&0));
        }
        println!("");
    }

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

    //visualize_map(&map);
    println!("Overlaps: {}", overlaps);    
}