// Standard library.
use std::cmp::Ordering;

// Third crates.
use rand::Rng;

// Own objects.
use crate::Graph::{Graph, Node};

// Standard library
use std::ops::{Mul, Sub};

// Trait for multiply tuples of size 2 with scalar constant.
trait TupleScalarMul<RHS> {
    type Output;
    fn multiply_scalar(self, others: RHS) -> Self::Output;
}

// Trait for subtracting tuples of size 2.
trait TupleSub<RHS> {
    type Output; /* defines output in A generic way after implementation in impl-block */
    fn subtract_sub(self, others: RHS) -> Self::Output;
}

// Represents A node in the algorithm.
#[derive(Clone)]
pub struct NodePos {
    pub no: usize,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl NodePos {
    fn new(no: usize, x: f32, y: f32, dx: f32, dy: f32) -> Self {
        NodePos {
            no,
            pos: (x, y),
            vel: (dx, dy),
        }
    }
}

// Implementation of TupleSub-trait to 'tuple of size 2'-construct.
impl<T, RHS> TupleSub<RHS> for (T, T)
// where block defines additional constraints on the types that can implement this trait TupleSub!
where
    T: Sub<Output = T>, /* T must implement the Sub-trait (means that minus operator could used on it) and the output type must also be T (so from same type) */
    RHS: Into<(T, T)>, /* RHS must implement the Into-trait which means that RHS can be converted into A tuple of size 2 containing values of type T */
{
    type Output = (T, T);

    fn subtract_sub(self, others: RHS) -> Self::Output {
        let other = others.into();
        (self.0 - other.0, self.1 - other.1)
    }
}

// Implementation of PartialEq-trait (necessary for sorting)
impl PartialEq<Self> for NodePos {
    fn eq(&self, other: &Self) -> bool {
        self.no.eq(&other.no)
    }
}

// Implementation of TupleScalarMult trait to allow scalar multiplication with tuple of size 2.
impl<T, RHS> TupleScalarMul<RHS> for (T, T)
where
    T: Mul<Output = T> + Copy, /* Copy trait is necessary to ensure that T is A primitive type and can used multiple times without A move is necessary */
    RHS: Into<T>,
{
    type Output = (T, T);

    fn multiply_scalar(self, others: RHS) -> Self::Output {
        let other = others.into();
        (self.0 * other, self.1 * other)
    }
}

// Implementation of PartialOrd-trait (necessary for sorting)
impl PartialOrd for NodePos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.no.partial_cmp(&other.no)
    }
}

pub struct GraphOptimization<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GraphOptimization<'a> {
    // Constants:
    const DT: f32 = 0.1; // time step dt
    const ITERATIONS: usize = 500; // maximum number of ITERATIONS
    const THRESHOLD: f32 = 0.4; // minimum displacement
    // Force constants:
    const K: f32 = 0.6; // repulsion
    const A: f32 = 0.1; // attraction (0.2 - Did also work well!)

    // Preparatory steps for positioning algorithm: Creates a vector in which each node of given graph is associated to a NodePos element with random initial coordinates.
    fn init(graph: &'a Graph<'a>, start: &'a Node) -> Vec<NodePos> /* WC: O(n * (1 + log(n) ) */ {
        let mut positions: Vec<NodePos> = Vec::new();

        // Closure is used to return initial coordinates for each node so the algorithm doesn't get stuck.
        let init_logic = || -> (f32, f32) /* O(1) */ {
            let mut rng = rand::thread_rng();

            // Creates random number in interval [0.0, 1.0]
            let x: f32 = rng.gen_range(0.0..=1.0);
            let y: f32 = rng.gen_range(0.0..=1.0);

            (x, y)
        };

        for (_, e) in graph.nodes.iter().enumerate() /* O(n) */ {
            if e == start {
                // Start node shall be in center of graphical representation.
                positions.push(NodePos::new(start.no(), 0.0, 0.0, 0.0, 0.0));
            } else {
                // Other nodes are initialized around start node with different (not random) coordinates.
                let (x, y) = init_logic();

                positions.push(NodePos::new(e.no(), x, y, 0.0, 0.0));
            }
        }

        // Sort vector by NodePos.no to enable index-based access (so positions[x] -> node_x).
        positions.sort_by(|a, b| a.partial_cmp(b).unwrap()); // WC: O(n * log(n) )

        return positions;
    }

    // Execute positioning algorithm: Fundamental principle is that each node has both repulsion and attraction forces to all other nodes.
    // Algorithm tries to place each node in such way that acting forces become minimal or a maximum of iterations is performed.
    pub fn run(graph: &'a Graph<'a>, start: &'a Node) -> Vec<NodePos> {
        let mut positions: Vec<NodePos> = Self::init(graph, start);
        let node_len = graph.node_len;

        // Returns amount of vector (tuple of size 2).
        let amount = |x: (f32, f32)| -> f32 {
            let (a, b) = x; // unpacking tuple

            f32::sqrt(a.powi(2) + b.powi(2))
        };

        // Returns normalized vector (tuple of size 2).
        let norm = |x: (f32, f32)| -> (f32, f32) {
            let (a, b) = x; // unpacking tuple
            let value = amount(x);

            if value != 0.0 { ((a / value), (b / value)) } else { (0.0, 0.0) }
        };

        // Calculates repulsion force with respect on distance vector and edge weight.
        let calc_attraction = |dist_vec: (f32, f32)| -> (f32, f32) {
            dist_vec.multiply_scalar(Self::A)
        };

        // Calculates attraction force with respect to distance vector.
        let calc_repulsion = |dist_vec: (f32, f32)| -> (f32, f32) {
            let norm = norm(dist_vec);
            let amount = amount(dist_vec);
            let amount_third = amount.powi(3);
            let scalar = -Self::K / amount_third;

            norm.multiply_scalar(scalar)
        };

        // Returns weight of an edge which connects src-node and dst-node.
        let get_weight = |src: usize, dst: usize| -> u32 /* O(n) */ {
            let opt_edge = graph
                .edges
                .iter()
                .find(|&x| (x.source().no() == src && x.dest().no() == dst) || (x.dest().no() == src && x.source().no() == src));

            // Determines whether a connecting edge was found:
            match opt_edge {
                Some(edge) => edge.weight(),
                None => 1, /* Important that NOT 0 is returned! Otherwise nodes without connecting edge would have no or only little distance to each other! Since dijkstra algorithm forces weights greater than zero and integer, 1 is smallest value allowed */
            }
        };

        // Algorithm: Iterate at least ITERATIONS-times (unless quit-condition is fulfilled)
        for _ in 0..=Self::ITERATIONS /* O(1) */ {
            let mut total_displacement: f32 = 0.0; // Gets reset for every new iteration

            for i in 0..node_len /* O(n) TODO: Be careful! This condition forces the graph to have at least one node! */ {
                // Reference to current node's position:
                let v = positions[i].pos;

                // Velocity vector:
                let mut dv: (f32, f32) = (0.0, 0.0); // Reset dv vector (values could also be applied from previous iteration)

                // Calculates resulting forces for node v in which distance to all other nodes are determined.
                for j in 0..node_len /* O(n) */ {
                    if i == j { continue; } // (distance to itself is trivial)

                    // Reference to viewed node's position:
                    let u = &positions[j].pos;

                    // Try to find connecting edge between v and the j-th node.
                    let cur_weight = (get_weight(i, j) as f32);

                    // Calculate distance between both nodes.
                    let distance = u.subtract_sub(v);

                    // Calculate forces:
                    let (f_rx, f_ry) = calc_repulsion(distance);
                    let (f_ax, f_ay) = calc_attraction(distance);
                    let (f_resx, f_resy) = (f_ax + f_rx, f_ay + f_ry);

                    // Update velocity vector:
                    let (dv_x, dv_y) = dv; // unpack (just for next line)
                    dv = (dv_x + (cur_weight * f_resx), dv_y + (cur_weight * f_resy)); // To give edge weight more affection influence, multiply it with calculated resulted force
                }

                // Update position:
                let ((x, y), (dx, dy)) = (v, dv.multiply_scalar(Self::DT)); // origin position plus calculated velocity vector multiplied with dt
                let (x_new, y_new) = (x + dx, y + dy); // unpack (just for next line)
                positions[i].pos = (x_new, y_new); // Update node's position

                // Difference vector between old and new position (for displacement calculation):
                let ((x, y), (x_old, y_old)) = ((x_new, y_new), v); // unpack
                let diff = (x - x_old, y - y_old); // position difference of v
                let diff_amount = amount(diff);

                // update displacement:
                total_displacement += diff_amount;

            }

            // If displacement is smaller than threshold, the algorithm generates only very small changes and can be aborted!
            if total_displacement < Self::THRESHOLD {
                break;
            }
        }
        /* Cost complexity of this algorithm: O(n**3) */

        positions
    }
}
