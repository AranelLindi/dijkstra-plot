use std::cmp::Ordering;
//use std::process::Output;

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
    const THRESHOLD: f32 = 0.001; // minimum displacement
    // Force constants:
    const K: f32 = 0.01; // repulsion
    const A: f32 = 0.1; // attraction

    fn init(graph: &'a Graph<'a>, start: &'a Node) -> Vec<NodePos> {
        let mut positions: Vec<NodePos> = Vec::new();

        // Closure is used to return initial coordinates for each node by A specific logic so the algorithm doesn't get stuck.
        let init_logic = || -> (f32, f32) {
            let mut rng = rand::thread_rng();

            let x: f32 = rng.gen_range(0.0..=1.0);
            let y: f32 = rng.gen_range(0.0..=1.0);

            (x, y)
            /*match i % 4 {
                0 => (i as f32, 0.0),    /* 1st quadrant */
                1 => (0.0, i as f32),    /* 2nd quadrant */
                2 => (-(i as f32), 0.0), /* 3rd quadrant */
                _ => (0.0, -(i as f32)), // case: 3 (_ must be used!) /* 4th quadrant */
            }*/
        };

        // Iterates through
        for (_, e) in graph.nodes.iter().enumerate() {
            if e == start {
                // Start node shall be in center of graphical representation.
                positions.push(NodePos::new(start.no(), 0.0, 0.0, 0.0, 0.0));
            } else {
                // Other nodes are initialized around start node with different (not random) coordinates.
                let (x, y) = init_logic();

                positions.push(NodePos::new(e.no(), x, y, 0.0, 0.0));
            }
        }

        // Sort vector by NodePos.no to enable index-based access (so positions[x] -> node_x.
        positions.sort_by(|a, b| a.partial_cmp(b).unwrap()); // TODO: check if this is really necessary to map!

        return positions;
    }

    pub fn run(graph: &'a Graph<'a>, start: &'a Node) -> Vec<NodePos> {
        let mut positions: Vec<NodePos> = Self::init(graph, start);
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
        let calc_attraction = |dist_vec: &(f32, f32), weight: u32| -> (f32, f32) {
            // Formula: -K/r^3 * norm(r) * (1-r/weight)

            if weight == 0 {
                // If edge weight is 0 (maybe no edge was found in graph.edges) return 0 so repulsion force doesn't exist.
                (0.0, 0.0)
            } else {
                let vec = dist_vec.clone();
                // Interim results:
                //let norm = norm(dist_vec); // normalized vector
                //let amount = amount(dist_vec); // amount of distance vector
                //let amount_third = amount.powi(3); // amount of distance vector to power of 3
                //let scalar = -Self::K / amount_third;// * (1.0 - amount / (weight as f32)); //

                //norm.multiply_scalar(scalar) // operator overloading: multiplies scalar with tuple of size 2!

                vec.multiply_scalar(Self::A)
            }
        };

        // Calculates attraction force with respect to distance vector.
        let calc_repulsion = |dist_vec: &(f32, f32)| -> (f32, f32) {
            let norm = norm(dist_vec);
            let amount = amount(dist_vec);
            let amount_third = amount.powi(3);
            let scalar = -Self::K / amount_third;

            norm.multiply_scalar(scalar)


            // Formula: A * r

            //dist_vec.multiply_scalar(Self::A) // operator overloading: multiplies scalar with tuple of size 2!

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
            let mut total_displacement: f32 = 0.0;

            for i in 0..node_len /* TODO: Be careful! This condition forces the graph to have at least one node! */ {
                let mut v = positions[i].pos;
                let mut dv = positions[i].vel;

                let mut tmp_old = v;

                // Reset dv vector:
                dv = (0.0, 0.0); // Could be disabled if previous iteration should still be have an effect to current node

                for j in 0..node_len {
                    if i == j { continue; }

                    let u = positions[j].pos;

                    let distance = u.subtract_sub(v);

                    let src_no = positions[i].no;
                    let dst_no = positions[j].no;

                    // Calculate forces:
                    let (f_ax, f_ay) = calc_repulsion(&distance);
                    let (f_rx, f_ry) = calc_attraction(&distance, get_weight(src_no, dst_no));
                    let (f_resx, f_resy) = (f_ax + f_rx, f_ay + f_ry);

                    let (dv_x, dv_y) = dv;

                    dv = (dv_x + f_resx, dv_y + f_resy);
                }

                // Update position
                let ((x, y), (dx, dy)) = (v, dv.multiply_scalar(Self::DT));
                positions[i].pos = (x + dx, y + dy);

                // Difference vector between old and new position:
                let ((x, y), (x_old, y_old)) = (v, tmp_old);
                let diff = (x - x_old, y - y_old);
                let diff_amount = amount(&diff);

                // update displacement:
                total_displacement += diff_amount as f32;

                // If displacement is smaller than threshold the algorithm is finished!
                //if total_displacement < Self::THRESHOLD {
                //    print!("Schwellwert!");
                //    break;
                //}
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
