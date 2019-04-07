// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashSet;
use crate::opt::Opt;
use crate::opt::OptCombination;

pub struct OverlappingSubproblems<'a> {
    opts: &'a Vec<Opt>,
    cache: Vec<Option<HashSet<OptCombination<'a>>>>
}

impl<'a> OverlappingSubproblems<'a> {
    pub fn with_opts(opts: &'a Vec<Opt>) -> Self {
        OverlappingSubproblems{opts, cache: vec![]}
    }

    pub fn solve(&mut self, n: u32) -> OptCombination<'a> {
        self.cache.clear();
        self.cache.resize((n + 1) as usize, None);
        let solutions = self.recurse(n);
        solutions.into_iter()
                 .max_by(|a,b| a.value().partial_cmp(&b.value()).expect("Not NaN"))
                 .unwrap()
    }

    fn recurse(&mut self, n: u32) -> HashSet<OptCombination<'a>> {
        if let Some(h) = &self.cache[n as usize] {
            return h.clone();
        }

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

        self.cache[n as usize] = Some(solutions.clone());
        solutions
    }
}


