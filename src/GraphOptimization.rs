use std::cmp::Ordering;
use std::process::Output;

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
pub struct NodePos {
    pub no: u32,
    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl NodePos {
    fn new(no: u32, x: f32, y: f32, dx: f32, dy: f32) -> Self {
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
    const THRESHOLD: f64 = 0.01; // minimum displacement
    // Force constants:
    const K: f32 = 0.5; // repulsion
    const A: f32 = 0.15; // attraction

    fn init(graph: &'a Graph<'a>, start: &'a Node) -> Vec<NodePos> {
        let mut positions: Vec<NodePos> = Vec::new();

        // Closure is used to return initial coordinates for each node by A specific logic so the algorithm doesn't get stuck.
        let init_logic = |i: usize| -> (f32, f32) {
            let mut rng = rand::thread_rng();

            let x: f32 = rng.gen_range(0.0..=10.0);
            let y: f32 = rng.gen_range(0.0..=10.0);

            (x, y)
            /*match i % 4 {
                0 => (i as f32, 0.0),    /* 1st quadrant */
                1 => (0.0, i as f32),    /* 2nd quadrant */
                2 => (-(i as f32), 0.0), /* 3rd quadrant */
                _ => (0.0, -(i as f32)), // case: 3 (_ must be used!) /* 4th quadrant */
            }*/
        };

        // Iterates through
        for (i, e) in graph.nodes.iter().enumerate() {
            if e == start {
                // Start node shall be in center of graphical representation.
                positions.push(NodePos::new(start.no(), 0.0, 0.0, 0.0, 0.0));
            } else {
                // Other nodes are initialized around start node with different (not random) coordinates.
                let (x, y) = init_logic(i);

                positions.push(NodePos::new(e.no(), x, y, 0.0, 0.0));
            }
        }

        // Sort vector by NodePos.no to enable index-based access (so positions[x] -> node_x.
        positions.sort_by(|a, b| a.partial_cmp(b).unwrap()); // TODO: check if this is really necessary to map!

        return positions;
    }

    pub fn run(graph: &'a mut Graph<'a>, start: &'a Node) -> Vec<NodePos> {
        let positions: Vec<NodePos> = Self::init(graph, start);
        let node_len = graph.node_len;

        // Returns normalized vector (tuple of size 2).
        let norm = |x: &(f32, f32)| -> (f32, f32) {
            let (a, b) = x; // unpacking tuple
            let value = f32::sqrt(a.powi(2) + b.powi(2));

            (value * a, value * b)
        };

        // Returns amount of vector (tuple of size 2).
        let amount = |x: &(f32, f32)| -> f32 {
            let (a, b) = x; // unpacking tuple

            f32::sqrt(a.powi(2) + b.powi(2))
        };

        // Calculates repulsion force with respect on distance vector and edge weight.
        let calc_repulsion = |dist_vec: &(f32, f32), weight: u32| -> (f32, f32) {
            // Formula: -K/r^3 * norm(r) * (1-r/weight)

            if weight == 0 {
                // If edge weight is 0 (maybe no edge was found in graph.edges) return 0 so repulsion force doesn't exist.
                (0.0, 0.0)
            } else {
                // Interim results:
                let norm = norm(dist_vec); // normalized vector
                let amount = amount(dist_vec); // amount of distance vector
                let amount_third = amount.powi(3); // amount of distance vector to power of 3
                let scalar = -Self::K / amount_third * (1.0 - amount / (weight as f32)); //

                norm.multiply_scalar(scalar) // operator overloading: multiplies scalar with tuple of size 2!
            }
        };

        // Calculates attraction force with respect to distance vector.
        let calc_attraction = |dist_vec: &(f32, f32)| -> (f32, f32) {
            // Formula: A * r

            dist_vec.multiply_scalar(Self::A) // operator overloading: multiplies scalar with tuple of size 2!
        };

        // Returns weight of an edge which connects src-node and dst-node.
        let get_weight = |src: u32, dst: u32| -> u32 {
            let opt_edge = graph
                .edges
                .iter()
                .find(|&x| x.source().no() == src && x.dest().no() == dst);

            match opt_edge {
                Some(edge) => edge.weight(),
                None => 0,
            }
        };

        for _ in 0..=Self::ITERATIONS {
            let mut total_displacement: f64 = 0.0;

            for i in 1..=node_len-1
            /* TODO: Be careful! This condition forces the graph to have at least one node! */
            {
                // Location vector (previous sorting in init function causes that
                let mut v = &positions[i].pos;
                let mut dv = &positions[i].vel;
                let u = &positions[i - 1].pos;

                // Current distance vector:
                let dist = v.subtract_sub(*u); // operator overloading: subtract two tuples of size 2!

                // Calculate force vectors on current node v:
                let repulse_force = calc_repulsion(&dist, get_weight(positions[i].no, positions[i - 1].no));
                let attract_force = calc_attraction(&dist);

                // Result force vector:
                let ((ax, ay), (rx, ry)) = (attract_force, repulse_force); // destructure the tuples ...
                let result_force = (ax + rx, ay + ry); // .. for easier syntax (result force vector)

                // Calculate change vector:
                let ((dx, dy), (fx, fy)) = (dv, result_force); // destructure the tuples ...
                let dv_vec = &(dx + fx, dy + fy); // ... for easier syntax (this line is necessary otherwise the old reference is overwritten and dropped before its scope ends)
                dv = &dv_vec; // update

                // Keep current position in cache for later calculations.
                let v_old = v.clone();

                // Update node position depending on current acceleration/changes:
                let dv_update = dv.multiply_scalar(Self::DT); // changes multiplied with DT
                let ((x, y), (dx, dy)) = (v, dv_update); // destructure the tuples ...
                let v_vec = (x + dx, y + dy); // ... for easier syntax (this line is necessary otherwise old reference would be overwritten before its scope ends
                v = &v_vec; // update

                // Calculate difference in old and new position:
                let ((x, y), (x_old, y_old)) = (v, v_old); // destruct the tuples ...
                let diff_amount = amount(&(x - x_old, y - y_old)); // ... for easier syntax

                // update displacement:
                total_displacement += diff_amount as f64;

                // If displacement is smaller than threshold the algorithm is finished!
                if total_displacement < Self::THRESHOLD {
                    break;
                }
            }
        }
        /*
          Might be that the graph after the algorithm has A global offset.
          To bring it back in center calculate center of gravity vector
          and subtract it from each node.
        */
        positions
    }
}
