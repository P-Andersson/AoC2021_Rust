use std::fs;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day9/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

struct Heightmap {
    map : Vec<Vec<i32>>,
}

impl Heightmap {
    pub fn width(&self) -> i64 {
        return self.map[0].len() as i64;
    }

    pub fn height(&self) -> i64 {
        return self.map.len() as i64;
    }

    pub fn val(&self, x: i64, y: i64) -> i32 {
        return self.map[y as usize][x as usize];
    }

    pub fn adjacents(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut result: Vec<(i64, i64)> = Vec::new();
        for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let pt_x = x + offset.0;
            let pt_y = y + offset.1;
            if pt_x >= 0 && pt_x < self.width() && pt_y >= 0 && pt_y < self.height() {
                result.push((pt_x, pt_y));
            }

        }
        return result;
    }
}

fn parse(data: String) -> Heightmap {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    for elem in data.lines() {
        let mut row: Vec<i32> = Vec::new();
        for pt in elem.chars() {
            row.push((pt as i32) - ('0' as i32));
        }
        heights.push(row);
    }
    return Heightmap{map: heights};
}

fn main() {
    let inp = read_input();
    let parsed = parse(inp);
    
    let mut lowpoints: Vec<i32> = Vec::new();
    for y in 0 .. parsed.height() {
        for x in 0 .. parsed.width(){
            let adj = parsed.adjacents(x, y);
            let test = parsed.val(x, y);
            if adj.iter().all(| p | parsed.val(p.0, p.1) > test ) {
                lowpoints.push(test);
            }
            
        }
    }
    println!("Risk sum: {}", lowpoints.iter().map(| lp | 1 + lp).sum::<i32>());    
}