use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Adapted from https://doc.rust-lang.org/std/collections/binary_heap/

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
#[derive(Debug, Eq, PartialEq)]
pub struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn dijkstra(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}


fn invert_adjecency_list(adj_list: &Vec<Vec<Edge>>) -> Vec<Vec<Edge>> {
    let len = adj_list.len();
    let mut inverted: Vec<Vec<Edge>> = Vec::new();

    for _ in 0..len {
        inverted.push(Vec::new());
    }

    for (i, l) in adj_list.iter().enumerate() {
        for Edge { node, cost } in l {
            // exists: i -> node with cost
            // create: node -> i with cost
            inverted[*node].push(Edge { node: i, cost: *cost });

        }
    }

    inverted
}


pub fn dijkstra_bidir(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    println!("-------------------");
    println!("Goal: {}, Start: {}", goal, start);
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist_f: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut dist_b: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut prio_f = BinaryHeap::new();
    let mut prio_b = BinaryHeap::new();

    let adj_list_inv = &invert_adjecency_list(adj_list);

    // We're at `start`, with a zero cost
    dist_f[start] = 0;
    dist_b[goal]  = 0;

    prio_f.push(State { cost: 0, position: start });
    prio_b.push(State { cost: 0, position: goal });

    fn check_stop(dist_f: &Vec<usize>, dist_b: &Vec<usize>,
                  prio_f: &BinaryHeap<State>,
                  prio_b: &BinaryHeap<State>)
        -> Option<usize> {

        println!("check for stopping condition");
        if let Some(State { position, cost }) = prio_f.peek() {
            let position_f = position;
            let cost_f = cost;
            if let Some(State { position, cost }) = prio_b.peek() {
                let cost_b = cost;
                for i in 0..dist_f.len() {
                    if dist_f[i] == usize::MAX || dist_b[i] == usize::MAX {
                        continue;
                    }
                    if dist_f[i] + dist_b[i] <= cost_f + cost_b {
                        println!("At positions {} and {} with cost {} and {}", position_f, position, cost_f, cost_b);
                        println!("Compared to {} and {}", dist_f[i], dist_b[i]);
                        return Some(dist_f[i] + dist_b[i]);
                    }
                }
            }
        }
        None
    }

    while !prio_b.is_empty() || !prio_b.is_empty() {

        println!("forward step");
        if let Some(res) = dijkstra_step(adj_list, goal, &mut prio_f, &mut dist_f) {
            return Some(res);
        }
        if let Some(res) = check_stop(&dist_f, &dist_b, &prio_f, &prio_b) {
            return Some(res);
        }

        println!("backward step");
        if let Some(res) = dijkstra_step(adj_list_inv, start, &mut prio_b, &mut dist_b) {
            return Some(res);
        }
        if let Some(res) = check_stop(&dist_f, &dist_b, &prio_f, &prio_b) {
            return Some(res);
        }

    }
    None
}


fn dijkstra_step(adj_list: &Vec<Vec<Edge>>,
                 goal: usize,
                 heap: &mut BinaryHeap<State>,
                 dist: &mut Vec<usize>) 
    -> Option<usize> {

    // Examine the frontier with lower cost nodes first (min-heap)
    if let Some(State { cost, position }) = heap.pop() {
        println!("I: {}, P: {}", position, cost);
        // We found a shortest path!
        if position == goal {
            println!("Found {} == {} at {}", position, goal, cost);
            return Some(cost);
        }
        // Stopping criteria: a vertex is done from both sides
        // needs to be checked for in 'parent' function

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let cost = cost + edge.cost;

            // If so, add it to the frontier and continue
            if cost < dist[edge.node] {
                println!("{} < {} !", cost, dist[edge.node]);
                heap.push(State { position: edge.node, cost });
                // Relaxation, we have now found a better way
                dist[edge.node] = cost;
            }
        }
    }

    // Goal not reached in this step
    None
}


// pub fn astar_shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//     unimplemented!()
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    // Chosen for its efficiency.
    let graph = vec![
        // Node 0
        vec![Edge { node: 2, cost: 10 },
             Edge { node: 1, cost: 1 }],
        // Node 1
        vec![Edge { node: 3, cost: 2 }],
        // Node 2
        vec![Edge { node: 1, cost: 1 },
             Edge { node: 3, cost: 3 },
             Edge { node: 4, cost: 1 }],
        // Node 3
        vec![Edge { node: 0, cost: 7 },
             Edge { node: 4, cost: 2 }],
        // Node 4
        vec![]];

    assert_eq!(dijkstra(&graph, 0, 1), Some(1));
    assert_eq!(dijkstra(&graph, 0, 3), Some(3));
    assert_eq!(dijkstra(&graph, 3, 0), Some(7));
    assert_eq!(dijkstra(&graph, 0, 4), Some(5));
    assert_eq!(dijkstra(&graph, 4, 0), None);
    }

    #[test]
    fn test_dijkstra_bidir() {
    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    // Chosen for its efficiency.
    let graph = vec![
        // Node 0
        vec![Edge { node: 2, cost: 10 },
             Edge { node: 1, cost: 1 }],
        // Node 1
        vec![Edge { node: 3, cost: 2 }],
        // Node 2
        vec![Edge { node: 1, cost: 1 },
             Edge { node: 3, cost: 3 },
             Edge { node: 4, cost: 1 }],
        // Node 3
        vec![Edge { node: 0, cost: 7 },
             Edge { node: 4, cost: 2 }],
        // Node 4
        vec![]];

    assert_eq!(dijkstra_bidir(&graph, 0, 1), Some(1));
    assert_eq!(dijkstra_bidir(&graph, 0, 3), Some(3));
    assert_eq!(dijkstra_bidir(&graph, 3, 0), Some(7));
    assert_eq!(dijkstra_bidir(&graph, 0, 4), Some(5));
    assert_eq!(dijkstra_bidir(&graph, 4, 0), None);
    }

    // #[test]
    // fn test_astar_dijkstra() {
    // // This is the directed graph we're going to use.
    // // The node numbers correspond to the different states,
    // // and the edge weights symbolize the cost of moving
    // // from one node to another.
    // // Note that the edges are one-way.
    // //
    // //                  7
    // //          +-----------------+
    // //          |                 |
    // //          v   1        2    |  2
    // //          0 -----> 1 -----> 3 ---> 4
    // //          |        ^        ^      ^
    // //          |        | 1      |      |
    // //          |        |        | 3    | 1
    // //          +------> 2 -------+      |
    // //           10      |               |
    // //                   +---------------+
    // //
    // // The graph is represented as an adjacency list where each index,
    // // corresponding to a node value, has a list of outgoing edges.
    // // Chosen for its efficiency.
    // let graph = vec![
    //     // Node 0
    //     vec![Edge { node: 2, cost: 10 },
    //          Edge { node: 1, cost: 1 }],
    //     // Node 1
    //     vec![Edge { node: 3, cost: 2 }],
    //     // Node 2
    //     vec![Edge { node: 1, cost: 1 },
    //          Edge { node: 3, cost: 3 },
    //          Edge { node: 4, cost: 1 }],
    //     // Node 3
    //     vec![Edge { node: 0, cost: 7 },
    //          Edge { node: 4, cost: 2 }],
    //     // Node 4
    //     vec![]];

    // assert_eq!(astar_shortest_path(&graph, 0, 1), Some(1));
    // assert_eq!(astar_shortest_path(&graph, 0, 3), Some(3));
    // assert_eq!(astar_shortest_path(&graph, 3, 0), Some(7));
    // assert_eq!(astar_shortest_path(&graph, 0, 4), Some(5));
    // assert_eq!(astar_shortest_path(&graph, 4, 0), None);
    // }

    #[test]
    fn test_adjency_list_inversion() {
        let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: 10 },
                 Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![Edge { node: 1, cost: 1 },
                 Edge { node: 3, cost: 3 },
                 Edge { node: 4, cost: 1 }],
            // Node 3
            vec![Edge { node: 0, cost: 7 },
                 Edge { node: 4, cost: 2 }],
            // Node 4
            vec![]];
        let res = vec![
            // Node 0
            vec![Edge { node: 3, cost: 7 }],
            // Node 1
            vec![Edge { node: 0, cost: 1 },
                 Edge { node: 2, cost: 1 }],
            // Node 2
            vec![Edge { node: 0, cost: 10 }],
            // Node 3
            vec![Edge { node: 1, cost: 2 },
                 Edge { node: 2, cost: 3 }],
            // Node 4
            vec![Edge { node: 2, cost: 1 },
                 Edge { node: 3, cost: 2 }]
        ];
        assert_eq!(invert_adjecency_list(&graph), res);
    }
}
