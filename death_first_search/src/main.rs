use std::{cell::RefCell, collections::VecDeque, fmt::Debug, io, rc::Rc};

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

type NodeRef = Rc<RefCell<Node>>;

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
    // TODO: consider whether to use it in verifying given data.
    let e = parse_input!(inputs[2], usize);

    // Whole map of the network.
    const INIT: Option<NodeRef> = None;
    let mut n_map: [Option<NodeRef>; MAX_NODE_AMOUNT] = [INIT; MAX_NODE_AMOUNT];

    // read the input about node connections and build the map.
    for _ in 0..l {
        let inputs = read_line();
        let inputs = inputs.split(' ').collect::<Vec<_>>();

        // N1 and N2 defines a link between these nodes
        let nodes = (
            parse_input!(inputs[0], usize),
            parse_input!(inputs[1], usize),
        );

        let node_a = match n_map[nodes.0] {
            Some(ref n) => Rc::clone(n),
            None => {
                let tmp = Rc::new(RefCell::new(Node::new(nodes.0)));
                n_map[nodes.0] = Some(Rc::clone(&tmp));
                tmp
            }
        };
        let node_b = match n_map[nodes.1] {
            Some(ref n) => Rc::clone(n),
            None => {
                let tmp = Rc::new(RefCell::new(Node::new(nodes.1)));
                n_map[nodes.1] = Some(Rc::clone(&tmp));
                tmp
            }
        };

        node_a.borrow_mut().add_conn(Rc::clone(&node_b));
        node_b.borrow_mut().add_conn(Rc::clone(&node_a));
    }

    for _ in 0..e {
        let input = parse_input!(read_line().trim(), usize);
        if let Some(ref node) = n_map[input] {
            node.borrow_mut().gate = true;
        }
    }

    // game loop
    loop {
        // The index of the node on which the Bobnet agent is positioned this turn
        let si = parse_input!(read_line(), usize);
        let nsi = Rc::clone(n_map[si].as_ref().unwrap());

        // first lets try to capture the easiest case
        {
            let nsic = nsi.borrow();
            if nsic.conns.len() < 2 && !nsic.conns.is_empty() {
                let d_node = nsic.conns[0].borrow_mut();
                println!("{} {}", nsic.id, d_node.id);

                disconnect(&n_map, nsic.id, d_node.id);
                remove(&mut n_map, n);
            }
        }

        // now we disconnect the immediate gatways connected to Bobnet
        {
            let nsic_id = nsi.borrow().id;
            let d_node_l = nsi
                .borrow()
                .conns
                .iter()
                .filter(|n| n.borrow().gate)
                .map(Rc::clone)
                .collect::<Vec<NodeRef>>();

            if d_node_l.len() > 1 {
                eprintln!("WARNING: We have lost. Bobnet has access to more than")
            }

            match d_node_l.get(0) {
                Some(d_node) => {
                    let cd_node = (**d_node).clone().into_inner();

                    println!("{} {}", nsic_id, cd_node.id);

                    disconnect(&n_map, nsic_id, cd_node.id);
                    remove(&mut n_map, n);
                    continue;
                }
                None => eprintln!("INFO: no immediate gateways connected to Bobnet."),
            }
        }

        // finally, more advanced
        {
            let mut scan_pool: VecDeque<_> = nsi
                .borrow()
                .conns
                .iter()
                .map(|n| (Rc::clone(n), nsi.borrow().id))
                .collect();

            let mut c_node = nsi.borrow().id; // current node

            let mut n_pool: [(Option<usize>, Option<usize>); MAX_NODE_AMOUNT] =
                [(None, None); MAX_NODE_AMOUNT];
            n_pool[c_node] = (Some(c_node), None);

            // Breadth-first search (BFS)
            while let Some(ref node_info) = scan_pool.pop_front() {
                let nic = node_info.0.borrow().id; // node_id_cache
                let ngc = node_info.0.borrow().gate; // node_gate_cache

                // add scanned node to n_pool
                n_pool[nic] = (Some(nic), Some(node_info.1));

                // test if one of gateways
                if ngc {
                    c_node = nic;
                    break;
                }

                // add to scan_pool if scanned node is not gateway
                for subnode in node_info.0.borrow().conns.iter() {
                    let res = scan_pool
                        .iter()
                        .find(|(n, _ni)| n.borrow().id == (*subnode).borrow().id);

                    if n_pool[subnode.borrow().id].0.is_none() && res.is_none() {
                        scan_pool.push_back((Rc::clone(subnode), nic));
                    }
                }
            }

            let mut p_node = c_node; // stores previous node id
            c_node = n_pool[c_node].1.unwrap();

            // Tracing back from BFS
            while let Some(node) = n_pool[c_node].1 {
                p_node = c_node;
                c_node = node;
            }

            println!("{} {}", p_node, c_node);
            disconnect(&n_map, p_node, c_node);
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
fn disconnect(n_map: &[Option<NodeRef>], node_a_id: usize, node_b_id: usize) {
    let node_a = Rc::clone(n_map[node_a_id].as_ref().unwrap());
    let node_b = Rc::clone(n_map[node_b_id].as_ref().unwrap());

    node_a.borrow_mut().remove_conn(&Rc::clone(&node_b));
    node_b.borrow_mut().remove_conn(&Rc::clone(&node_a));
}

/// Removes disconnected node from Global map
///
/// # Panics
///
/// May panic if unable retrieve NodeRef.
fn remove(n_map: &mut [Option<NodeRef>], nc: usize) {
    let d_nodes: Vec<_> = n_map[..nc]
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_some())
        .map(|(i, n)| (i, n.as_ref().unwrap()))
        .filter(|(_, n)| n.borrow().conns.is_empty())
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
    gate: bool,
    conns: VecDeque<NodeRef>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            gate: false,
            conns: VecDeque::new(),
        }
    }

    /// method for quicker access to Vec::push
    #[inline]
    fn add_conn(&mut self, node: NodeRef) {
        self.conns.push_back(node);
    }

    #[inline]
    fn remove_conn(&mut self, node: &NodeRef) {
        let node_index = self
            .conns
            .iter()
            .enumerate()
            .find(|(_i, n)| n.borrow().id == node.borrow().id)
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
        let node_str: Vec<_> = self
            .conns
            .iter()
            .map(|n| format!("Node{}", n.borrow().id))
            .collect();

        write!(
            f,
            "Node {{ id: {}, gate: {}, conns: {:?}}}",
            self.id, self.gate, node_str
        )
    }
}
