// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashSet;
use crate::opt::Opt;
use crate::opt::OptCombination;

pub struct OptimalSubstructure<'a> {
    opts: &'a Vec<Opt>,
}

impl<'a> OptimalSubstructure<'a> {
    pub fn with_opts(opts: &'a Vec<Opt>) -> Self {
        OptimalSubstructure{opts}
    }

    pub fn solve(&self, n: u32) -> OptCombination<'a> {
        self.recurse(n)
    }

    fn recurse(&self, n: u32) -> OptCombination<'a> {
        let mut solutions = HashSet::new();

        for i in 1..=n {
            for opt in self.opts.iter().filter(|opt| opt.accepts(i)) {
                let rem = n - i;
                let mut combo_rem = self.recurse(rem);

                solutions.insert({combo_rem.add(opt, i); combo_rem});
            }
        }

        solutions
            .into_iter()
            .max_by(|a,b| a.value().partial_cmp(&b.value()).expect("Not NaN"))
            .unwrap_or(OptCombination::new())
    }
}
