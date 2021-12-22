use std::fs;
use std::fmt;
use std::cmp;

use std::collections::HashMap;


fn read_input() -> String {
    let inputs =  fs::read_to_string("Day20/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

type Pos = (i64, i64);

#[derive(Clone)] #[derive(Copy)] #[derive(PartialEq)]
enum Pixel {
    Lit,
    Dark,
}

impl Pixel {
    fn from(c: char) -> Pixel {
        return match c {
            '#' => Pixel::Lit,
            '.' => Pixel::Dark,
            _ => panic!(),
        };
    }

    fn as_char(&self) -> char {
        return match self {
            Pixel::Lit => '#',
            Pixel::Dark => ' ',
        };
    }
}

type ImageEnhancementAlgorithm = Vec<Pixel>;

enum PixelCount {
    Finite(u64),
    Infinite,
}

impl fmt::Display for PixelCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelCount::Finite(v) => write!(f, "{}", v),
            PixelCount::Infinite => write!(f, "Inf"),
        }
    }
} 


struct Image {
    pixels: HashMap<Pos, Pixel>,
    outside_bounds: Pixel,
}

impl Image {
    fn bounds(&self) -> (Pos, Pos) {
        if self.pixels.len() == 0 {
            return ((0,0), (0,0));
        }
        
        let mut min = (i64::MAX, i64::MAX);
        let mut max = (i64::MIN, i64::MIN);
        for pos in self.pixels.keys() {
            min.0 = cmp::min(min.0, pos.0);
            min.1 = cmp::min(min.1, pos.1);
            max.0 = cmp::max(max.0, pos.0);
            max.1 = cmp::max(max.1, pos.1);
        }
        return (min, max);
    }

    fn get(&self, pos: Pos) -> Pixel {
        return *self.pixels.get(&pos).unwrap_or(&self.outside_bounds);
    }

    fn draw(&self) {
        let (min, max) = self.bounds();
        for y in min.1 - 1 ..= max.1 + 1 {
            for x in min.0 - 1 ..= max.0 + 1 {
                print!("{}", self.get((x, y)).as_char());
            }
            println!("");
        }
    }

    fn count_lit(&self) -> PixelCount {
        if self.outside_bounds == Pixel::Lit {
            return PixelCount::Infinite;
        }
        return PixelCount::Finite(self.pixels.values().map(|p| if *p == Pixel::Lit {1} else {0}).sum());
    }

    fn enhance(&self, algo: &ImageEnhancementAlgorithm) -> Image {
        let mut non_default_image: HashMap<Pos, Pixel> = HashMap::new();

        let (min, max) = self.bounds();
        for y in min.1 - 1 ..= max.1 + 1 {
            for x in min.0 - 1 ..= max.0 + 1 {
                let mut index = 0;
                let mut left_shifts = 8;
                for grid_y in y - 1 ..= y + 1 {
                    for grid_x in x - 1 ..= x + 1 {
                        if self.get((grid_x, grid_y)) == Pixel::Lit {
                            index |= 1 << left_shifts;
                        }
                        left_shifts -= 1;
                    }   
                }
                non_default_image.insert((x, y), algo[index]);
            }
        }

        let new_outside_bounds = match self.outside_bounds {
            Pixel::Lit => algo[0x1FF],
            Pixel::Dark => algo[0],
        };

        return Image{pixels: non_default_image, outside_bounds: new_outside_bounds};
    }
}

fn parse(data: &String) -> (ImageEnhancementAlgorithm, Image) {
    let mut parsing_algo = true;    

    let mut algo = ImageEnhancementAlgorithm::new();

    let mut non_default_image: HashMap<Pos, Pixel> = HashMap::new();
    let mut image_y = 0;    
    for line in data.lines() {
        if line.len() == 0 {
            parsing_algo = false;
        } else if parsing_algo {
            for c in line.chars() {
                algo.push(Pixel::from(c));
            }
        } else {
            for (x, c) in line.chars().enumerate() {
                non_default_image.insert((x as i64, image_y), Pixel::from(c));
            }
            image_y += 1;
        }
    }
    return (algo, Image{pixels: non_default_image, outside_bounds: Pixel::Dark});
}


fn main() {
    let inp = read_input();
    let (algo, mut image) = parse(&inp);

    for _step in 0 .. 2 {
        image.draw();
        image = image.enhance(&algo);
    }
    image.draw();
    println!("Lit: {}", image.count_lit());
        
}