use std::fs;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day15/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Debug)] 
struct RiskMap {
    map: Vec<u8>,
    width: i64,
    height: i64,
}

type Pos = (i64, i64);

#[derive(Eq)] #[derive(PartialEq)] #[derive(Clone)]
struct PathNode {pos: Pos, risk: u8}

#[derive(Eq)] #[derive(Clone)]
struct Path {path: Vec<PathNode>, risk: i64, est_cost_to_end: i64}

impl Path {
    pub fn new() -> Self {
        return Path{path: Vec::new(), risk: 0, est_cost_to_end: 0};
    }

    pub fn append(&mut self, node: PathNode, end: &Pos) {
        self.risk += node.risk as i64;
        self.est_cost_to_end = self.risk;
        self.path.push(node);
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.est_cost_to_end.cmp(&other.est_cost_to_end);
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.est_cost_to_end.cmp(&other.est_cost_to_end));
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        return self.risk == other.risk;
    }
}

impl RiskMap {
    pub fn get_risk(&self, pos: Pos) -> u8 {
        let (x, y) = pos;
        return self.map[(y * self.height + x) as usize];

    }

    pub fn get(&self, pos: Pos) -> Option<PathNode> {
        let (x, y) = pos;
        if x < self.width && y < self.height {
            return Some(PathNode{pos: pos, risk: self.map[(y * self.height + x) as usize]});
        }
        return None;
    }

    pub fn extend(&self, multiplier: i64) -> RiskMap{
        let mut new_risks: Vec<u8> = Vec::new();
        let new_width = self.width * multiplier;
        let new_height = self.height * multiplier;

        for y in 0 .. new_height {
            for x in 0 .. new_width {
                let ox = x % self.width;
                let oy = y % self.height;
                let risk = ((self.get_risk((ox, oy)) - 1) as i64 + x/self.width + y/self.height) % 9 + 1;
                new_risks.push(risk as u8);
            }
        }
        return RiskMap{map: new_risks, width: new_width, height: new_height};
    }

    pub fn render(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                print!("{}", self.get_risk((x, y)));
            }
            println!("");
        }
    }

    pub fn adjacents(&self, pos: Pos) -> Vec<Pos> {
        let (x, y) = pos;

        let mut results: Vec<Pos> = Vec::new();

        for offsets in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (o_x, o_y) = offsets;

            let (nx, ny) = (x + o_x, y + o_y);
            if nx < 0 || nx >= self.width || ny < 0 || ny >= self.height {
                continue;
            }
            results.push((nx, ny));
        }

        return results;
    }

    pub fn safest(&self) -> Path {
        let mut queue: BinaryHeap<Reverse<Path>> = BinaryHeap::new();

        let end = (self.width - 1, self.height - 1);
        let mut start = Path::new();
        let mut visited: HashSet<Pos> = HashSet::new();
        start.append(self.get((0, 0)).unwrap(), &end);        
        queue.push(Reverse(start));

     
        loop {
            let best = queue.pop().unwrap();
            let head = (&best.0.path.last()).unwrap();
            //println!("Probe Est Cost: {}, Dist: {}", best.0.est_cost_to_end, best.0.path.len());


            for adj in self.adjacents(head.pos) {
                let cand = self.get(adj).unwrap();
                if !visited.contains(&cand.pos) {
                    visited.insert(cand.pos);
                    let mut new = best.0.clone();
                    new.append(cand, &end);

                    if new.path.last().unwrap().pos == end {
                        return new;
                    }

                    queue.push(Reverse(new));
                }
            }
        }

    }
}

fn parse(data: &String) -> RiskMap {
    let mut width = 0;
    let mut risks: Vec<u8> = Vec::new();
    for l in data.lines() {
        width = l.len();
        for c in l.chars() {
            risks.push((c as u8) - ('0' as u8));
        }    
    }
    return RiskMap{map: risks.clone(), width: width as i64, height: (risks.len()/width) as i64};
}

fn main() {
    let inp = read_input();
    let risks = parse(&inp).extend(5);

    //risks.render();

    let safest = risks.safest();

    println!("Risk: {}", safest.risk - safest.path[0].risk as i64);
}