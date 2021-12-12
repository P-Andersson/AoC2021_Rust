use std::fs;
use std::collections::HashMap;

fn read_input() -> String {
    let inputs =  fs::read_to_string("Day12/input").unwrap()
                       .replace("\r", "");
    
    return inputs;
}

#[derive(Debug)] #[derive(PartialEq)]
enum NodeType {
    Big,
    Small,
}

#[derive(Debug)]
struct Node {
    name: String,
    node_type: NodeType,
    links: Vec<usize>,
}

#[derive(Debug)]
struct NodeGraph {
    nodes : Vec<Node>,
    names : HashMap<String, usize>, 
}

type Path = Vec<usize>;

impl NodeGraph {
    pub fn new() -> NodeGraph {
        return NodeGraph{
            nodes: Vec::new(),
            names: HashMap::new(),
        }
    }

    pub fn add(&mut self, node: Node) -> usize {
        let id = self.nodes.len(); 
        self.names.insert(node.name.clone(), id);

        self.nodes.push(node);

        return id;
      
    }

    pub fn link(&mut self, node1: usize, node2: usize) {
        self.get_mut(node1).links.push(node2);
        self.get_mut(node2).links.push(node1);
    }

    pub fn get(&self, id: usize) -> &Node{
        return &self.nodes[id];
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Node{
        return self.nodes.get_mut(id).unwrap();
    }

    pub fn contains(&self, id: usize) -> bool {
        return id < self.nodes.len();
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<&usize> {
        return self.names.get(name);
    }

    pub fn explore(&self) -> Vec<Path> {
        type VisitedToExitIndex = HashMap<usize, Vec<usize>>;

        #[derive(Clone)]
        struct ExploringPath{ visisted: VisitedToExitIndex, path: Path }
        
        let mut expl_stack: Vec<ExploringPath> = Vec::new();

        let start = *self.get_id_by_name("start").unwrap();
        expl_stack.push(ExploringPath{visisted: VisitedToExitIndex::new(), path: [start].to_vec()});

        let mut exit_paths: Vec<Path> = Vec::new();

        while expl_stack.len() > 0 {
            let cur = expl_stack.pop().unwrap();
            let last = cur.path[cur.path.len() - 1];
            let cur_node = self.get(last);

            if cur_node.name == "end" {
                exit_paths.push(cur.path);
                continue;
            }

            let empty_default = Vec::new();
            let explored = cur.visisted.get(&last).unwrap_or(&empty_default); 

            //println!("Exploring: {:?}", cur.path.iter().map(|n| &self.get(*n).name ).collect::<Vec<&String>>());
            //println!("H. Visits: {:?}", explored.iter().map(|n| &self.get(*n).name ).collect::<Vec<&String>>());

            for exit_index in 0 .. cur_node.links.len() {
                let exit_id = cur_node.links[exit_index];
                if explored.contains(&exit_id) {
                    continue;
                }     
                
                let next = self.get(exit_id);
                if next.node_type != NodeType::Small || !cur.visisted.contains_key(&exit_id) {
                    let mut next_path = cur.clone();
                    next_path.path.push(exit_id);
                    let mut new_visisted = explored.clone();
                    new_visisted.push(exit_id);
                    next_path.visisted.insert(last, new_visisted);

                    expl_stack.push(next_path);
                }
                
            } 
        }

        return exit_paths;
    }
}

fn parse(data: &String) -> NodeGraph {
    let mut nodes: NodeGraph = NodeGraph::new();

    for l in data.lines() {
        let mut prev: Option<usize> = None;
        for name in l.split('-') {
            if nodes.get_id_by_name(name) == None  {
                let node_type = 
                    if name.chars().all(|c| c.is_uppercase()) {NodeType::Big}
                    else {NodeType::Small};

                let new_node = Node{
                    name: name.to_string(),
                    node_type: node_type,
                    links: Vec::new(),
                    }; 
                nodes.add(new_node);
            }
            let node_id = *nodes.get_id_by_name(name).unwrap();
            if !prev.is_none() {
                let prev_node_id = prev.unwrap();
                nodes.link(prev_node_id, node_id);
            }
            prev = Some(node_id);
        }
    }
    return nodes;
}

fn main() {
    let inp = read_input();
    let nodes = parse(&inp);

    let paths = nodes.explore();

    /*for path in &paths {
        println!("{:?}", path.iter().map(|n| &nodes.get(*n).name ).collect::<Vec<&String>>());
    }*/
    println!("Solutions: {}", paths.len());
}