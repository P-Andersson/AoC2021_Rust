use std::fs;

use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day21/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Debug)]
struct PlayerState {
    id: u8,
    pos: u8,
    next_dice_start: u8,
    score: u64,
}

impl PlayerState {
    fn do_turn(&self, player_count: usize) -> PlayerState {
        let steps = (self.next_dice_start as u32 * 3) + 6;
        let new_pos = ((self.pos as u32 + steps) % 10) as u8;
        let new_score = self.score + new_pos as u64 + 1; 
        let new_next_dice_start = (self.next_dice_start + player_count as u8 * 3) % 100;

        return PlayerState{id: self.id, pos: new_pos, next_dice_start: new_next_dice_start, score: new_score};
    }
}

fn parse(data: &String) -> Vec<PlayerState> {
    let start_pos_re = Regex::new(r"^Player (\d+) starting position: (\d+)$").unwrap();

    let mut players: Vec<PlayerState> = Vec::new();
    for line in data.lines() {
        for cap in start_pos_re.captures_iter(line) {
            let id = cap.get(1).unwrap().as_str().parse().unwrap();
            let pos = cap.get(2).unwrap().as_str().parse::<u8>().unwrap();
            players.push(PlayerState{id: id, pos: pos - 1, next_dice_start: (players.len() * 3) as u8, score: 0});
        }
    }
    return players;
}


fn main() {
    let inp = read_input();
    let mut states = parse(&inp);
    
    let mut index = 0;
    let mut rolls = 0;
    loop {
        let real_index = index % states.len();
        states[real_index] = states[real_index].do_turn(states.len());
        rolls += 3;
        if states[real_index].score >= 1000 {
            break;
        } 
        index += 1;
    }

    for player in states {
        if player.score < 1000 {
            println!("Loser Score x Dice Rolls: {}", player.score * rolls);
        }
    }


    
}