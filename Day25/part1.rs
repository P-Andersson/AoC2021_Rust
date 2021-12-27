use std::fs;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day25/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Clone)] #[derive(PartialEq)]
enum Facing {
    Right,
    Down
}

#[derive(Clone)] #[derive(PartialEq)]
enum Tile {
    Empty,
    SeaCucumber(Facing),
}

#[derive(Clone)]
struct SeaCucumberMap {
    map: Vec<Tile>,
    width: i64,
    height: i64,
}

impl SeaCucumberMap {
    fn render(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                print!("{}", match self.get(x, y) {
                    Tile::Empty => '.',
                    Tile::SeaCucumber(Facing::Right) => '>',
                    Tile::SeaCucumber(Facing::Down) => 'v',
                });
            }
            println!("");
        }
        println!("");
    }

    fn get(&self, x: i64, y: i64) -> Tile {
        return self.map[((y * self.width) + x) as usize].clone();
    }

    fn set(&mut self, x: i64, y: i64, tile: Tile) {
        self.map[((y * self.width) + x) as usize] = tile;
    }
    
    fn update(&mut self) -> usize {
        let mut moved = 0;
        // Right facing
        let mut next_state = self.clone();

        for y in 0 .. self.height {
            for x in 0 .. self.width {
                let tile = self.get(x, y);
                if tile == Tile::SeaCucumber(Facing::Right) {
                    let next_x = (x + 1) % self.width;
                    let next_y = y;
                    let next_t = self.get(next_x, next_y);
                    if next_t == Tile::Empty {
                        moved += 1;
                        next_state.set(next_x, next_y, tile);
                        next_state.set(x, y, next_t);
                    }
                }
            }
        }
        self.map = next_state.map.clone();
        // Down facing
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                let tile = self.get(x, y);
                if tile == Tile::SeaCucumber(Facing::Down) {
                    let next_x = x;
                    let next_y = (y + 1) % self.height;
                    let next_t = self.get(next_x, next_y);
                    if next_t == Tile::Empty {
                        moved += 1;
                        next_state.set(next_x, next_y, tile);
                        next_state.set(x, y, next_t);
                    }
                }
            }
        }
        self.map = next_state.map.clone();
        return moved;
    }
}

fn parse(data: &String) -> SeaCucumberMap {
    let mut width = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    for l in data.lines() {
        width = l.len();
        for c in l.chars() {
            tiles.push(match c {
                '.' => Tile::Empty,
                '>' => Tile::SeaCucumber(Facing::Right),
                'v' => Tile::SeaCucumber(Facing::Down),
                _ => panic!("Unknown characters: \"{}\"", c),
            });
        }    
    }
    return SeaCucumberMap{map: tiles.clone(), width: width as i64, height: (tiles.len()/width) as i64};
}

fn main() {
    let inp = read_input();
    let mut cucumber_map = parse(&inp);

    let mut step = 1;
    while cucumber_map.update() > 0 {
        step += 1;
    }
    cucumber_map.render();
    println!("Stopped after {} steps", step);
}