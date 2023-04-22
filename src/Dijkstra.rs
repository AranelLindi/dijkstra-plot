// Own objects.
use crate::Graph::Graph;
use crate::Graph::node::Node;
//use crate::Graph::edge::Edge;

// Standard library.
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;
use crate::Graph::graph_type::graph_enum::GraphType;
//use crate::Graph::graph_type::graph_enum::GraphType;//::Undirected;

// Traits are needed so standard functions can be performed.
#[derive(PartialEq, Eq, Clone)]
pub struct DijkstraHeapEle<'a> {
    pub owner: &'a Node<'a>,
    pub prev: Option<&'a Node<'a>>,
    c: u32,
}

// Trait implementation.
impl<'a> PartialOrd<Self> for DijkstraHeapEle<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.c.cmp(&other.c).reverse())
    }
}

// Trait implementation.
impl<'a> Ord for DijkstraHeapEle<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.c.cmp(&other.c).reverse()
    }
}

// Struct definition.
pub struct Dijkstra<'a> {
    marker: std::marker::PhantomData<&'a ()>, /* Necessary to convince compiler, lifetime parameter is necessary */
}

// Implementation.
impl<'a> Dijkstra<'a> {
    // Prepares graph to be ready for execution of Dijkstra algorithm by creating a min-heap with node_len-entries. Complexity: O(n)
    fn init(graph: &'a Graph<'a>, start: &'a Node<'a>) -> BinaryHeap<DijkstraHeapEle<'a>> {
        let mut costs: BinaryHeap<DijkstraHeapEle> = BinaryHeap::new();

        // Set distance to each node - except start node - to "infinity" and its predecessor to null (predecessor is determined in main function).
        for node in graph.nodes.iter() /* O(n) */ {
            if node.no() != start.no() {
                costs.push(DijkstraHeapEle {
                    owner: node.as_ref(),
                    prev: None,
                    c: u32::MAX,
                });
            } else {
                costs.push(DijkstraHeapEle {
                    owner: start,
                    prev: None,
                    c: 0, /* costs to start node are zero */
                });
            }
        }

        return costs;
    }

    pub fn run(graph: &'a Graph<'a>, start: &'a Node<'a>, dest: Option<&'a Node<'a>>) -> Vec<DijkstraHeapEle<'a>> {
        // Initialize graph before dijkstra starts:
        let mut Q = Self::init(graph, start);
        // Q is a min-heap in which the node with minimum costs to get is on top.

        // Creates set of nodes to which most favorable path already has been found.
        let mut N_ : HashSet<usize> = HashSet::new(); // graph.nodes.iter().collect();
        N_.insert(start.no()); // start node is trivial

        // Result of executed algorithm: Contains summarized costs and predecessor for each node.
        let mut result: Vec<DijkstraHeapEle> = Vec::new();

        // Copies edges of given graph and sorts them first by id of source node and second by weight.
        //let edges = graph.edges.clone();

        // Closure iterates through given BinaryHeap until specific node was found, updates its values and pushes each entry back into queue.
        let update_node =
            |node: &Node, predecessor: Option<&'a Node<'a>>, costs: u32, q: &mut BinaryHeap<DijkstraHeapEle<'a>>| /* WC: O(n*log(n)) */ {
                let mut _q: BinaryHeap<DijkstraHeapEle> = q.clone();
                let mut _a: BinaryHeap<DijkstraHeapEle> = BinaryHeap::new();

                for _ in 0..=_q.len() /* iterate through q without iterator() */
                {
                    if let Some(obj) = _q.pop() /* WC: O(log(n)) */ {
                        // If taken node is searched not its values are updated and pushed back...
                        if obj.owner == node {
                            _a.push(DijkstraHeapEle {
                                owner: obj.owner, /* owner remains same */
                                prev: predecessor, /* changed predecessor */
                                c: costs, /* changed costs to node */
                            }); // O(1)
                        } else {
                            // ... otherwise it is immediately pushed back unchanged.
                            _a.push(obj); // O(1)
                        }
                    }
                }

                // Return q but with changed values for node.
                return _a;
            };

        // Closure iterates through given BinaryHeap until specific node was found and returns current known costs to it.
        let get_cost = |node: &Node, q: &mut BinaryHeap<DijkstraHeapEle>| -> Option<u32> /* WC: O(N) */ {
            // iter() generates an iterator. Every following operation is performed on each element in the heap which is iterated.
            let i = q.iter().find(|&y| y.owner == node);

            // Might be that searched node is not part of q so result is packed into Option container to prevent errors.
            if let Some(obj) = i {
                Some(obj.c)
            } else {
                None
            }
        };

        /* Dijkstra algorithm:
            Iterate as long as Q isn't empty (which is the max. number of nodes), select in each iteration
            node with favorable costs and look for cheaper paths to its neighbours. If an such
            path was found update information otherwise is favorable paths found.
            After while loop cheapest paths to all nodes from start node are found.

            Note: Works with directed and undirected edges!
        */
        while !Q.is_empty() /* O(V) */ {
            if let Some(u) = Q.pop() /* WC: O(log(V)) */ {
                N_.insert(u.owner.no()); // O(1), WC: O(V)

                result.push(u.clone()); // O(1), WC: O(V)

                for edge in graph.edges.iter() /* WC: O(E) */ {
                    // Might be that there's a more cheaper path to dest node when taking edge backwards. If its not forbidden (because directed edge) then check both directions!
                    let origin_node_ref= {
                        if edge.source().no() == u.owner.no() /* If edge is not considered inverted, the consideration of the direction condition is superfluous */ {
                            Some(edge.dest())
                        } else if *edge.etype() == GraphType::Undirected && edge.dest().no() == u.owner.no() /* If edge can be inverted, check if its a valid way */ {
                            Some(edge.source())
                        } else {
                            None
                        }
                    };

                    if let Some(v) = origin_node_ref {
                        if !N_.contains(&v.no()) /* O(1), WC: O(V) */ {
                            let dist: u32 = edge.weight() + u.c;

                            if let Some(c) = get_cost(v.as_ref(), Q.borrow_mut()) /* WC: O(V) */ {
                                // Potential new path costs must be real smaller than current path costs:
                                if dist < c {
                                    Q = update_node(v.as_ref(), Option::from(u.owner), dist, Q.borrow_mut()); // WC: O(V*log(V)) // TODO: Must be designed even more favorable!
                                }
                            }
                        }
                    }
                }
            }
        }

        /* Cost complexity of this implementation of dijkstra algorithm: O(V*log(V)*(1+E)) (room for improvement) */
        // TODO:
        // - Cheaper implementation of update_node (probably best method is to use another common data structure which allows updating in O(n)-time)
        // - Maybe implement fibonacci heap data structure by myself (will be ver difficult, at least...) to extract min entry in O(1)-time

        return result;
    }
}