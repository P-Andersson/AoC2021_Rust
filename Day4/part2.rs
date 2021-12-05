use std::fs;
use std::fmt;
use std::collections::HashSet;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day4/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

struct BingoBoard {
    vals : Vec<Vec<i32>>,
    marks : Vec<Vec<bool>>,
}

impl BingoBoard {
    pub fn new(values: &Vec<Vec<i32>>) -> Self {
        let mut marks: Vec<Vec<bool>> = Vec::new();
        for row in values {
            let mut row_marks: Vec<bool> = Vec::new();
            row_marks.resize(row.len(), false);
            marks.push(row_marks);
        }        
        
        return Self{ vals: values.clone(), marks: marks };
    }

    fn width(&self) -> usize {
        return self.vals[0].len();
    }
    fn height(&self) -> usize {
        return self.vals.len();
    }

    fn val(&self, x: usize, y: usize) -> i32 {
        return self.vals[y][x];
    }
    fn mark(&self, x: usize, y: usize) -> bool {
        return self.marks[y][x];
    }

    fn score(&self, last_draw: i32) -> i32{
        let mut total = 0;
        for y in 0 .. self.height() {
            for x in 0 .. self.width() {
                if !self.mark(x, y) {
                    total += self.val(x, y);
                }
            }
        }
        return total * last_draw;
    }

    fn check_row_completed(&self, y: usize) -> bool {
        for x in 0 .. self.width() {
            if !self.mark(x, y) {
                return false;
            }
        } 
        return true;
    }
    fn check_column_completed(&self, x: usize) -> bool {
        for y in 0 .. self.height() {
            if !self.mark(x, y) {
                return false;
            }
        } 
        return true;
    }

    //
    // Returns true if the board got bingo due to this mark
    // false otherwise
    //
    fn mark_value(&mut self, val: i32) -> bool {
        for y in 0 .. self.height() {
            for x in 0 .. self.width() {
                if self.val(x, y) == val {
                    self.marks[y][x] = true;
                    return self.check_column_completed(x) || self.check_row_completed(y);
                }
            }
        }
        return false;
    }

}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0 .. self.height() {
            for x in 0 .. self.width() {
                write!(f, "{}{:3} ", if self.mark(x, y) { '#' } else { ' ' }, 
                self.val(x, y)).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        return fmt::Result::Ok(());
    }
} 

fn parse(data: String) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut draws: Vec<i32> = Vec::new();
    let mut boards: Vec<BingoBoard> = Vec::new();
    
    let mut index = 0;
    let mut current_board: Vec<Vec<i32>> = Vec::new();
    for elem in data.split("\n") {
        if index == 0 {
            draws = elem.split(",").map(|s| s.parse::<i32>().expect(
                &format!("Unparsable:  {}", elem))).collect();
        } else
        {
            if elem.len() == 0 {
                if current_board.len() > 0 {
                    boards.push(BingoBoard::new(&current_board));
                }
                current_board = Vec::new();
            } else {
                let row_values : Vec<i32> = elem.split_whitespace().map(|s| s.parse::<i32>().expect(
                    &format!("Unparsable:  {}", elem))).collect();

                current_board.push(row_values);
            }
        }
        index += 1;
    }

    if current_board.len() > 0 {
        boards.push(BingoBoard::new(&current_board));
    }

    return (draws, boards);
}

fn main() {
    let inp = read_input();
    let (draws, mut boards) = parse(inp);

    let mut winning_score = 0;

    let mut winners: HashSet<usize> = HashSet::new();

    let boards_count: usize = boards.len();
    'drawer: for draw in &draws {
        let mut board_index: usize = 0;
        for board in boards.iter_mut() {
            if board.mark_value(*draw) == true {
                winning_score = board.score(*draw);
                winners.insert(board_index);
                if winners.len() == boards_count {
                    winning_score = board.score(*draw);
                    break 'drawer;
                }
            }
            board_index += 1;
        }
    }
    
    println!("Last Winning Score: {}", winning_score);
}