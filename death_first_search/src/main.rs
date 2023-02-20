use std::{collections::VecDeque, fmt::Debug, io};

/// Shorthand for adding parsing into desired type and calling panic
/// with clear message where the error ocurred.
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().expect(&format!(
            "MACRO parse_input: Could not parse given line into '{}'",
            stringify!($t)
        ))
    };
}

const MAX_NODE_AMOUNT: usize = 500;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let inputs = read_line();
    let inputs = inputs.split(' ').collect::<Vec<_>>();

    // the total number of nodes in the level, including the gateways
    let n = parse_input!(inputs[0], usize);

    // the number of links
    let l = parse_input!(inputs[1], usize);

    // the number of exit gateways.
    let e = parse_input!(inputs[2], usize);

    // Whole map of the network.
    const INIT: Option<Node> = None;
    let mut n_map: [Option<Node>; MAX_NODE_AMOUNT] = [INIT; MAX_NODE_AMOUNT];

    // read the input about node connections and build the map.
    for _ in 0..l {
        let inputs = read_line();
        let inputs = inputs.split(' ').collect::<Vec<_>>();

        // N1 and N2 defines a link between these nodes
        let nodes = (
            parse_input!(inputs[0], usize),
            parse_input!(inputs[1], usize),
        );

        match n_map[nodes.0] {
            Some(ref mut n) => n.add_conn(nodes.1),
            None => {
                let mut node = Node::new(nodes.0);
                node.add_conn(nodes.1);
                n_map[nodes.0] = Some(node);
            }
        };

        match n_map[nodes.1] {
            Some(ref mut n) => n.add_conn(nodes.0),
            None => {
                let mut node = Node::new(nodes.1);
                node.add_conn(nodes.0);
                n_map[nodes.1] = Some(node);
            }
        };
    }

    let gateways: Vec<_> = (0..e)
        .map(|_| parse_input!(read_line().trim(), usize))
        .collect();

    // game loop
    loop {
        // The index of the node on which the Bobnet agent is positioned this turn
        let si = parse_input!(read_line(), usize);
        let nsi = n_map[si].as_ref().unwrap().clone();

        // first lets try to capture the easiest case
        {
            if nsi.conns.len() < 2 && !nsi.conns.is_empty() {
                let d_node = n_map[nsi.conns[0]].as_ref().unwrap().id;
                println!("{} {}", nsi.id, d_node);

                disconnect(&mut n_map, nsi.id, d_node);
                remove(&mut n_map, n);
            }
        }

        // now we disconnect the immediate gatways connected to Bobnet
        {
            let nsic_id = nsi.id;
            let d_node_ids = nsi
                .conns
                .iter()
                .filter(|n| gateways.contains(*n))
                .map(|nid| n_map[*nid].as_ref().unwrap().id)
                .collect::<Vec<_>>();

            match d_node_ids.get(0) {
                Some(d_node) => {
                    println!("{} {}", nsic_id, d_node);

                    disconnect(&mut n_map, nsic_id, *d_node);
                    remove(&mut n_map, n);
                    continue;
                }
                None => eprintln!("INFO: no immediate gateways connected to Bobnet."),
            }
        }

        // finally, more advanced
        {
            let nsi_id = nsi.id;
            let mut scan_pool: VecDeque<_> = nsi.conns.iter().map(|n| (*n, nsi_id)).collect();

            let mut c_node = nsi_id; // current node

            let mut n_pool: [(Option<usize>, Option<usize>); MAX_NODE_AMOUNT] =
                [(None, None); MAX_NODE_AMOUNT];
            n_pool[c_node] = (Some(c_node), None);

            // Breadth-first search (BFS)
            while let Some(ref node_info) = scan_pool.pop_front() {
                let node = n_map[node_info.0].as_ref().unwrap().clone();

                // add scanned node to n_pool
                n_pool[node.id] = (Some(node.id), Some(node_info.1));

                // test if one of gateways
                if gateways.contains(&node_info.0) {
                    c_node = node.id;
                    break;
                }

                // add to scan_pool if scanned node is not gateway
                for subnode in node.conns.iter() {
                    let res = scan_pool.iter().find(|(n, _ni)| {
                        n_map[*n].as_ref().unwrap().id == n_map[*subnode].as_ref().unwrap().id
                    });

                    if n_pool[*subnode].0.is_none() && res.is_none() {
                        scan_pool.push_back((*subnode, node.id));
                    }
                }
            }

            let (c_node, p_node) = (n_pool[c_node].0.unwrap(), n_pool[c_node].1.unwrap());

            println!("{} {}", c_node, p_node);
            disconnect(&mut n_map, p_node, c_node);
            remove(&mut n_map, n);
        }
    }

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");
}

/// Removes a connection between the 2 nodes and possibly drops the node from the global map (only
/// if [NodeRef](#NodeRef) strong count is 0)
///
/// # Arguments
///
/// * `n_map` - Global map of the network. Used for complete removal of banished nodes.
/// * `node_a_id` - id to node_a
/// * `node_b_id` - id to node_b
///
/// # Panics
///
/// May panic if given IDs are over MAX_NODE_AMOUNT or lead to non-valid node in global map.
/// NOTE: May be good idea to make this return Result instead of panic.
fn disconnect(n_map: &mut [Option<Node>], node_a_id: usize, node_b_id: usize) {
    n_map[node_a_id].as_mut().unwrap().remove_conn(node_b_id);
    n_map[node_b_id].as_mut().unwrap().remove_conn(node_a_id);
}

/// Removes disconnected node from Global map
///
/// # Panics
///
/// May panic if unable retrieve NodeRef.
fn remove(n_map: &mut [Option<Node>], nc: usize) {
    let d_nodes: Vec<_> = n_map[..nc]
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_some())
        .map(|(i, n)| (i, n.as_ref().unwrap()))
        .filter(|(_, n)| n.conns.is_empty())
        .map(|(i, _n)| i)
        .collect();

    for idn in d_nodes {
        n_map[idn] = None;
    }
}

/// reads a single line from STDIN into String.
/// TODO: add error capturing so that it panics on empty line.
fn read_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

#[derive(Default, Clone, Eq)]
struct Node {
    id: usize,
    conns: VecDeque<usize>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            conns: VecDeque::new(),
        }
    }

    /// method for quicker access to Vec::push
    #[inline]
    fn add_conn(&mut self, node: usize) {
        self.conns.push_back(node);
    }

    #[inline]
    fn remove_conn(&mut self, node: usize) {
        let node_index = self
            .conns
            .iter()
            .enumerate()
            .find(|(_i, n)| **n == node)
            .unwrap()
            .0;
        self.conns.remove(node_index);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ id: {}, conns: {:?}}}", self.id, self.conns)
    }
}
