use std::fs;
use std::cmp;
use std::collections::HashMap;

use regex::Regex;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day21/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Debug)] #[derive(PartialEq)] #[derive(Eq)] #[derive(Hash)] #[derive(Clone)]
struct PlayerState {
    id: u8,
    pos: u8,
    score: u64,
}

impl PlayerState {
    fn do_turn(&self) -> Vec<PlayerState> {
        let mut new_universes: Vec<PlayerState> = Vec::new();
        for d1 in [1, 2, 3] {
            for d2 in [1, 2, 3] {
                for d3 in [1, 2, 3] {
                    let steps = d1 + d2 + d3;
                    let new_pos = ((self.pos as u32 + steps) % 10) as u8;
                    let new_score = self.score + new_pos as u64 + 1; 
            
                    new_universes.push(PlayerState{id: self.id, pos: new_pos, score: new_score});
                }
            }    
        }
        return new_universes;
    }
}

#[derive(PartialEq)] #[derive(Eq)] #[derive(Hash)] #[derive(Clone)]
struct GameState {
    current_player_index: usize,
    players: Vec<PlayerState>,
}

type Wins = i64;
type WinCounts = HashMap<u8, Wins>;
type Cache = HashMap<GameState, WinCounts>; 


fn play(players: &Vec<PlayerState>) -> WinCounts
{
    fn _play_rec(state: GameState, cache: &mut Cache) -> WinCounts {
        let cache_entry = cache.get(&state);
        if cache_entry.is_some() {
            return cache_entry.unwrap().clone();
        }

        let mut wins = WinCounts::new();
        let cur_turn_index = state.current_player_index;
        let next_turn = (cur_turn_index+ 1) % state.players.len();
        let new_player_states = state.players[cur_turn_index].do_turn();
        for player_state in new_player_states {
            let mut new_game_state = state.clone();
            new_game_state.players[cur_turn_index] = player_state.clone();
            new_game_state.current_player_index = next_turn;
            if player_state.score >= 21 {
                wins.insert(player_state.id, wins.get(&player_state.id).unwrap_or(&0) + 1);
            } else {
                for (id, win_counts) in _play_rec(new_game_state, cache) {
                    wins.insert(id, wins.get(&id).unwrap_or(&0) + win_counts);
                }
            }

        }

        cache.insert(state, wins.clone());
        return wins;
    }    

    let mut cache = Cache::new();
    let state = GameState{current_player_index: 0, players: players.clone()};

    return _play_rec(state, &mut cache);
}

fn parse(data: &String) -> Vec<PlayerState> {
    let start_pos_re = Regex::new(r"^Player (\d+) starting position: (\d+)$").unwrap();

    let mut players: Vec<PlayerState> = Vec::new();
    for line in data.lines() {
        for cap in start_pos_re.captures_iter(line) {
            let id = cap.get(1).unwrap().as_str().parse().unwrap();
            let pos = cap.get(2).unwrap().as_str().parse::<u8>().unwrap();
            players.push(PlayerState{id: id, pos: pos - 1, score: 0});
        }
    }
    return players;
}


fn main() {
    let inp = read_input();
    let states = parse(&inp);
    
    let wins = play(&states);
    println!("ID -> Wins: {:?}", wins);
    
    let mut highest = 0;
    for win_count in wins.values() {
        highest = cmp::max(highest, *win_count);
    }
    println!("Winners victory counts: {}", highest);
    
}