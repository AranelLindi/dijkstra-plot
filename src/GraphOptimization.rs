use std::cmp::Ordering;
use std::process::Output;

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
    type Output; /* defines output in a generic way after implementation in impl-block */
    fn subtract_sub(self, others: RHS) -> Self::Output;
}

// Represents a node in the algorithm.
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
    RHS: Into<(T, T)>, /* RHS must implement the Into-trait which means that RHS can be converted into a tuple of size 2 containing values of type T */
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
    T: Mul<Output = T> + Copy, /* Copy trait is necessary to ensure that T is a primitive type and can used multiple times without a move is necessary */
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

struct GraphOptimization<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GraphOptimization<'a> {
    // Constants:
    const c: f32 = 0.5; // damping constant
    const dt: f32 = 0.1; // time step
    const ITERATIONS: usize = 500; // maximum number of ITERATIONS
    const threshold: f32 = 0.01; // minimum displacement

    fn init(graph: &'a mut Graph, start: &'a Node) -> Vec<NodePos> {
        let mut positions: Vec<NodePos> = Vec::new();

        // Closure is used to return initial coordinates for each node by a specific logic so the algorithm doesn't get stuck.
        let init_logic = |i: usize| -> (f32, f32) {
            match i % 4 {
                0 => (i as f32, 0.0),    /* 1st quadrant */
                1 => (0.0, i as f32),    /* 2nd quadrant */
                2 => (-(i as f32), 0.0), /* 3rd quadrant */
                _ => (0.0, -(i as f32)), // case: 3 (_ must be used!) /* 4th quadrant */
            }
        };

        // Iterates through
        for (i, e) in graph.nodes.iter().enumerate() {
            if node != start {
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

    pub fn run(mut graph: Graph<'a>, start: &'a Node) -> Graph<'a> {
        let mut positions: Vec<NodePos> = Self.init(&mut graph, start);

        // Returns normalized vector (tuple of size 2).
        let norm = |x: (f32, f32)| -> (f32, f32) {
            let a = x.0;
            let b = x.1;
            let value = f32::sqrt(a.powi(2) + b.powi(2));

            (value * a, value * b)
        };

        // Returns amound of vector (tuple of size 2).
        let amount = |x: (f32, f32)| -> f32 {
            let a = x.0;
            let b = x.1;

            f32::sqrt(a.powi(2) + b.powi(2))
        };

        // Spring constant (= Federkonstante) depends on graphs value (variation might be necessary!)
        let k = f32::sqrt((40 / graph.node_len) as f32);

        // Attraction constant
        let a: f32 = 1.0;

        for i in 0..=Self::ITERATIONS {
            let mut totalDisplacement = 0;

            for i in 0..=graph.node_len() - 1
            /* TODO: Be careful! This condition forces the graph to have at least one node! */
            {
                // Location vector (previous sorting in init function causes that
                let mut v = &positions[i].pos;
                let mut dv = &positions[i].vel;
                let u = &positions[i + 1].pos;

                // Calculate repulsion force:
                // Distance vector:
                let dist = v.subtract_sub(u); // operator overloading: subtract two tuples of size 2!
                let dist_amount = amount(dist);

                // TODO: Repulsion force shall affected by the weight of the connected edge between v and u. Higher weight should result in larger distance between the nodes while lower weight should cause smaller distance.

                let repulse_force = -k / dist_amount.powi(2);
                let attract_force = a * dist_amount;
                let result_force = repulse_force + attract_force;

                //let repulse_force = dist.multiply_scalar(-k); // operator overloading: multiplies scalar with tuple of size 2!
                dv = *(dv.0 + result_force, dv.1 + result_force);

                let v_old = v.clone();

                // Update nodes position depending on current acceleration:
                v = v + dv.multiply_scalar(dt);

                // Calculate difference in old and new position:
                let diff_amount = amount(v - v_old);

                totalDisplacement += diff_amount;

                if totalDisplacement < Self::threshold {
                    break;
                }
            }
        }
        graph
    }
}
