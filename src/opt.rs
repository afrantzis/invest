// Copyright © 2019 Alexandros Frantzis
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug,Clone)]
pub struct Opt {
    pub name: String,
    pub start: u32,
    pub end: u32,
    pub interest: f64,
    pub bonus: f64,
}

impl Hash for Opt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Opt {
    fn eq(&self, other: &Opt) -> bool {
        self.name == other.name
    }
}

impl Eq for Opt {}

impl Opt {
    pub fn gain_for(&self, amount: u32) -> f64 {
        amount as f64 * self.interest + self.bonus
    }

    pub fn accepts(&self, amount: u32) -> bool {
        amount >= self.start && amount <= self.end
    }
}

impl FromStr for Opt {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems: Vec<&str> = s.split(' ').collect();

        let name = elems[0].to_string();
        let start = elems[1].parse::<u32>().unwrap();
        let end = elems[2].parse::<u32>().unwrap();
        let interest = elems[3].parse::<f64>().unwrap();
        let bonus = elems[4].parse::<f64>().unwrap();

        Ok(Opt{name, start, end, interest, bonus})
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct OptCombination<'a>(Vec<(&'a Opt, u32)>);

impl<'a> OptCombination<'a> {
    pub fn new() -> Self {
        OptCombination(vec![])
    }

    pub fn new_with_opt(opt: &'a Opt, amount: u32) -> Self {
        OptCombination(vec![(opt, amount)])
    }

    pub fn add(&mut self, opt: &'a Opt, amount: u32) {
        // Insert sorted, so that PartialEq/Hash produce the right result regardless
        // of add order.
        let elem = (opt, amount);
        let pos = self.0.iter().position(
            |e| elem.0.name.cmp(&e.0.name).then(elem.1.cmp(&e.1)) == Ordering::Less)
            .unwrap_or(self.0.len());
        self.0.insert(pos, elem);
    }

    // Add unsorted, which is faster, but leads to OptCombinations with
    // the same elements in different order to compare unequal. Use with
    // caution.
    pub fn add_unordered(&mut self, opt: &'a Opt, amount: u32) {
        self.0.push((opt, amount));
    }

    pub fn value(&self) -> f64 {
        self.0.iter().map(|(opt,v)| opt.gain_for(*v)).sum::<f64>()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(&'a Opt, u32)>  {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (&'a Opt, u32)>  {
        self.0.iter_mut()
    }
}
