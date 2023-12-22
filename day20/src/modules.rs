use std::{collections::HashSet, fmt::Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pulse { Hi, Lo }


#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Module<'a> {
    Button {
        name: &'a str,
        targets: Vec<&'a str>,
    },
    Broadcaster {
        name: &'a str,
        targets: Vec<&'a str>,
    },

    FlipFlop {
        name: &'a str,
        on: bool,
        
        state_counter: usize,
        state_mod: usize,

        targets: Vec<&'a str>,
        low_targets: Vec<&'a str>,
    },
    Conjunction {
        name: &'a str,
        remembered: Vec<(&'a str, Pulse)>,
        targets: Vec<&'a str>,
        low_targets: Vec<&'a str>,
    },
    Inverter {
        name: &'a str,
        targets: Vec<&'a str>,
        low_targets: Vec<&'a str>,
    }
}



impl<'a> Module<'a> {
    pub fn parse(line: &'a str) -> Self {
        let (name, targets) = line.split_once(" -> ").unwrap();
        let targets = targets.split(", ").collect();
        
        match name.chars().next().unwrap() {
            '%' => Self::FlipFlop { name: &name[1..], on: false, targets, low_targets: vec![], state_counter: 0, state_mod: 1 },
            '&' => Self::Conjunction { name: &name[1..], remembered: vec![], targets, low_targets: vec![] },
            _ => Self::Broadcaster { name, targets },
        }
    }

    pub fn setup_inputs(&mut self, inputs: &[&'a str]) {
        match self {
            | Self::Button { .. }
            | Self::Broadcaster { .. }
            | Self::FlipFlop { .. }
            | Self::Inverter { .. } => {},
            Self::Conjunction { name, remembered, targets, low_targets } => {
                if inputs.len() == 1 {
                    *self = Self::Inverter { name, targets: std::mem::take(targets), low_targets: std::mem::take(low_targets) };
                } else {
                    *remembered = inputs.iter().map(|name| (*name, Pulse::Lo)).collect();
                }
            },
        }
    }

    pub fn move_to_low(&mut self, low_modules: &HashSet<&str>) {
        match self {
            | Self::FlipFlop { targets, low_targets, .. }
            | Self::Conjunction { targets, low_targets, .. }
            | Self::Inverter { targets, low_targets, .. } => {
                let (additional_low, new_targets): (Vec<_>, _) = std::mem::take(targets)
                    .into_iter()
                    .partition(|target| low_modules.contains(target));

                low_targets.extend(additional_low);
                *targets = new_targets;
            },
            _ => {},
        }
    }

    pub fn input_pulse(&mut self, pulse: PulseInfo<'a>) -> (Vec<PulseInfo<'a>>, usize, usize) {
        match self {
            Self::Button { name, targets } | Self::Broadcaster { name, targets } => {
                let pulses = targets
                    .iter()
                    .copied()
                    .map(|target| PulseInfo { source: name, target, pulse: Pulse::Lo })
                    .collect();

                (pulses, 0, 0)
            },
            Module::FlipFlop {
                name, targets, low_targets,
                on, state_counter, state_mod,
            } => {
                let prev_state = *state_counter;
                
                if pulse.pulse == Pulse::Hi { return (vec![], 0, 0); }
                *state_counter += 1;
                *state_counter %= *state_mod;
                
                let pulses = prev_state ^ *state_counter;
                let lo_pulses = prev_state & pulses;
                let hi_pulses = *state_counter & pulses;
                if *state_counter != 0 {
                    return (vec![], lo_pulses, hi_pulses);
                }

                *on = !*on;

                let sent_pulse = if *on { Pulse::Hi } else { Pulse::Lo };
                let mapper = |target| PulseInfo { source: name, target, pulse: sent_pulse };
                if sent_pulse == Pulse::Lo {
                    let pulses = targets
                        .iter()
                        .copied()
                        .chain(low_targets.iter().copied())
                        .map(mapper)
                        .collect();
                    (pulses, lo_pulses, hi_pulses)
                } else {
                    let pulses = targets
                        .iter()
                        .copied()
                        .map(mapper)
                        .collect();
                    (pulses, lo_pulses, hi_pulses + low_targets.len())
                }
            }
            Module::Conjunction { name, targets, low_targets, remembered } => {
                remembered.iter_mut().find(|(source, _)| *source == pulse.source).unwrap().1 = pulse.pulse;

                let all_high = remembered.iter().all(|(_, pulse)| *pulse == Pulse::Hi);
                let sent_pulse = if all_high { Pulse::Lo } else { Pulse::Hi };

                let mapper = |target| PulseInfo { source: name, target, pulse: sent_pulse };
                if sent_pulse == Pulse::Lo {
                    let pulses = targets
                        .iter()
                        .copied()
                        .chain(low_targets.iter().copied())
                        .map(mapper)
                        .collect();
                    (pulses, 0, 0)
                } else {
                    let pulses = targets
                        .iter()
                        .copied()
                        .map(mapper)
                        .collect();
                    (pulses, 0, low_targets.len())
                }
            },
            Module::Inverter { name, targets, low_targets } => {
                let sent_pulse = if pulse.pulse == Pulse::Hi { Pulse::Lo } else { Pulse::Hi };

                let mapper = |target| PulseInfo { source: name, target, pulse: sent_pulse };
                if sent_pulse == Pulse::Lo {
                    let pulses = targets
                        .iter()
                        .copied()
                        .chain(low_targets.iter().copied())
                        .map(mapper)
                        .collect();
                    (pulses, 0, 0)
                } else {
                    let pulses = targets
                        .iter()
                        .copied()
                        .map(mapper)
                        .collect();
                    (pulses, 0, low_targets.len())
                }
            }
        }
    }

    pub fn name(&self) -> &'a str {
        match self {
            | Self::Button { name, .. }
            | Self::Broadcaster { name, .. }
            | Self::FlipFlop { name, .. }
            | Self::Conjunction { name, .. }
            | Self::Inverter { name, .. } => name,
        }
    }

    pub fn targets(&self) -> &[&'a str] {
        match self {
            | Self::Button { targets, .. }
            | Self::Broadcaster { targets, .. }
            | Self::FlipFlop { targets, .. }
            | Self::Conjunction { targets, .. }
            | Self::Inverter { targets, .. } => targets,
        }
    }

    pub fn low_targets(&self) -> &[&'a str] {
        match self {
            | Self::FlipFlop { low_targets, .. }
            | Self::Conjunction { low_targets, .. }
            | Self::Inverter { low_targets, .. } => low_targets,
            _ => &[],
        }
    }

    pub fn stately(&self) -> bool {
        matches!(self, Self::FlipFlop { .. } | Self::Conjunction { .. })
    }

    pub fn bits_of_state(&self) -> usize {
        match self {
            Self::Button { .. } | Self::Broadcaster { .. } | Self::Inverter { .. } => 0,
            Self::FlipFlop { state_mod, .. } => 1 + (*state_mod as f64).log2().ceil() as usize,
            Self::Conjunction { remembered, .. } => remembered.len(),
        }
    }

    pub fn is_low(&self) -> bool {
        matches!(self, Self::FlipFlop { .. })
    }

    pub fn flop(&mut self) {
        match self {
            Self::FlipFlop { on, .. } => *on = !*on,
            _ => panic!("Cannot flop {:?}", self),
        }
    }

    pub fn chain_flop(&mut self, into: &Self) {
        match (self, into) {
            (
                Self::FlipFlop {
                    name: _, targets, low_targets,
                    state_counter, state_mod, on,
                },
                Self::FlipFlop {
                    name: _, targets: into_targets, low_targets: into_low_targets,
                    on: into_on, state_counter: into_state_counter, state_mod: into_state_mod,
                }) => {
                    *state_counter += *state_mod * *on as usize;
                    *state_mod *= 2;
                    *state_mod *= *into_state_mod;
                    *state_counter += *into_state_mod * *into_state_counter;

                    *on = *into_on;

                    *targets = into_targets.clone();
                    *low_targets = into_low_targets.clone();
                },
            _ => panic!("Cannot chain non-flops with `chain_flop`"),
        }
    }

    pub fn set_targets(&mut self, targets: Vec<&'a str>) {
        match self {
            | Self::Button { targets: old_targets, .. }
            | Self::Broadcaster { targets: old_targets, .. }
            | Self::FlipFlop { targets: old_targets, .. }
            | Self::Conjunction { targets: old_targets, .. }
            | Self::Inverter { targets: old_targets, .. } => *old_targets = targets,
        }
    }

    pub fn set_low_targets(&mut self, low_targets: Vec<&'a str>) {
        match self {
            | Self::FlipFlop { low_targets: old_low_targets, .. }
            | Self::Conjunction { low_targets: old_low_targets, .. }
            | Self::Inverter { low_targets: old_low_targets, .. } => *old_low_targets = low_targets,
            _ => panic!("Cannot set low targets of {:?}", self),
        }
    }
}

impl Debug for Module<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        let targets = self.targets();
        let low_targets = self.low_targets();
        let symbol = match self {
            Self::Button { .. } | Self::Broadcaster { .. } => "",
            Self::FlipFlop { .. } => "%",
            // Self::Conjunction { .. } | Self::Inverter { .. } => "&",
            Self::Conjunction { .. } => "&",
            Self::Inverter { .. } => "!",
        };

        let after_name = match self {
            Self::FlipFlop { on, state_mod, state_counter, .. } => if *state_mod > 1 {
                format!(
                    " {state_counter:0width$b} {}",
                    if *on { "(on)" } else { "(off)" },
                    width=(*state_mod as f64).log2().ceil() as usize,
                )
            } else if *on {
                " (on)".to_string()
            } else {
                " (off)".to_string()
            },
            _ => "".to_string(),
        };

        write!(f, "{}{}{after_name} -> ", symbol, name)?;

        let mut iter = targets.iter().chain(low_targets);

        if let Some(name) = iter.next() {
            write!(f, "{}", name)?;
        }

        for name in iter {
            write!(f, ", {}", name)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PulseInfo<'a> {
    pub pulse: Pulse,
    pub source: &'a str,
    pub target: &'a str,
}
