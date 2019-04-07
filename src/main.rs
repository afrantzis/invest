// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use structopt::StructOpt;
use std::str::FromStr;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader,BufRead};

mod opt;
mod brute_force;
mod overlapping_subproblems;
mod optimal_substructure;
mod dynamic_programming;
mod greedy;

use opt::{Opt,OptCombination};
use brute_force::BruteForce;
use overlapping_subproblems::OverlappingSubproblems;
use optimal_substructure::OptimalSubstructure;
use dynamic_programming::DynamicProgramming;
use dynamic_programming::DynamicProgrammingOptimized;
use greedy::Greedy;

enum Method {
    BruteForce,
    OverlappingSubproblems,
    OptimalSubstructure,
    DPRecursive,
    DPRecursiveOptimized,
    DPIterativeOptimized,
    Greedy,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "brute-force" => Ok(Method::BruteForce),
            "overlapping-subproblems" => Ok(Method::OverlappingSubproblems),
            "optimal-substructure" => Ok(Method::OptimalSubstructure),
            "dp-recursive" => Ok(Method::DPRecursive),
            "dp-recursive-optimized" => Ok(Method::DPRecursiveOptimized),
            "dp-iterative-optimized" => Ok(Method::DPIterativeOptimized),
            "greedy" => Ok(Method::Greedy),
            _ => Err("Unknown method: ".to_owned() + s),
        }
    }
}

impl Method {
    fn variants() -> &'static [&'static str] {
        &["brute-force", "overlapping-subproblems", "optimal-substructure",
          "dp-recursive", "dp-recursive-optimized", "dp-iterative-optimized",
          "greedy"]
    }

}

/// Find optimal solutions to investment problems involving options with
/// profits of the form: profit(n) = interest * n + bonus, n in [min, max]
#[derive(StructOpt)]
struct Args {
    /// The method to use to solve the problem
    #[structopt(short = "m", long = "method",
      raw(possible_values="&Method::variants()"),
      default_value="dp-iterative-optimized")]
    method: Method,
    /// The amount to solve the problem for
    #[structopt(short = "n", long = "amount")]
    amount: u32,
    /// A file containing the problem options. Each line is an option
    /// of the form: "name min max interest bonus"
    #[structopt(short = "o", long = "opt-file")]
    optfile: PathBuf,
    /// The algorithm scale to apply to the options.
    #[structopt(short = "s", long = "scale")]
    scale: Option<u32>,
    /// A factor to multiply the options' interests by
    #[structopt(short = "i", long = "interest-factor", default_value="1.0")]
    interest_factor: f64
}

fn parse_opt_bufread(buf: impl BufRead) -> Vec<Opt> {
    buf.lines()
       .map(|s| s.unwrap().trim().to_string())
       .filter(|s| s.len() > 0 && !s.starts_with('#'))
       .map(|s| s.parse().unwrap())
       .collect()
}

fn parse_opt_file(path: &PathBuf) -> Vec<Opt> {
    let file = File::open(path).unwrap();
    let buffered = BufReader::new(file);

    parse_opt_bufread(buffered)
}

fn main() {
    let args = Args::from_args();

    let mut opts_orig = parse_opt_file(&args.optfile);
    let scale = args.scale.unwrap_or(1);
    let amount = args.amount / scale;

    for opt in &mut opts_orig {
        opt.interest *= args.interest_factor;
    }

    let mut opts = opts_orig.clone();

    if scale != 1 {
        for opt in &mut opts {
            opt.start = opt.start / scale + 
                if opt.start > 0 && opt.start / scale == (opt.start - 1) / scale {
                    1
                } else {
                    0
                };
            opt.end = opt.end / scale;
            opt.bonus = opt.bonus / (scale as f64);
        }
    }

    let solution =
        match args.method {
            Method::BruteForce =>
                BruteForce::with_opts(&opts).solve(amount),
            Method::OverlappingSubproblems =>
                OverlappingSubproblems::with_opts(&opts).solve(amount),
            Method::OptimalSubstructure =>
                OptimalSubstructure::with_opts(&opts).solve(amount),
            Method::DPRecursive =>
                DynamicProgramming::with_opts(&opts).solve_recursively(amount),
            Method::DPRecursiveOptimized =>
                DynamicProgrammingOptimized::with_opts(&opts).solve_recursively(amount),
            Method::DPIterativeOptimized =>
                DynamicProgrammingOptimized::with_opts(&opts).solve_iter(amount),
            Method::Greedy =>
                Greedy::with_opts(&opts).solve(amount),
        };

    let solution = 
        if scale == 1 {
            solution
        } else {
            let mut unscaled_solution = OptCombination::new();
            for (opt,opt_amount) in solution.iter() {
                let opt_orig = opts_orig.iter().find(|x| x.name == opt.name).unwrap();
                unscaled_solution.add(opt_orig, scale * opt_amount);
            }
            unscaled_solution
        };

    for (opt,opt_amount) in solution.iter() {
        println!("{}: {}", opt.name, opt_amount);
    }
    println!("Value: {}", solution.value());
}

#[cfg(test)]
mod test;
