/*!
Copyright 2024 Justin Kirk

Licensed under the TIG Inbound Game License v1.0 or (at your option) any later
version (the "License"); you may not use this file except in compliance with the
License. You may obtain a copy of the License at

https://github.com/tig-foundation/tig-monorepo/tree/main/docs/licenses

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the specific
language governing permissions and limitations under the License.
*/

// TIG's UI uses the pattern tig_challenges::<challenge_name> to automatically detect your algorithm's challenge

use std::collections::HashSet;
use tig_challenges::vehicle_routing::{Challenge, Solution};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng; // Added the necessary import for Rng

pub fn solve_challenge(challenge: &Challenge) -> Result<Option<Solution>, String> {
    let mut rng = StdRng::seed_from_u64(challenge.seed as u64);
    let mut best_solution = None;
    let mut best_distance = f64::INFINITY;

    // Start with a greedy solution
    let mut solution = generate_greedy_solution(&challenge);
    best_solution = Some(solution.clone());
    best_distance = solution.total_distance(&challenge);

    // Perform local search on the greedy solution
    solution = local_search(&mut rng, solution, &challenge);
    let distance = solution.total_distance(&challenge);
    if distance < best_distance {
        best_distance = distance;
        best_solution = Some(solution);
    }

    // Perform additional iterations of random restarts and local search
    for _ in 0..100000 {
        let mut solution = generate_random_solution(&mut rng, &challenge);
        let distance = solution.total_distance(&challenge);

        if distance < best_distance {
            best_distance = distance;
            best_solution = Some(solution.clone());
        }

        solution = local_search(&mut rng, solution, &challenge);
        let distance = solution.total_distance(&challenge);

        if distance < best_distance {
            best_distance = distance;
            best_solution = Some(solution);
        }
    }

    Ok(best_solution)
}

fn generate_greedy_solution(challenge: &Challenge) -> Solution {
    let mut solution = Solution::new(vec![0]);
    let mut remaining_nodes: Vec<_> = (1..challenge.distance_matrix.len()).collect();

    while !remaining_nodes.is_empty() {
        let mut min_distance = f64::INFINITY;
        let mut closest_node = None;

        for &node in &remaining_nodes {
            let distance = challenge.distance_matrix[*solution.routes.last().unwrap()][node];
            if distance < min_distance {
                min_distance = distance;
                closest_node = Some(node);
            }
        }

        if let Some(node) = closest_node {
            solution.routes.push(node);
            remaining_nodes.retain(|&n| n != node);
        } else {
            break;
        }
    }

    solution
}

fn generate_random_solution(rng: &mut StdRng, challenge: &Challenge) -> Solution {
    let mut nodes: Vec<_> = (0..challenge.distance_matrix.len()).collect();
    nodes.shuffle(rng);
    Solution::new(nodes)
}

fn local_search(rng: &mut StdRng, mut solution: Solution, challenge: &Challenge) -> Solution {
    let mut best_solution = solution.clone();
    let mut best_distance = best_solution.total_distance(challenge);

    for _ in 0..10000 {
        let mut neighbors = HashSet::new();
        for _ in 0..100 {
            let a = rng.gen_range(0..solution.routes.len());
            let b = rng.gen_range(0..solution.routes.len());
            if a != b {
                solution.routes.swap(a, b);
                neighbors.insert(solution.clone());
                solution.routes.swap(a, b);
            }
        }

        for neighbor in neighbors {
            let distance = neighbor.total_distance(challenge);
            if distance < best_distance {
                best_distance = distance;
                best_solution = neighbor;
            }
        }

        if best_distance < solution.total_distance(challenge) {
            solution = best_solution.clone();
        } else {
            break;
        }
    }

    best_solution
}

// Assuming these methods are implemented for Solution
impl Solution {
    pub fn new(routes: Vec<usize>) -> Self {
        Solution { routes }
    }

    pub fn total_distance(&self, challenge: &Challenge) -> f64 {
        // Calculate the total distance of the solution
        self.routes.windows(2).map(|w| challenge.distance_matrix[w[0]][w[1]]).sum()
    }

    pub fn clone(&self) -> Self {
        Solution { routes: self.routes.clone() }
    }
}
