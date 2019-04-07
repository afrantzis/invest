// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;

fn test_options() -> Vec<Opt> {
    let buf = BufReader::new(
        "o1 1 2 0.011 0.01
         o2 3 4 0.016 0.022
         o3 5 9 0.018 0.025
         o4 10 19 0.02 0.04".as_bytes());

    parse_opt_bufread(buf)
}

macro_rules! assert_float_eq {
    ($e1:expr, $e2:expr) => { 
        assert!(($e1 - $e2).abs() < std::f64::EPSILON);
    }
}

macro_rules! assert_solution_eq {
    ($e1:expr, $e2:expr) => { 
        let mut s1: Vec<_> = $e1.iter().map(|x| (x.0.name.clone(), x.1)).collect();
        s1.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut s2: Vec<_> = $e2.iter().map(|x| (x.0.to_string(), x.1)).collect();
        s2.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        
        assert_eq!(s1, s2);
    }
}

#[test]
fn test_brute_force() {
    let opts = test_options();

    let solution = BruteForce::with_opts(&opts).solve(17);
    assert_float_eq!(solution.value(), 0.401);
    assert_solution_eq!(solution, &[("o1", 1), ("o2", 3), ("o2", 3), ("o4", 10)]);
}

#[test]
fn test_overlapping_subproblems() {
    let opts = test_options();

    let solution = OverlappingSubproblems::with_opts(&opts).solve(17);
    assert_float_eq!(solution.value(), 0.401);
    assert_solution_eq!(solution, &[("o1", 1), ("o2", 3), ("o2", 3), ("o4", 10)]);
}

#[test]
fn test_optimal_substructure() {
    let opts = test_options();

    let solution = OptimalSubstructure::with_opts(&opts).solve(17);
    assert_float_eq!(solution.value(), 0.401);
    assert_solution_eq!(solution, &[("o1", 1), ("o2", 3), ("o2", 3), ("o4", 10)]);
}

#[test]
fn test_dp_recursive() {
    let opts = test_options();

    let solution = DynamicProgrammingOptimized::with_opts(&opts).solve_recursively(17);
    assert_float_eq!(solution.value(), 0.401);
    assert_solution_eq!(solution, &[("o1", 1), ("o2", 3), ("o2", 3), ("o4", 10)]);
}

#[test]
fn test_dp_recursive_optimized() {
    let opts = test_options();

    let solution = DynamicProgrammingOptimized::with_opts(&opts).solve_recursively(17);
    assert_float_eq!(solution.value(), 0.401);
    assert_solution_eq!(solution, &[("o1", 1), ("o2", 3), ("o2", 3), ("o4", 10)]);
}

#[test]
fn test_dp_iterative_optimized() {
    let opts = test_options();

    let solution = DynamicProgrammingOptimized::with_opts(&opts).solve_iter(17);
    assert_float_eq!(solution.value(), 0.401);
    assert_solution_eq!(solution, &[("o1", 1), ("o2", 3), ("o2", 3), ("o4", 10)]);
}

#[test]
fn test_greedy() {
    let opts = test_options();

    let solution = Greedy::with_opts(&opts).solve(17);
    assert_float_eq!(solution.value(), 0.401);
    assert_solution_eq!(solution, &[("o1", 1), ("o2", 3), ("o2", 3), ("o4", 10)]);
}
