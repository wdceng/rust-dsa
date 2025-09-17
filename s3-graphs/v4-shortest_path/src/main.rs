use core::fmt;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

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

impl std::fmt::Display for GraphErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mess)   // <-- now the field is actually *read*
    }
}

// Trait for weighted values with a weight method, implemented here so i32 returns itself as the weight
pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

// Route node in a path with position, optional previous link and total length
#[derive(Debug)]
pub struct Route<ID> {
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}

// Constraint: IDß must support equality comparisons (==) so routes can compare positions
impl<ID: Eq> Route<ID> {
    // Create and return a new starting Route wrapped in Rc with no path and zero length
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }

    // Check if the given ID exists in this route or any of its previous links
    pub fn contains(&self, id: &ID) -> bool {
        if self.pos == *id {
            return true;
        }
        // If there is a previous route, check it recursively; otherwise return false
        match self.path {
            Some(ref p) => p.contains(id),
            None => false,
        }
    }
}

// Implement Display for Route so it can be formatted as a string using its path, length, and position
impl<ID: fmt::Debug> fmt::Display for Route<ID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref p) = self.path {
            write!(f, "{}-{}-", p, self.len)?;
        }
        write!(f, "{:?}", self.pos)
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

impl<T, E: Weighted, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    // Find the shortest path from `from` to `to`, returning an Rc<Route> if one exists
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        // Keep track of visited node IDs to avoid revisiting
        let mut visited = HashSet::new();
        // Candidate paths to explore
        let mut routes = Vec::new();
        // Start with a route beginning at `from`
        routes.push(Route::start_rc(from));
        // Main search loop
        loop {
            // Take the next route to explore (or return None if no more)
            let c_route = routes.pop()?;
            // If we've reached the target node, return the current route
            if to == c_route.pos {
                return Some(c_route);
            }
            // Skip if we've already visited this node
            if visited.contains(&c_route.pos) {
                // No point in searching from the same place twice
                continue;
            }
            // Mark the current node as visited
            visited.insert(c_route.pos.clone());

            // Get the node data for the current route position (return None if missing)
            let exist = self.data.get(&c_route.pos)?;
            // Iterate over all edge IDs connected to this node
            for eid in &exist.1 {
                // Look up the edge details by its ID (return None if missing)
                let edge = self.edges.get(eid)?;
                // Determine the neighbor node: if edge.1 is current pos, use edge.2, else use edge.1
                let npos = if edge.1 == c_route.pos {
                    // Choose the opposite side of the edge from the current position
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                // Compute the new path length as current length plus this edge’s weight
                let nlen = c_route.len + edge.0.weight();
                // Create a new Route pointing to the neighbor, with updated length and path back to current route
                let nroute = Rc::new(Route {
                    pos: npos,
                    len: nlen,
                    path: Some(c_route.clone()), // clone Rc to increase reference count
                });
                if routes.len() == 0 {
                    routes.push(nroute);
                    continue;
                }
                // Insert this new route into the candidate list in sorted order
                let mut iafter = routes.len() - 1;
                loop {
                    if routes[iafter].len > nlen {
                        // Lowest element last 
                        routes.insert(iafter + 1, nroute);
                        break;
                    }
                    if iafter == 0 {
                        // Reached end
                        routes.insert(0, nroute);
                        break;
                    }
                    iafter -= 1;
                }
            }
        }
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

    println!("Shortest path A-D = {}", g.shortest_path('A', 'D').unwrap());
    println!("Shortest path H-B = {}", g.shortest_path('H', 'B').unwrap());

    Ok(())
}
