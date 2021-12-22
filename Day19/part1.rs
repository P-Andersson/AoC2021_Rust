use std::fs;
use std::mem;
use std::ops;

use std::collections::HashSet;
use std::collections::HashMap;

use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day19/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Clone)] #[derive(Copy)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    fn rotation_steps(&self) -> i8  {
        return match self {
            Rotation::Deg0 => 0,
            Rotation::Deg90 => 1,
            Rotation::Deg180 => 2,
            Rotation::Deg270 => 3,
        };
    }
}

#[derive(Clone)] #[derive(Copy)] #[derive(Hash)]  #[derive(PartialEq)] #[derive(Eq)] #[derive(Debug)]
struct Pos {
    vec: [i32; 3],
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        return Pos{vec: [x, y, z]};
    }

    fn rotate(&self, rotation: SpaceRotation) -> Pos {
        let d1 = rotation.get_dimension_mapping(0);
        let d2 = rotation.get_dimension_mapping(1);
        let d3 = rotation.get_dimension_mapping(2);
        let mut x = self.vec[d1];
        let mut y = self.vec[d2];
        let mut z = self.vec[d3];
        x *= rotation.get_dimension_multiplier(0);
        y *= rotation.get_dimension_multiplier(1);
        z *= rotation.get_dimension_multiplier(2);

        return Pos::new(x, y, z);
    }
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Pos {
        return Pos::new(self.vec[0] + rhs.vec[0], self.vec[1] + rhs.vec[1], self.vec[2] + rhs.vec[2]);
    }
}

impl ops::Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Pos {
        return Pos::new(self.vec[0] - rhs.vec[0], self.vec[1] - rhs.vec[1], self.vec[2] - rhs.vec[2]);
    }
}

#[derive(Clone)]
struct SpaceRotation {
    dimension_indicies_vec: [i8; 3],
}

impl SpaceRotation {
    fn new() -> Self {
        return SpaceRotation{dimension_indicies_vec: [1, 2, 3]};
    }

    fn get_dimension_mapping(&self, mapping: usize)-> usize{
        return ((self.dimension_indicies_vec[mapping].abs()) as usize) - 1;
    }

    fn get_dimension_multiplier(&self, mapping: usize)-> i32{
        return if self.dimension_indicies_vec[mapping] < 0 {-1} else {1};
    }

    fn adjust_pitch(&self, rotation: Rotation) -> SpaceRotation {
        let mut new_rotation = self.clone();
        for _step in 0 .. rotation.rotation_steps() {
            let mut d1 = new_rotation.dimension_indicies_vec[0];
            let mut d2 = new_rotation.dimension_indicies_vec[1];
            mem::swap(&mut d1, &mut d2);
            d2 *= -1;
            new_rotation.dimension_indicies_vec[0] = d1;
            new_rotation.dimension_indicies_vec[1] = d2;
        }
        return new_rotation;
    }

    fn adjust_yaw(&self, rotation: Rotation) -> SpaceRotation {
        let mut new_rotation = self.clone();
        for _step in 0 .. rotation.rotation_steps() {
            let mut d1 = new_rotation.dimension_indicies_vec[0];
            let mut d2 = new_rotation.dimension_indicies_vec[2];
            mem::swap(&mut d1, &mut d2);
            d2 *= -1;
            new_rotation.dimension_indicies_vec[0] = d1;
            new_rotation.dimension_indicies_vec[2] = d2;
        }
        return new_rotation;
    }
    fn adjust_roll(&self, rotation: Rotation) -> SpaceRotation {
        let mut new_rotation = self.clone();
        for _step in 0 .. rotation.rotation_steps() {
            let mut d1 = new_rotation.dimension_indicies_vec[1];
            let mut d2 = new_rotation.dimension_indicies_vec[2];
            mem::swap(&mut d1, &mut d2);
            d2 *= -1;
            new_rotation.dimension_indicies_vec[1] = d1;
            new_rotation.dimension_indicies_vec[2] = d2;
        }
        return new_rotation;
    }
}

#[derive(Clone)] #[derive(Debug)]
struct Scanner {
    id: u32,
    pos: Option<Pos>,
    beacons: Vec<Pos>,
    rotation: Pos,
}

impl Scanner {
    fn new(id: u32) -> Self{
        return Scanner{id: id, pos: None, beacons: Vec::new(), rotation: Pos::new(1, 2, 3)};
    }

    fn orient(&self, pitch: Rotation, roll: Rotation, yaw: Rotation) -> Scanner {
        let rotation = SpaceRotation::new();
        let mut new_scanner = Scanner::new(self.id);
        new_scanner.pos = self.pos.clone();
        new_scanner.rotation = self.rotation;
        let used_rotation = rotation.adjust_pitch(pitch.clone()).adjust_roll(roll).adjust_yaw(yaw);
        new_scanner.rotation = new_scanner.rotation.rotate(used_rotation.clone());
        for beacon in &self.beacons {
            new_scanner.beacons.push(beacon.rotate(used_rotation.clone()));
        }
        return new_scanner;
    }

    fn orientations(&self) -> Vec<Scanner> {
        let mut result: Vec<Scanner> = Vec::new();
        for yaw in [Rotation::Deg0, Rotation::Deg90, Rotation::Deg180, Rotation::Deg270, ] {
            for pitch in [Rotation::Deg0, Rotation::Deg90, Rotation::Deg180, Rotation::Deg270, ] {
                result.push(self.orient(pitch, Rotation::Deg0, yaw));
            }
        }
        for roll in [Rotation::Deg90, Rotation::Deg270, ] {
            for yaw in [Rotation::Deg0, Rotation::Deg90, Rotation::Deg180, Rotation::Deg270, ] {
                result.push(self.orient(Rotation::Deg0, roll, yaw));
            }
        }

        return result;
    }

    fn position(&self, pos: Pos) -> Scanner {
        let mut new_scanner = Scanner::new(self.id);
        new_scanner.pos = Some(pos);
        new_scanner.beacons = self.beacons.clone();
        new_scanner.rotation = self.rotation;

        return new_scanner;
    }

    fn try_position_with(&self, other: &Scanner) -> Vec<ScannerGroup> {
        let mut scanner_groups: Vec<ScannerGroup> = Vec::new();
        let self_pos = self.pos.unwrap();         

        for scanner in other.orientations() {
            let mut offset_candidates: HashMap<Pos, u32> = HashMap::new();
            for beacon in &self.beacons {
                for other_beacon in &scanner.beacons {
                    let offset = (*beacon + self_pos) - *other_beacon;
                    offset_candidates.insert(offset, offset_candidates.get(&offset).unwrap_or(&0) + 1);   
                }
            }
            let filtered_candidates = offset_candidates.iter()
                          .filter(|(_ ,v)| **v >= 12).map(|(k, _)| *k).collect::<Vec<Pos>>();
            'offset_checker : for candidate in &filtered_candidates {
                let positioned_scanner = scanner.position(*candidate);
                let mut real_positions: HashSet<Pos> = HashSet::new();         
                
                for beacon in &self.beacons {
                    real_positions.insert(*beacon + self_pos);
                }
                for beacon in &positioned_scanner.beacons {
                    let abs_position = *beacon + positioned_scanner.pos.unwrap();
                    let rel_position = abs_position - self_pos;
                    if !rel_position.vec.iter().any(|v| v.abs() > 1000) {
                        if !real_positions.contains(&abs_position){
                            // A beacon that should be seen is not
                            continue 'offset_checker;
                        }
                    }
                    real_positions.insert(abs_position);
                }
                let sg = ScannerGroup{scanners: (self.clone(), positioned_scanner), beacons_abs: real_positions};
                scanner_groups.push(sg);
            }
        }
        return scanner_groups;
    }
}

#[derive(Debug)] #[derive(Clone)]
struct ScannerGroup {
    scanners: (Scanner, Scanner),
    beacons_abs: HashSet<Pos>,
}

impl ScannerGroup {
    fn get_overlap_count(&self) -> usize {
        return [&self.scanners.0, &self.scanners.1].iter().map(|&s| s.beacons.len() ).sum::<usize>()
                - self.beacons_abs.len();
    }
}


fn parse(data: &String) -> Vec<Scanner> {
    let header_re = Regex::new(r"^--- scanner ([-\d]+) ---$").unwrap();

    let mut scanners: Vec<Scanner> = Vec::new();
    let mut scanner: Option<Scanner> = None;
    for line in data.lines() {
        let captures = header_re.captures(line);
        if captures.is_some() {
            let cap = captures.unwrap();
            scanner = Some(Scanner::new(cap.get(1).unwrap().as_str().parse::<u32>().unwrap()));
        } else if line.len() == 0 {
            if scanner.is_some() {
                scanners.push(scanner.unwrap());
                scanner = None;
            }
        } else {
            let splits: Vec<&str> = line.split(',').collect();
            let beacon = Pos::new(splits[0].parse::<i32>().unwrap(),
                                  splits[1].parse::<i32>().unwrap(),
                                  splits[2].parse::<i32>().unwrap());
            scanner.as_mut().unwrap().beacons.push(beacon);
        }
    
    }
    if scanner.is_some() {
        scanners.push(scanner.unwrap());
    }
    return scanners;
}


fn main() {
    let inp = read_input();
    let mut scanners = parse(&inp);
    scanners[0].pos = Some(Pos::new(0, 0, 0));

    let mut pairs: Vec<ScannerGroup> = Vec::new();

    let mut checked = HashSet::new();
    'pairing_loop: loop {
        let mut positioned = Vec::new();        
        let mut unpositioned = Vec::new();
        for i in 0 .. scanners.len() {
            let scanner = &scanners[i];
            if scanner.pos.is_some() {
                if !checked.contains(&i) {
                   positioned.push(i);
                }
            } else {
                unpositioned.push(i);
            }
        }
        println!("Unpos: {}", unpositioned.len());
        if unpositioned.len() == 0 {
            break;
        }
        for i in &positioned {
            let mut any_new_positioned = false;
            for j in &unpositioned {
                let first = &scanners[*i];
                let second = &scanners[*j];
                let new_pairs = first.try_position_with(second);
                if new_pairs.len() >= 1 {
                    scanners[*j] = new_pairs[0].scanners.1.clone();
                    pairs.push(new_pairs[0].clone());
                    any_new_positioned = true;
                }

            }
            checked.insert(*i);
            if any_new_positioned {
                continue 'pairing_loop;
            }
        }

    }
    let mut abs_beacons : HashSet<Pos> = HashSet::new();
    for pair in pairs {
        for beacon in pair.beacons_abs {
            abs_beacons.insert(beacon);
        }
    }
    
    
 //   println!("{:?}", abs_beacons);
    println!("Count: {}", abs_beacons.len());
    
}