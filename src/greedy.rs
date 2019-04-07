// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::opt::Opt;
use crate::opt::OptCombination;
use std::cmp::min;

fn start_slope_for(opt: &Opt) -> f64 {
    (opt.start as f64 * opt.interest + opt.bonus) / opt.start as f64
}

pub struct Greedy<'a> {
    opts: &'a Vec<Opt>
}

impl<'a> Greedy<'a> {
    pub fn with_opts(opts: &'a Vec<Opt>) -> Self {
        Greedy{opts}
    }

    pub fn solve(&self, n: u32) -> OptCombination<'a> {
        let mut combo = OptCombination::new();
        let mut rem = n;

        while rem > 0 {
            // Find the best existing combination we can add to.
            let best_combo_mut = combo.iter_mut()
                .filter(|(o,v)| v < &o.end)
                .map(|ov| { let slope = ov.0.interest; (ov, slope) })
                .max_by_key(|(_, slope)| (10000.0 * slope) as u32);

            // Find the best option to add as new.
            let best_opt_start = self.opts.iter()
                .filter(|opt| rem >= opt.start)
                .map(|opt| (opt, start_slope_for(opt)))
                .max_by_key(|(_,slope)| (10000.0 * slope) as u32);

            let rem_slope = if let Some((_,s)) = best_combo_mut { s } else { 0.0 };
            let start_slope = if let Some((_,s)) = best_opt_start { s } else { 0.0 };

            // No valid choices, we are done.
            if rem_slope == 0.0 && start_slope == 0.0 { break; }

            // Choose adding to an existing vs creating new depending on their slope.
            if rem_slope > start_slope {
                let (o,v) = best_combo_mut.unwrap().0;
                let o_rem = o.end - *v;
                *v += min(o_rem, rem);
                rem -= min(o_rem, rem);
                println!("Adding to existing option {:?}", o);
            } else {
                let opt = best_opt_start.unwrap().0;
                combo.add(opt, opt.start);
                rem -= opt.start;
                println!("Adding new option {:?} {}", opt, rem);
            }
        }

        combo
    }
}
