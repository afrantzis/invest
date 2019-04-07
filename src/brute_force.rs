// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashSet;
use crate::opt::Opt;
use crate::opt::OptCombination;

pub struct BruteForce<'a> {
    opts: &'a Vec<Opt>
}

impl<'a> BruteForce<'a> {
    pub fn with_opts(opts: &'a Vec<Opt>) -> Self {
        BruteForce{opts}
    }

    pub fn solve(&self, n: u32) -> OptCombination<'a> {
        let solutions = self.recurse(n);
        solutions.into_iter()
                 .max_by(|a,b| a.value().partial_cmp(&b.value()).expect("Not NaN"))
                 .unwrap()
    }

    fn recurse(&self, n: u32) -> HashSet<OptCombination<'a>> {
        let mut solutions = HashSet::new();

        for i in 1..=n {
            for opt in self.opts.iter().filter(|opt| opt.accepts(i)) {
                let rem = n - i;
                let solutions_rem = self.recurse(rem);

                if solutions_rem.len() == 0 {
                    solutions.insert(OptCombination::new_with_opt(opt, i));
                } else {
                    solutions.extend(
                        solutions_rem.into_iter().map(|mut s| {s.add(opt, i); s}));
                }
            }
        }

        solutions
    }
}
