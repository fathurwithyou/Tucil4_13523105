// src/solver.rs
use std::collections::HashMap;

pub(crate) const INFINITY: i32 = i32::MAX / 2;
pub type Graph = Vec<Vec<i32>>;

pub(crate) struct TspSolver {
    n: usize,
    graph: Graph,
    memo: HashMap<(usize, usize), (i32, Option<usize>)>,
    start_node: usize,
}

impl TspSolver {
    pub(crate) fn new(graph: Graph, start_node: usize) -> Self {
        let n = graph.len();
        TspSolver {
            n,
            graph,
            memo: HashMap::new(),
            start_node,
        }
    }

    pub(crate) fn solve(&mut self) -> (i32, Vec<usize>) {
        if self.n == 0 {
            return (0, vec![]);
        }
        if self.n == 1 {
            return (0, vec![self.start_node, self.start_node]);
        }

        for i in 0..self.n {
            if i == self.start_node {
                continue;
            }
            if self.graph[self.start_node][i] != INFINITY {
                let mask = (1 << self.start_node) | (1 << i);
                self.memo
                    .insert((mask, i), (self.graph[self.start_node][i], Some(self.start_node)));
            }
        }

        for mask_size in 3..=self.n {
            // Iterate over masks. Ensure current_mask is treated as usize for count_ones.
            // The range 0..(1 << self.n) will produce usize if self.n is not excessively large
            // (which it won't be for TSP DP).
            for current_mask in 0usize..(1usize << self.n) {
                if current_mask.count_ones() as usize != mask_size { // Compare usize with usize
                    continue;
                }
                if (current_mask & (1 << self.start_node)) == 0 {
                    continue;
                }

                for current_end_node in 0..self.n {
                    if current_end_node != self.start_node
                        && (current_mask & (1 << current_end_node)) != 0
                    {
                        let prev_mask = current_mask ^ (1 << current_end_node);
                        let mut current_min_cost_to_end_node = INFINITY;
                        let mut best_prev_node_for_end_node = None;

                        for prev_node_candidate in 0..self.n {
                            if prev_node_candidate != self.start_node
                                && (prev_mask & (1 << prev_node_candidate)) != 0
                            {
                                if let Some((cost_to_prev, _)) =
                                    self.memo.get(&(prev_mask, prev_node_candidate))
                                {
                                    if *cost_to_prev != INFINITY
                                        && self.graph[prev_node_candidate][current_end_node]
                                            != INFINITY
                                    {
                                        let new_cost = *cost_to_prev
                                            + self.graph[prev_node_candidate][current_end_node];
                                        if new_cost < current_min_cost_to_end_node {
                                            current_min_cost_to_end_node = new_cost;
                                            best_prev_node_for_end_node =
                                                Some(prev_node_candidate);
                                        }
                                    }
                                }
                            }
                        }

                        if current_min_cost_to_end_node != INFINITY {
                            self.memo.insert(
                                (current_mask, current_end_node),
                                (current_min_cost_to_end_node, best_prev_node_for_end_node),
                            );
                        }
                    }
                }
            }
        }

        let all_visited_mask = (1 << self.n) - 1;
        let mut min_total_tour_cost = INFINITY;
        let mut last_node_before_returning_to_start = None;

        for k in 0..self.n {
            if k == self.start_node {
                continue;
            }
            if let Some((cost_to_k, _)) = self.memo.get(&(all_visited_mask, k)) {
                if *cost_to_k != INFINITY && self.graph[k][self.start_node] != INFINITY {
                    let tour_cost = *cost_to_k + self.graph[k][self.start_node];
                    if tour_cost < min_total_tour_cost {
                        min_total_tour_cost = tour_cost;
                        last_node_before_returning_to_start = Some(k);
                    }
                }
            }
        }

        if min_total_tour_cost == INFINITY {
            return (INFINITY, vec![]);
        }

        let mut path = Vec::with_capacity(self.n + 1);
        if let Some(mut current_node_in_reconstruction) = last_node_before_returning_to_start {
            let mut current_reconstruction_mask = all_visited_mask;
            let mut temp_path_segment = Vec::new();

            while current_node_in_reconstruction != self.start_node {
                 if temp_path_segment.len() >= self.n { 
                    return (INFINITY, vec![]); 
                }
                temp_path_segment.push(current_node_in_reconstruction);
                match self.memo.get(&(
                    current_reconstruction_mask,
                    current_node_in_reconstruction,
                )) {
                    Some((_, Some(prev_node))) => {
                        let actual_prev_node = *prev_node;
                        current_reconstruction_mask ^= 1 << current_node_in_reconstruction; // Removed parentheses
                        current_node_in_reconstruction = actual_prev_node;
                    }
                    Some((_, None)) => {
                        if current_reconstruction_mask == ((1 << self.start_node) | (1 << current_node_in_reconstruction)) {
                             break;
                        }
                        return (INFINITY, vec![]); 
                    }
                    None => {
                        return (INFINITY, vec![]); 
                    }
                }
            }
            path.push(self.start_node);
            path.extend(temp_path_segment.into_iter().rev());
            path.push(self.start_node);
        } else if self.n > 0 { 
             return (INFINITY, vec![]); 
        }


        if self.n > 1 && (path.len() != self.n + 1 || path.first() != Some(&self.start_node) || path.last() != Some(&self.start_node)) {
             if min_total_tour_cost != INFINITY {
                 return (min_total_tour_cost, vec![]); 
             }
        } else if self.n == 1 && path.is_empty() { 
            path = vec![self.start_node, self.start_node];
        }

        (min_total_tour_cost, path)
    }
}