use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pulse { Hi, Lo }


#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Module<'a> {
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
    },
    Conjunction {
        name: &'a str,
        remembered: Vec<(&'a str, Pulse)>,
        targets: Vec<&'a str>,
    },
    Inverter {
        name: &'a str,
        targets: Vec<&'a str>,
    }
}



impl<'a> Module<'a> {
    pub fn parse(line: &'a str) -> Self {
        let (name, targets) = line.split_once(" -> ").unwrap();
        let targets = targets.split(", ").collect();
        
        match name.chars().next().unwrap() {
            '%' => Self::FlipFlop { name: &name[1..], on: false, targets, state_counter: 0, state_mod: 1 },
            '&' => Self::Conjunction { name: &name[1..], remembered: vec![], targets },
            _ => Self::Broadcaster { name, targets },
        }
    }

    pub fn setup_inputs(&mut self, inputs: &[&'a str]) {
        match self {
            | Self::Broadcaster { .. }
            | Self::FlipFlop { .. }
            | Self::Inverter { .. } => {},
            Self::Conjunction { name, remembered, targets } => {
                if inputs.len() == 1 {
                    *self = Self::Inverter { name, targets: std::mem::take(targets) };
                } else {
                    *remembered = inputs.iter().map(|name| (*name, Pulse::Lo)).collect();
                }
            },
        }
    }

    pub fn input_pulse(&mut self, pulse: PulseInfo<'a>) -> (Vec<PulseInfo<'a>>, usize, usize) {
        match self {
            Self::Broadcaster { name, targets } => {
                let pulses = targets
                    .iter()
                    .copied()
                    .map(|target| PulseInfo { source: name, target, pulse: Pulse::Lo })
                    .collect();

                (pulses, 0, 0)
            },
            Module::FlipFlop {
                name, targets,
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

                let pulses = targets
                    .iter()
                    .copied()
                    .map(mapper)
                    .collect();
                (pulses, lo_pulses, hi_pulses)
            }
            Module::Conjunction { name, targets, remembered } => {
                remembered.iter_mut().find(|(source, _)| *source == pulse.source).unwrap().1 = pulse.pulse;

                let all_high = remembered.iter().all(|(_, pulse)| *pulse == Pulse::Hi);
                let sent_pulse = if all_high { Pulse::Lo } else { Pulse::Hi };

                let mapper = |target| PulseInfo { source: name, target, pulse: sent_pulse };
                let pulses = targets
                    .iter()
                    .copied()
                    .map(mapper)
                    .collect();
                (pulses, 0, 0)
            },
            Module::Inverter { name, targets } => {
                let sent_pulse = if pulse.pulse == Pulse::Hi { Pulse::Lo } else { Pulse::Hi };

                let mapper = |target| PulseInfo { source: name, target, pulse: sent_pulse };

                let pulses = targets
                    .iter()
                    .copied()
                    .map(mapper)
                    .collect();
                (pulses, 0, 0)
            }
        }
    }

    pub fn name(&self) -> &'a str {
        match self {
            | Self::Broadcaster { name, .. }
            | Self::FlipFlop { name, .. }
            | Self::Conjunction { name, .. }
            | Self::Inverter { name, .. } => name,
        }
    }

    pub fn targets(&self) -> &[&'a str] {
        match self {
            | Self::Broadcaster { targets, .. }
            | Self::FlipFlop { targets, .. }
            | Self::Conjunction { targets, .. }
            | Self::Inverter { targets, .. } => targets,
        }
    }

    pub fn chain_flop(&mut self, into: &Self) {
        match (self, into) {
            (
                Self::FlipFlop {
                    name: _, targets,
                    state_counter, state_mod, on,
                },
                Self::FlipFlop {
                    name: _, targets: into_targets,
                    on: into_on, state_counter: into_state_counter, state_mod: into_state_mod,
                }) => {
                    *state_counter += *state_mod * *on as usize;
                    *state_mod *= 2;
                    *state_mod *= *into_state_mod;
                    *state_counter += *into_state_mod * *into_state_counter;

                    *on = *into_on;

                    *targets = into_targets.clone();
                },
            _ => panic!("Cannot chain non-flops with `chain_flop`"),
        }
    }
}

impl Debug for Module<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        let targets = self.targets();
        let symbol = match self {
            Self::Broadcaster { .. } => "",
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

        let mut iter = targets.iter();

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
