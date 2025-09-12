use std::collections::HashMap;
use std::hash::Hash;

// Simple custom error type with a message field and a constructor
#[derive(Debug)]
pub struct GraphErr {
    mess: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr {
            mess: s.to_string(),
        }
    }
}

// MapPointer based graph
#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

// Impl for Graph with generic node data T, edge data E, and clonable+hashable+equatable IDs
impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    // Returns an empty graph with no nodes and no edges.
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, dt: T) {
        // Node has no edges yet
        // Insert a new node with the given ID and data, starting with no edges.
        self.data.insert(id, (dt, Vec::new()));
    }

    pub fn add_edge(&mut self, ed_id: ID, from: ID, to: ID, edat: E) -> Result<(), GraphErr> {
        // Return error if the 'from' node ID does not exist in the graph
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new("'from' not in nodes"));
        }
        // Try to get a mutable reference to the value for key `to` in the map, and bind it as `dt` if it exists
        if let Some(ref mut dt) = self.data.get_mut(&to) {
            // Insert the new edge into the edges map
            self.edges.insert(ed_id.clone(), (edat, from.clone(), to));
            // Add the edge ID to the 'to' node's adjacency list
            dt.1.push(ed_id.clone());
        } else {
            // Return error if the 'to' node ID does not exist in the graph
            return Err(GraphErr::new("'to' not in nodes"));
        }

        // Add `ed_id` to the adjacency list of node `from`, panicking if `from` is missing
        self.data.get_mut(&from).unwrap().1.push(ed_id);
        Ok(())
    }
}

fn main() -> Result<(), GraphErr> {
    let mut g = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, ());
    }

    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;

    println!("Hello, graph {:?}", g);
    Ok(())
}
