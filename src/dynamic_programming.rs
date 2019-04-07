// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashSet;
use crate::opt::Opt;
use crate::opt::OptCombination;

pub struct DynamicProgramming<'a> {
    opts: &'a Vec<Opt>,
    cache: Vec<Option<OptCombination<'a>>>,
}

impl<'a> DynamicProgramming<'a> {
    pub fn with_opts(opts: &'a Vec<Opt>) -> Self {
        DynamicProgramming{opts, cache: vec![]}
    }

    pub fn solve_recursively(&mut self, n: u32) -> OptCombination<'a> {
        self.cache.clear();
        self.cache.resize((n + 1) as usize, None);
        self.recurse(n)
    }

    fn recurse(&mut self, n: u32) -> OptCombination<'a> {
        if let Some(h) = &self.cache[n as usize] {
            return h.clone();
        }

        let mut solutions = HashSet::new();

        for i in 1..=n {
            for opt in self.opts.iter().filter(|opt| opt.accepts(i)) {
                let rem = n - i;
                let mut combo_rem = self.recurse(rem);

                solutions.insert({combo_rem.add(opt, i); combo_rem});
            }
        }

        let best = solutions
            .into_iter()
            .max_by(|a,b| a.value().partial_cmp(&b.value()).expect("Not NaN"))
            .unwrap_or(OptCombination::new());

        self.cache[n as usize] = Some(best.clone());
        best
    }
}

// optimized:
// 1. Find best solution while iterating, don't store everything.
// 2. Don't recalculate value.
// 3. Create best combination only at the end.
// 4. Return only f64 (for recursive).
// 5. Early exit if we can tell that no further options will apply
//    (due to options being sorted by start of range).
pub struct DynamicProgrammingOptimized<'a> {
    opts: Vec<&'a Opt>,
    cache: Vec<Option<(OptCombination<'a>, f64)>>,
}

impl<'a> DynamicProgrammingOptimized<'a> {
    pub fn with_opts(opts: &'a Vec<Opt>) -> Self {
        let mut opts_sorted: Vec<_> = opts.iter().collect();
        opts_sorted.sort_unstable_by_key(|opt| opt.start);
        DynamicProgrammingOptimized{opts: opts_sorted, cache: vec![]}
    }

    pub fn solve_recursively(&mut self, n: u32) -> OptCombination<'a> {
        self.cache.clear();
        self.cache.resize((n + 1) as usize, None);
        self.recurse(n, &self.opts.clone());
        self.cache[n as usize].as_ref().unwrap().0.clone()
    }

    // Recurse takes opts as an argument to avoid borrow-checker issues. In
    // particular, we need to do this to be able to iterate over the opts
    // while recursing inside the iterations.
    fn recurse(&mut self, n: u32, opts: &Vec<&'a Opt>) -> f64 {
        if let Some(h) = &self.cache[n as usize] {
            return h.1;
        }

        let mut best_value = 0.0;
        let mut best_combo_info = None;

        for i in opts[0].start..=n {
            for (oi,opt) in opts.iter()
                                .enumerate()
                                .take_while(|(_,opt)| i >= opt.start)
                                .filter(|(_,opt)| opt.accepts(i)) {
                let rem = n - i;
                let value_rem = self.recurse(rem, opts);
                let value = value_rem + opt.gain_for(i);

                if value > best_value {
                    best_combo_info = Some((i, oi));
                    best_value = value;
                }
            }
        }

        let best_combo =
            if let Some((i, oi)) = best_combo_info {
                let mut b = self.cache[(n - i) as usize].as_ref().unwrap().0.clone();
                // We use add_unordered, since the algorithm doesn't use
                // OptCombination (partial) equality or hash at all.
                b.add_unordered(opts[oi as usize], i);
                b
            } else {
                OptCombination::new()
            };

        self.cache[n as usize] = Some((best_combo, best_value));

        best_value
    }

    pub fn solve_iter(&mut self, n: u32) -> OptCombination<'a> {
        self.cache.clear();
        self.cache.resize((n + 1) as usize, Some((OptCombination::new(), 0.0)));
        for i in 0..=n {
            self.cache[i as usize] = Some(self.iter(i));
        }
        self.cache[n as usize].as_ref().unwrap().0.clone()
    }

    fn iter(&self, n: u32) -> (OptCombination<'a>,f64) {
        let mut best_value = 0.0;
        let mut best_combo_info = None;

        for i in self.opts[0].start..=n {
            for (oi,opt) in self.opts.iter()
                                     .enumerate()
                                     .take_while(|(_,opt)| i >= opt.start)
                                     .filter(|(_,opt)| opt.accepts(i)) {
                let rem = n - i;
                let value_rem = self.cache[rem as usize].as_ref().unwrap().1;
                let value = value_rem + opt.gain_for(i);

                if value > best_value {
                    best_combo_info = Some((i, oi));
                    best_value = value;
                }
            }
        }

        let best_combo =
            if let Some((i, oi)) = best_combo_info {
                let mut b = self.cache[(n - i) as usize].as_ref().unwrap().0.clone();
                // We use add_unordered, since the algorithm doesn't use
                // OptCombination (partial) equality or hash at all.
                b.add_unordered(&self.opts[oi as usize], i);
                b
            } else {
                OptCombination::new()
            };

        (best_combo, best_value)
    }
}
