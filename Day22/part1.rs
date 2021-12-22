use std::fs;

use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day22/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Clone)] #[derive(Copy)] #[derive(Debug)]
struct Cube {
    start: (i64, i64, i64),
    end: (i64, i64, i64),
}

impl Cube {
    fn new(start: (i64, i64, i64), end: (i64, i64, i64)) -> Self {
        return Cube{start: start, end: end};
    }

    fn remove(&self, other: &Cube) -> Vec<Cube> {
        let mut targets: Vec<Cube> = [self.clone()].to_vec();
        loop {
            let mut any_intersection = false;
            for i in 0 .. targets.len() {
                let target = targets[i];
                if (other.end.0 > target.start.0 && other.start.0 < target.end.0) 
                    && (other.end.1 > target.start.1 && other.start.1 < target.end.1)
                    && (other.end.2 > target.start.2 && other.start.2 < target.end.2) {
                        any_intersection = true;
                        targets.remove(i);
                        // Repeatedly divide, note that this logic should remove cubes entirely
                        // contained within the other
                        // Divide along X
                        if (other.start.0 > target.start.0) && (other.start.0 <= target.end.0) {
                            targets.push(Cube::new((target.start.0, target.start.1, target.start.2),
                                                   (other.start.0, target.end.1, target.end.2)));
                            targets.push(Cube::new((other.start.0, target.start.1, target.start.2),
                                                   (target.end.0, target.end.1, target.end.2)));
                        }
                        else if (other.end.0 < target.end.0) && (other.end.0 >= target.start.0) {
                            targets.push(Cube::new((target.start.0, target.start.1, target.start.2),
                                                   (other.end.0, target.end.1, target.end.2)));
                            targets.push(Cube::new((other.end.0, target.start.1, target.start.2),
                                                   (target.end.0, target.end.1, target.end.2)));
                        }
                        // Divide along y
                        else if (other.start.1 > target.start.1) && (other.start.1 <= target.end.1) {
                            targets.push(Cube::new((target.start.0, target.start.1, target.start.2),
                                                   (target.end.0, other.start.1, target.end.2)));
                            targets.push(Cube::new((target.start.0, other.start.1, target.start.2),
                                                   (target.end.0, target.end.1, target.end.2)));
                        }
                        else if (other.end.1 < target.end.1) && (other.end.1 >= target.start.1) {
                            targets.push(Cube::new((target.start.0, target.start.1, target.start.2),
                                                   (target.end.0, other.end.1, target.end.2)));
                            targets.push(Cube::new((target.start.0, other.end.1, target.start.2),
                                                   (target.end.0, target.end.1, target.end.2)));
                        }
                        // Divide along z
                        else if (other.start.2 > target.start.2) && (other.start.2 <= target.end.2) {
                            targets.push(Cube::new((target.start.0, target.start.1, target.start.2),
                                                    (target.end.0, target.end.1, other.start.2)));
                            targets.push(Cube::new((target.start.0, target.start.1, other.start.2),
                                                    (target.end.0, target.end.1, target.end.2)));
                        }
                        else if (other.end.2 < target.end.2) && (other.end.2 >= target.start.2) {
                            targets.push(Cube::new((target.start.0, target.start.1, target.start.2),
                                                    (target.end.0, target.end.1, other.end.2)));
                            targets.push(Cube::new((target.start.0, target.start.1, other.end.2),
                                                    (target.end.0, target.end.1, target.end.2)));
                        }
                        break;
                }
            }
            if !any_intersection {
                break;
            }
        }

        return targets;
    }

    fn volume(&self) -> i64 {
        return (self.end.0 - self.start.0).abs() * (self.end.1 - self.start.1).abs() * (self.end.2 - self.start.2).abs();  
    }

}

#[derive(Debug)]
struct CubeSapce {
    cubes: Vec<Cube>,
}

impl CubeSapce {
    fn new() -> Self{
        return CubeSapce{cubes: Vec::new()};
    }

    fn volume(&self) -> i64 {
        let mut volume: i64 = 0;
        for cube in &self.cubes {
            volume += cube.volume();
        }
        return volume;
    }

    fn add(&self, cube: Cube) -> CubeSapce{
        let mut new_space = CubeSapce::new();
        for old_target in &self.cubes {
            for new_cube in old_target.remove(&cube) {
                new_space.cubes.push(new_cube);
            }
        }
        new_space.cubes.push(cube);
        return new_space;
    }
    fn remove(&self, cube: Cube) -> CubeSapce{
        let mut new_space = CubeSapce::new();
        for old_target in &self.cubes {
            for new_cube in old_target.remove(&cube) {
                new_space.cubes.push(new_cube);
            }
        }
        return new_space;
    }

    fn intersection_volume(&self, intersector: &Cube) -> i64 {
        let high_volume = self.volume();
        let low_volume = self.remove(intersector.clone()).volume();
        return high_volume - low_volume;
    }
}



fn parse(data: &String) -> CubeSapce {
    let reboot_step_re = Regex::new(
        r"^((?:on)|(?:off)) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)$").unwrap();

    let mut cube_space = CubeSapce::new();
    for line in data.lines() {
        for cap in reboot_step_re.captures_iter(line) {
            let on = match cap.get(1).unwrap().as_str() {
                "on" => true,
                "off" => false,
                _ => panic!(),
            };
            let start = (cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                         cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                         cap.get(6).unwrap().as_str().parse::<i64>().unwrap());
            let end =   (cap.get(3).unwrap().as_str().parse::<i64>().unwrap() + 1,
                         cap.get(5).unwrap().as_str().parse::<i64>().unwrap() + 1,
                         cap.get(7).unwrap().as_str().parse::<i64>().unwrap() + 1);
            if on {
                cube_space = cube_space.add(Cube::new(start, end));
            } else {
                cube_space = cube_space.remove(Cube::new(start, end));
            }
        }
    }
    return cube_space;
}


fn main() {
    let inp = read_input();
    let cube_space = parse(&inp);

    println!("Intersection Volume: {}", cube_space.intersection_volume(&Cube::new(
        (-50, -50, -50),
        (50+1, 50+1, 50+1)
    )));
}