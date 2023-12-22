use std::{collections::{HashMap, HashSet, BTreeMap}, fmt::Debug};

use crate::modules::{Module, PulseInfo, Pulse};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModuleSet<'a> {
    modules: BTreeMap<&'a str, Module<'a>>,
}

impl<'a> ModuleSet<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            modules: input.lines().map(Module::parse).map(|module| (module.name(), module)).collect()
        }
    }

    pub fn inputs_to(&self, name: &'a str) -> Vec<&'a str> {
        let mut inputs = vec![];
        for module in self.modules.values() {
            if module.targets().contains(&name) || module.low_targets().contains(&name) {
                inputs.push(module.name());
            }
        }
        inputs
    }

    pub fn update_inputs(&mut self) {
        let inputs: HashMap<_, _> = self.modules.keys().map(|name| (*name, self.inputs_to(name))).collect();
        let low_inputs = self.low_inputs();

        for (name, module) in self.modules.iter_mut() {
            let inputs = &inputs[name];
            module.setup_inputs(inputs);
            module.move_to_low(&low_inputs);
        }
    }

    pub fn low_inputs(&self) -> HashSet<&'a str> {
        self.modules.iter().filter(|(_, module)| module.is_low()).map(|(name, _)| *name).collect()
    }

    pub fn pulse(&mut self, monitor: &str) -> (usize, usize, bool, Vec<(usize, Pulse)>) {
        let mut lo_pulses = 1;
        let mut hi_pulses = 0;
        let mut rx_pulse_seen = false;

        let mut monitoring = vec![];

        let mut pending_pulses = self.modules.get_mut("broadcaster").unwrap().input_pulse(PulseInfo {
            pulse: crate::modules::Pulse::Lo,
            source: "button",
            target: "broadcaster",
        }).0;

        lo_pulses += pending_pulses.iter().filter(|pulse| pulse.pulse == Pulse::Lo).count();
        hi_pulses += pending_pulses.iter().filter(|pulse| pulse.pulse == Pulse::Hi).count();

        for i in 0.. {
            if pending_pulses.is_empty() {
                break;
            }

            let mut new_pending_pulses = vec![];
            for pulse in pending_pulses {
                if pulse.target == "rx" && pulse.pulse == Pulse::Lo {
                    rx_pulse_seen = true;
                }

                if pulse.target == monitor {
                    monitoring.push((i, pulse.pulse));
                }

                let Some(target) = self.modules.get_mut(pulse.target) else { continue };
                let (pulses, irrel_lo_pulses, irrel_hi_pulses) = target.input_pulse(pulse);
                lo_pulses += irrel_lo_pulses;
                hi_pulses += irrel_hi_pulses;
                new_pending_pulses.extend(pulses);
            }

            pending_pulses = new_pending_pulses;

            lo_pulses += pending_pulses.iter().filter(|pulse| pulse.pulse == Pulse::Lo).count();
            hi_pulses += pending_pulses.iter().filter(|pulse| pulse.pulse == Pulse::Hi).count();


        }

        (lo_pulses, hi_pulses, rx_pulse_seen, monitoring)
    }

    pub fn bits_of_state(&self) -> usize {
        self.modules.values().map(|module| module.bits_of_state()).sum()
    }

    pub fn dependents_of(&self, name: &'a str) -> HashSet<&'a str> {
        let mut dependents = HashSet::new();
        let mut step_dependents: HashSet<_> = [name].into_iter().collect();
        while !step_dependents.is_empty() {
            let mut new_step_dependents = HashSet::new();
            for dep in step_dependents {
                let Some(module) = self.modules.get(dep) else { continue };

                new_step_dependents.extend(module.targets().iter().chain(module.low_targets().iter()));
            }
            step_dependents = new_step_dependents.difference(&dependents).copied().collect();
            dependents.extend(step_dependents.iter());
        }

        dependents
    }

    pub fn has_loops(&self) -> bool {
        let mut loop_found = false;
        for name in self.modules.keys() {
            if self.dependents_of(name).contains(name) {
                loop_found = true;
            }
        }
        loop_found
    }

    pub fn reduce_flip_flop_to_inverter(&mut self) -> bool {
        let mut flip_flops = vec![];
        for (name, module) in self.modules.iter() {
            if let Module::FlipFlop { .. } = module {
                flip_flops.push(*name);
            }
        }

        let mut modified = false;

        for name in flip_flops {
            let targets = self.modules[&name].targets();
            let low_targets = self.modules[&name].low_targets();

            if !targets.is_empty() || low_targets.len() != 1 { continue; }

            if let Module::Inverter { targets, low_targets, name: inverter_name } = self.modules[&low_targets[0]].clone() {
                self.modules.remove(inverter_name);
                self.modules.get_mut(name).unwrap().flop();
                self.modules.get_mut(name).unwrap().set_targets(targets.clone());
                self.modules.get_mut(name).unwrap().set_low_targets(low_targets.clone());
                modified = true;
            }
        }

        modified
    }

    pub fn reduce_flip_flop_to_flip_flop(&mut self) -> bool {
        let mut flip_flops = vec![];
        for (name, module) in self.modules.iter() {
            if let Module::FlipFlop { .. } = module {
                flip_flops.push(*name);
            }
        }

        let mut modified = false;

        for name in flip_flops {
            if self.modules.get(name).is_none() { continue; }

            let targets = self.modules[&name].targets();
            let low_targets = self.modules[&name].low_targets();

            if !targets.is_empty() || low_targets.len() != 1 { continue; }
            
            if matches!(self.modules[&low_targets[0]], Module::FlipFlop { .. }) {
                if self.inputs_to(self.modules[&low_targets[0]].name()).len() != 1 { continue }

                let into_flop = self.modules.remove(self.modules[&low_targets[0]].name()).unwrap();
                self.modules.get_mut(name).unwrap().chain_flop(&into_flop);
                modified = true;
            }
        }

        modified
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    pub fn subsets(&self) -> impl Iterator<Item = Self> + '_ {
        let broadcaster = self.modules.get("broadcaster").unwrap();

        broadcaster.targets().iter().chain(broadcaster.low_targets()).map(|name| {
            let subset_dependents = self.dependents_of(name);
            let subset_modules = self.modules.iter()
                .filter(|(&name, _)| subset_dependents.contains(name) || name == "broadcaster")
                .map(|(_, module)| (module.name(), module.clone()))
                .collect();
            Self { modules: subset_modules }
        })
    }
}

impl Debug for ModuleSet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for module in self.modules.values() {
            writeln!(f, "{:?}", module)?;
        }
        Ok(())
    }
}
