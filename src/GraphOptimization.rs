use std::cmp::Ordering;
use std::process::Output;

// Own objects.
use crate::Graph::{Graph, Node};

// Standard library
use std::ops::Sub;

// Trait for subtracting tuples of size 2.
trait TupleSub<RHS> {
    type Output; /* defines output in a generic way after implementation in impl-block */
    fn subtract_sub(self, others: RHS) -> Self::Output;
}

// Represents a node in the algorithm.
pub struct NodePos {
    pub no: u32,
    pub pos: (f32, f32),
    pub vel: (f32, f32)
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
impl<T, RHS> TupleSub<RHS> for (T, T) // where block defines additional constraints on the types that can implement this trait TupleSub!
    where T: Sub<Output = T> /* T must implement the Sub-trait (means that minus operator could used on it) and the output type must also be T (so from same type) */,
        RHS: Into<(T, T)> /* RHS must implement the Into-trait which means that RHS can be converted into a tuple of size 2 containing values of type T */,
{
    type Output = (T, T);

    fn subtract_sub(self, others: RHS) -> Self::Output {
        let other = other.into();
        (self.0 - other.0, self.1 - other.1)
    }
}

// Implementation of PartialEq-trait (necessary for sorting)
impl PartialEq<Self> for NodePos {
    fn eq(&self, other: &Self) -> bool {
        self.no.eq(&other.no)
    }
}

// Implementation of PartialOrd-trait (necessary for sorting)
impl PartialOrd for NodePos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.no.partial_cmp(&other.no)
    }
}

struct GraphOptimization<'a> {
    marker: std::marker::PhantomData<&'a()>
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
                0 => (i as f32, 0.0), /* 1st quadrant */
                1 => (0.0, i as f32), /* 2nd quadrant */
                2 => (-(i as f32), 0.0), /* 3rd quadrant */
                _ => (0.0, -(i as f32)), // case: 3 (_ must be used!) /* 4th quadrant */
            }
        };

        // Iterates through
        for (i, e) in graph.nodes.iter().enumerate() {
            if node != start {
                // Start node shall be in center of graphical representation.
                positions.push(NodePos::new(start.no(), 0.0, 0.0, 0.0, 0.0));
            }
            else {
                // Other nodes are initialized around start node with different (not random) coordinates.
                let (x, y) = init_logic(i);

                positions.push(NodePos::new(e.no(), x, y, 0.0, 0.0));
            }
        }

        // Sort vector by NodePos.no to enable index-based access (so positions[x] -> node_x.
        positions.sort_by(|a, b| a.partial_cmp(b).unwrap());

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

        for i in 0..=Self::ITERATIONS {
            let mut totalDisplacement = 0;

            // Calculate repulsive (= abstoßungs) forces between nodes.
            for edge in graph.edges {
                if edge.source() != edge.dest() {
                    // Location vector (= Ortsvektor).
                    let u = &positions[edge.source().no() as usize].pos;
                    let v = &positions[edge.dest().no() as usize].pos;

                    // Acceleration vectors.
                    let mut du = &positions[edge.source().no() as usize].vel;
                    let mut dv = &positions[edge.source().no() as usize].vel;

                    // Difference vector
                    let delta = u.subtract_sub(v);

                    // Normed distance to both points.
                    let dist = norm(delta);

                    if amount(dist) > 0.0 {
                        let force = k * k / dist * delta;

                        // TODO: Das geht so nicht! Operatorüberladung von gestern verwenden!
                        du = du - (force / edge.weight(), force / edge.weight());
                        dv = dv + (force / edge.weight(), force / edge.weight());
                    }
                }
            }
        }

        graph
    }
}