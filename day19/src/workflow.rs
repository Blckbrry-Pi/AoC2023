use crate::attribs::{Attrib, Attribs, AttribRange, AttribSplit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkflowResult<'a> {
    Accept,
    Reject,
    Workflow(&'a str)
}

impl<'a> WorkflowResult<'a> {
    pub fn parse(line: &'a str) -> Self {
        match line {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(line),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkflowStep<'a> {
    Attrib {
        attrib: Attrib,
        greater: bool,
        value: usize,
        result: WorkflowResult<'a>
    },
    Result {
        result: WorkflowResult<'a>,
    },
}

impl<'a> WorkflowStep<'a> {
    pub fn parse(line: &'a str) -> Self {
        if let Some((attrib_str, result)) = line.split_once(':') {
            let attrib = Attrib::parse(attrib_str.chars().next().unwrap());
            let greater = attrib_str.chars().nth(1).unwrap() == '>';
            let value = attrib_str[2..].parse().unwrap();

            let result = WorkflowResult::parse(result);

            Self::Attrib { attrib, greater, value, result }
        } else {
            let result = WorkflowResult::parse(line);
            Self::Result { result }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow<'a> {
    steps: Vec<WorkflowStep<'a>>,
}

impl<'a> Workflow<'a> {
    pub fn parse(line: &'a str) -> Self {
        let inner = line.trim_matches(['{', '}']);

        let steps = inner.split(',').map(WorkflowStep::parse).collect();
        Self { steps }
    }

    pub fn run(&self, item: Attribs) -> WorkflowResult<'a> {
        for step in self.steps.iter().copied() {
            match step {
                WorkflowStep::Attrib { attrib, greater, value, result } => {
                    let applies = if greater {
                        item.access(attrib) > value
                    } else {
                        item.access(attrib) < value
                    };

                    if applies {
                        return result
                    }
                }
                WorkflowStep::Result { result } => return result,
            }
        }

        panic!("Should not reach this area")
    }

    pub fn run_range(&self, item: AttribRange) -> Vec<(AttribRange, WorkflowResult<'a>)> {
        let mut outputs = vec![];
        let mut remaining = vec![item];
        for step in self.steps.iter().copied() {
            match step {
                WorkflowStep::Attrib { attrib, greater, value, result } => {
                    let mut new_remaining = vec![];
                    for range in remaining {
                        let AttribSplit { applies, doesnt_apply } = if greater {
                            range.split_gt(attrib, value)
                        } else {
                            range.split_lt(attrib, value)
                        };
                        if let Some(applies_range) = applies {
                            outputs.push((applies_range, result));
                        }
                        if let Some(doesnt_apply_range) = doesnt_apply {
                            new_remaining.push(doesnt_apply_range);
                        }
                    }
                    remaining = new_remaining;
                },
                WorkflowStep::Result { result } => {
                    outputs.extend(remaining.drain(0..remaining.len()).map(|range| (range, result)));
                },
            }
        }
        outputs
    }
}
