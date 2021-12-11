use std::fs;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day11/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

struct Dumbos {
    energy_levels: Vec<u8>,
}

impl Dumbos {
    fn width(&self) -> i32 {
        return 10;
    }    

    fn height(&self) -> i32 {
        return 10;
    }  
    
    fn energy_level(&mut self, x: i32, y: i32) -> &mut u8 {
        let width = self.width();
        return &mut self.energy_levels[((y * width) + x) as usize];
    }

    fn adjacents(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut results: Vec<(i32, i32)> = Vec::new();
        for x_offset in [-1, 0, 1] {
            for y_offset in [-1, 0, 1] {
                if x_offset != 0 || y_offset != 0 {
                    let pt_x = x + x_offset;
                    let pt_y = y + y_offset;
                    if pt_x >= 0 && pt_x < self.width() && pt_y >= 0 && pt_y < self.height() {
                        results.push((pt_x, pt_y));
                    }
                }
            }
        }
        return results;
    }

    fn reset_flashers(&mut self) {
        for y in 0 .. self.height() {
            for x in 0 .. self.width() {
                let el = self.energy_level(x, y);
                if *el > 9 {
                    *el = 0;
                }
            }
        }
    }
}


fn parse(data: &String) -> Dumbos {
    let mut energy_levels: Vec<u8> = Vec::new();    
    for l in data.lines() {
        for c in l.bytes() {
            energy_levels.push(c - ('0' as  u8));
        }
    }
    return Dumbos{energy_levels: energy_levels};
}

fn main() {
    let inp = read_input();
    let mut dumboes = parse(&inp);

    let mut step = 0;
    loop {
        let mut flashes = 0;
        step += 1;
        let mut flashers: Vec<(i32, i32)> = Vec::new();
        for y in 0 .. dumboes.height() {
            for x in 0 .. dumboes.width() {
                let el = dumboes.energy_level(x, y);
                *el += 1;
                if *el == 10 {
                    flashers.push((x, y));
                    flashes += 1;
                }
            }
        }
        while flashers.len() > 0 {
            let flasher = flashers.pop().unwrap();
            for target in dumboes.adjacents(flasher.0, flasher.1) {
                let el = dumboes.energy_level(target.0, target.1);
                *el += 1;
                if *el == 10 {
                    flashers.push(target);
                    flashes += 1;
                }
            }
        }
        dumboes.reset_flashers();

        if flashes >= 100 {
            break;
        }
    }

    println!("First Synch Step: {}", step);
  
}