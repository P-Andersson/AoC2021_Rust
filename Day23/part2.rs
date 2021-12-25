use std::fs;
use std::io;
use std::cmp::Reverse;
use std::cmp::Ordering;

use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::BinaryHeap;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day23/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Clone)] #[derive(PartialEq)]  #[derive(Eq)] #[derive(Debug)] #[derive(Copy)] #[derive(Hash)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodType {
    fn step_cost(&self) -> i64 {
        match self {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000,
        }
    }

    fn representation(&self) -> char {
        match self {
            AmphipodType::Amber =>'A',
            AmphipodType::Bronze => 'B',
            AmphipodType::Copper => 'C',
            AmphipodType::Desert => 'D',
        }
    }
}

#[derive(Clone)] #[derive(PartialEq)]  #[derive(Eq)] #[derive(Debug)] #[derive(Copy)] #[derive(Hash)]
struct Amphipod {
    atype: AmphipodType,
    id: usize,
}

impl Amphipod {
    fn new(id: usize, atype: AmphipodType) -> Self {
        return Amphipod{id: id, atype: atype};
    }
}


#[derive(Clone)] #[derive(Debug)] #[derive(Eq)] #[derive(PartialEq)]
enum Property {
    Corridor,
    Wait,
    Destination(AmphipodType),
}

type Slot = Option<Amphipod>;

#[derive(Clone)] #[derive(Debug)] #[derive(Eq)] #[derive(PartialEq)] 
struct Connection {
    target: usize,
    steps: i64, 
}

impl Connection {
    fn new(target: usize, steps: i64) -> Self {
        return Connection{target: target, steps: steps};
    }
}

#[derive(Clone)] #[derive(Debug)] #[derive(Eq)] #[derive(PartialEq)] 
struct Room {
    name: &'static str,
    property: Property,
    slots: Vec<Slot>,
    connections: Vec<Connection>,    
}

impl Room {
    fn new(name: &'static str, property: Property, slot_count: usize) -> Self {
        let mut slots = Vec::new();
        for _i in 0 .. slot_count {
            slots.push(None);
        }
        return Room{name: name, property: property, slots: slots, connections: Vec::new()};

    }    

    fn remove(&mut self, amphi_id: usize)
    {
        for slot in &mut self.slots{
            if slot.is_some() && slot.unwrap().id == amphi_id {
                *slot = None;
                return;
            }
        }
    }
}

#[derive(Clone)] #[derive(Eq)] #[derive(PartialEq)] 
struct Map {
    rooms: Vec<Room>
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (room_i, room) in self.rooms.iter().enumerate() {
            for (slot_i, slot) in room.slots.iter().enumerate() {
                (room_i, slot_i, slot).hash(state);
            }
        }
    }
}

impl Map {
    fn positions(&self) -> Vec<(usize, usize, Option<AmphipodType>)>
    {
        let mut results = Vec::new();
        for (room_i, room) in self.rooms.iter().enumerate() {
            for (slot_i, slot) in room.slots.iter().enumerate() {
                let amphi_type;
                if slot.is_some() {
                    amphi_type = Some(slot.unwrap().atype);
                } else {
                    amphi_type = None;
                }
                results.push((room_i, slot_i, amphi_type));
            }
        }
        return results;
    }

    fn new() -> Self {
        let mut rooms = Vec::new();

        rooms.push(Room::new("left", Property::Wait, 2));
        let left = rooms.len() - 1;
        rooms.push(Room::new("amber", Property::Destination(AmphipodType::Amber), 4));
        let amber = rooms.len() - 1;
        rooms.push(Room::new("ab", Property::Corridor, 1));
        let ab = rooms.len() - 1;
        rooms.push(Room::new("bronze", Property::Destination(AmphipodType::Bronze), 4));
        let bronze = rooms.len() - 1;
        rooms.push(Room::new("bc", Property::Corridor, 1));
        let bc = rooms.len() - 1;
        rooms.push(Room::new("copper", Property::Destination(AmphipodType::Copper), 4));
        let copper = rooms.len() - 1;
        rooms.push(Room::new("cd", Property::Corridor, 1));
        let cd = rooms.len() - 1;
        rooms.push(Room::new("desert", Property::Destination(AmphipodType::Desert), 4));
        let desert = rooms.len() - 1;
        rooms.push(Room::new("right", Property::Wait, 2));
        let right = rooms.len() - 1;

        rooms[left].connections.push(Connection::new(amber, 2));
        rooms[left].connections.push(Connection::new(ab, 2));

        rooms[amber].connections.push(Connection::new(left, 2));
        rooms[amber].connections.push(Connection::new(ab, 2));

        rooms[ab].connections.push(Connection::new(left, 2));
        rooms[ab].connections.push(Connection::new(amber, 2));
        rooms[ab].connections.push(Connection::new(bronze, 2));
        rooms[ab].connections.push(Connection::new(bc, 2));

        rooms[bronze].connections.push(Connection::new(ab, 2));
        rooms[bronze].connections.push(Connection::new(bc, 2));

        rooms[bc].connections.push(Connection::new(ab, 2));
        rooms[bc].connections.push(Connection::new(bronze, 2));
        rooms[bc].connections.push(Connection::new(copper, 2));
        rooms[bc].connections.push(Connection::new(cd, 2));

        rooms[copper].connections.push(Connection::new(bc, 2));
        rooms[copper].connections.push(Connection::new(cd, 2));

        rooms[cd].connections.push(Connection::new(bc, 2));
        rooms[cd].connections.push(Connection::new(copper, 2));
        rooms[cd].connections.push(Connection::new(desert, 2));
        rooms[cd].connections.push(Connection::new(right, 2));

        rooms[desert].connections.push(Connection::new(cd, 2));
        rooms[desert].connections.push(Connection::new(right, 2));

        rooms[right].connections.push(Connection::new(desert, 2));
        rooms[right].connections.push(Connection::new(cd, 2));

        return Map{rooms: rooms};
    }

    fn render(&self) {
        let mut row = 0;
        let mut all_done = false;
        while !all_done {
            all_done = true;
            for (room_i, room) in self.rooms.iter().enumerate() {
                match room.property {
                    Property::Destination(_) => {
                        if row == 0 {
                            print!(".");
                        }
                        else {
                            if row - 1 < room.slots.len() {
                                all_done = false;
                                let slot = room.slots[row - 1];
                                if slot.is_some() {
                                    print!("{}", slot.unwrap().atype.representation());
                                } else {
                                    print!(".");
                                }
                            }
                        }
                    }
                    Property::Wait | Property::Corridor => {
                        if row == 0 {
                            all_done = false;
                            // This case is a bit ugly, but genrates much nicer representation
                            if room_i == 0 {
                                for slot in room.slots.iter().rev() {
                                    if slot.is_some() {
                                        print!("{}", slot.unwrap().atype.representation());
                                    } else {
                                        print!(".");
                                    }
                                }
                            } else {
                                for slot in &room.slots {
                                    if slot.is_some() {
                                        print!("{}", slot.unwrap().atype.representation());
                                    } else {
                                        print!(".");
                                    }
                                }
                            }
                       
                        }
                        else {
                            for _slot in &room.slots {
                                print!(" ");
                            }
                        }
                    }
                };
            }
            row += 1;
            println!("");
        }
    }
}


#[derive(Clone)]
struct ComplexState {
    map: Map, 
    cost: i64,  
    heur_cost: i64,
    prev: Option<Box<ComplexState>>,
}

impl Eq for ComplexState{

}

impl PartialEq for ComplexState {
    fn eq(&self, other: &Self) -> bool {
        self.cost + self.heur_cost == other.cost + other.heur_cost
    }
}

impl PartialOrd for ComplexState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ComplexState {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cost + self.heur_cost).cmp(&(other.cost + other.heur_cost))
    }
}

impl ComplexState {
    fn render(&self, step: i32) -> i32{
        let step_shown;
        if self.prev != None {
            step_shown = self.prev.as_ref().unwrap().render(step + 1);
        } else {
            step_shown = 1;
        }
        println!("{:2}) -- Energy Used: {}", step_shown, self.cost);
        self.map.render();
        if step == 0 {
            println!("Tot. Cost: {}", self.cost);
        }
        return step_shown + 1;
    }

    fn heuristic(&self) -> i64{
        let mut heur_cost = 0;
            
        for room in &self.map.rooms {
            for slot in room.slots.iter() {
                if slot.is_some() {
                    heur_cost += match room.property {
                        Property::Destination(dtype) => {
                            let a = slot.unwrap(); 
                            if a.atype != dtype {
                                a.atype.step_cost() * 4
                            } else {
                                0
                            }
                        }
                        Property::Corridor | Property::Wait => {
                            let a = slot.unwrap(); 
                            a.atype.step_cost() * 2
                        }
                    };
                }
            }
        }
        return heur_cost;
    }

    fn is_done(&self) -> bool {
        for room in &self.map.rooms {
            match &room.property {
                Property::Destination(atype) => {
                    for slot in &room.slots {
                        if slot.is_some() {
                            if slot.unwrap().atype != *atype {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
                _ => {},
            };
        }
        return true;
    }

    fn next_states(&self) -> Vec<ComplexState> {
        let mut nexts: Vec<ComplexState> = Vec::new();

        fn expand_from_room(room_i: usize, 
                            parent: &ComplexState, 
                            amphi: &Amphipod, 
                            dest_only: bool, 
                            extra_steps: i64, 
                            prev_room_i: Option<usize>) -> Vec<(ComplexState, usize)> {
            let mut nexts: Vec<(ComplexState, usize)> = Vec::new();
            let room = &parent.map.rooms[room_i];
            for con in &room.connections {
                let dest_i = con.target;
                if prev_room_i == Some(dest_i) {
                    continue;
                }
                let dest_room = &parent.map.rooms[dest_i];
                match dest_room.property {
                    Property::Wait | Property::Corridor => {
                        for slot_i in 0 .. dest_room.slots.len() {
                            let slot = &dest_room.slots[slot_i];
                            if slot.is_some() {
                                break;
                            }
                            let mut new_state = parent.clone();
                            new_state.map.rooms[room_i].remove(amphi.id);
                            new_state.map.rooms[dest_i].slots[slot_i] = Some(amphi.clone());   
                            new_state.cost += (con.steps + extra_steps + slot_i as i64) * amphi.atype.step_cost();
                            new_state.heur_cost = new_state.heuristic();
                            if dest_room.property == Property::Corridor {
                                let mut subnexts = expand_from_room(dest_i, &new_state, &amphi, dest_only, slot_i as i64, Some(room_i));
                                nexts.append(&mut subnexts);
                            }
                            if !dest_only {
                                nexts.push((new_state, dest_i));
                            }                    
                        }
                    }
                    Property::Destination(dest_atype) => {
                        if amphi.atype == dest_atype {
                            let mut deepest = None;
                            for slot_i in 0 .. dest_room.slots.len() {
                                let slot = &dest_room.slots[slot_i];
                                if slot.is_some() {
                                    if slot.unwrap().atype != dest_atype {
                                        deepest = None;
                                        break;
                                    }
                                } else {
                                    deepest = Some(slot_i);
                                }
                            }
                            if deepest.is_some() {
                                let mut new_state = parent.clone();
                                new_state.map.rooms[room_i].remove(amphi.id);
                                new_state.map.rooms[dest_i].slots[deepest.unwrap()] = Some(amphi.clone());   
                                new_state.cost += (con.steps + extra_steps + deepest.unwrap() as i64) * amphi.atype.step_cost();
                                new_state.heur_cost = new_state.heuristic();

                                nexts.push((new_state, dest_i));
                                  
                            }
                        }
                    }
                }
            } 
            return nexts;   
        }

        

        for room_i in  0 .. self.map.rooms.len() {
            let room = &self.map.rooms[room_i];
            for slot_i in 0 .. room.slots.len() {
                let slot = room.slots[slot_i];
                if slot.is_some() {
                    let amphi = &slot.unwrap();
                    // Filter sources, creatures already in goal should never move
                    match room.property {
                        Property::Destination(dest_atype) => {
                            if dest_atype == amphi.atype {
                                if room.slots.iter().filter(|s| s.is_some()).all(|s| s.unwrap().atype == amphi.atype) {
                                   break;
                                }
                            }
                        }  
                        _ => {}
                    };

                    let dest_only = match room.property {
                        Property::Wait | Property::Corridor => true,
                        Property::Destination(_) => false,
                    };
                    let mut new_nexts =  expand_from_room(room_i, &self, &amphi, dest_only, slot_i as i64, None);

                    // Can move directly to a room? Do so!
                    let mut had_one_dest = false;
                    for (_sn, dest_i) in &new_nexts {    
                        if match self.map.rooms[*dest_i].property { Property::Destination(_) => true, _=> false} {
                            had_one_dest = true;
                            break;
                        }
                    }
                    for (sn, dest_i) in &mut new_nexts {
                        if !had_one_dest || match self.map.rooms[*dest_i].property { Property::Destination(_) => true, _=> false} {
                           //sn.prev = Some(Box::new(self.clone()));
                           nexts.push(sn.clone());
                        }
                    }
                    break;
                }
            }
        } 

        return nexts;
    }
    
    fn new(map: Map) -> Self {
        let mut state = ComplexState{map: map, cost: 0, heur_cost: 0, prev: None};
        state.heur_cost = state.heuristic();
        return state;
    }
}

fn parse(data: &String) -> ComplexState {

    let mut map = Map::new();
    fn insert_into_room(name: &str, amphi: Amphipod, map: &mut Map) {
        for room in &mut map.rooms {
            if room.name == name {
                for slot_i in 0 .. room.slots.len() {
                    if room.slots[slot_i].is_none() {
                        room.slots[slot_i] = Some(amphi);
                        return;
                    }
                }
            }
        }
        panic!("Room: {}", name);
    }

    let mut id = 0;
    let mut lines = Vec::new();
    for line in data.lines() {
        lines.push(line.chars().as_str());
    }
    // Inject raw string data¨
    lines.insert(3, "  #D#C#B#A#  ");
    lines.insert(4, "  #D#B#A#C#  ");

    for line in lines {
        for c in line.chars() {
            let name = match id % 4{
                0 => "amber",
                1 => "bronze",
                2 => "copper",
                3 => "desert",
                _ => panic!(),
            };
    
            match c {
                'A' => { 
                    insert_into_room(name, Amphipod::new(id,  AmphipodType::Amber), &mut map);
                    id += 1;
                },
                'B' => { 
                    insert_into_room(name, Amphipod::new(id,  AmphipodType::Bronze), &mut map); 
                    id += 1;
                },
                'C' => { 
                    insert_into_room(name, Amphipod::new(id,  AmphipodType::Copper), &mut map); 
                    id += 1;
                },
                'D' => { 
                    insert_into_room(name, Amphipod::new(id,  AmphipodType::Desert), &mut map); 
                    id += 1;
                },
                _ => {},
            }
        }
    }
    return ComplexState::new(map);
}


fn main() {
    let inp = read_input();
    let complex_state = parse(&inp);

    let mut pqueue: BinaryHeap<Reverse<ComplexState>> = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut best: Option<ComplexState> = None;

    pqueue.push(Reverse(complex_state));
    
    while pqueue.len() > 0 {
        let state = pqueue.pop().unwrap().0;
        
        let new_state_positions = state.map.positions();
        if !seen.insert(new_state_positions){
            continue;
        }

        if best.is_none() || best.as_ref().unwrap().heur_cost > state.heur_cost {
            best = Some(state.clone());
            state.render(0);
        }

        if state.is_done() {
            state.render(0);
            println!("Least Energy Cost: {}", state.cost);
            break;
        }
        let new_states = state.next_states();
        for new_state in new_states{
            pqueue.push(Reverse(new_state));
        }
    }
}