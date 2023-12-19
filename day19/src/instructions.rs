use std::collections::HashMap;

use crate::{workflow::{Workflow, WorkflowResult}, attribs::{Attribs, AttribRange}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instructions<'a> {
    map: HashMap<&'a str, Workflow<'a>>
}

impl<'a> Instructions<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut map = HashMap::new();

        for line in input.lines() {
            let (name, workflow) = line.split_at(line.chars().position(|c| c == '{').unwrap());
            let workflow = Workflow::parse(workflow);
            map.insert(name, workflow);
        }

        Self { map }
    }

    pub fn get(&self, name: &str) -> &Workflow<'a> {
        self.map.get(name).unwrap()
    }

    pub fn run(&self, item: Attribs) -> bool {
        let mut workflow = self.get("in");

        loop {
            match workflow.run(item) {
                WorkflowResult::Accept => return true,
                WorkflowResult::Reject => return false,
                WorkflowResult::Workflow(name) => workflow = self.get(name),
            }
        }
    }

    fn run_range_starting_at(&self, range: AttribRange, start: &'a str) -> usize {
        let mut works = 0;
        for (sub_range, result) in self.map[start].run_range(range) {
            match result {
                WorkflowResult::Accept => works += sub_range.len(),
                WorkflowResult::Reject => (),
                WorkflowResult::Workflow(new_start) => works += self.run_range_starting_at(sub_range, new_start),
            }
        }
        works
    }

    pub fn run_range(&self, range: AttribRange) -> usize {
        self.run_range_starting_at(range, "in")
    }
}
