use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn parse(input: &str) -> utils::Modules {
    let mut modules: rustc_hash::FxHashMap<String, utils::Module> = input
        .lines()
        .map(|line| {
            let (module_id, outputs) = line.split_once("->").unwrap();
            let outputs = outputs.split(',').map(|s| s.trim().to_owned()).collect();

            let module_id = module_id.trim();
            match module_id {
                broadcaster if broadcaster == "broadcaster" => {
                    (broadcaster.to_owned(), utils::Module::Broadcast { outputs })
                }
                flipflop if flipflop.starts_with('%') => (
                    flipflop.chars().skip(1).collect(),
                    utils::Module::FlipFlop {
                        state: false,
                        outputs,
                    },
                ),
                conjunction if conjunction.starts_with('&') => (
                    conjunction.chars().skip(1).collect(),
                    utils::Module::Conjunction {
                        input_memory: rustc_hash::FxHashMap::default(),
                        outputs,
                    },
                ),
                _ => unreachable!(),
            }
        })
        .collect();

    // Create the final "rx" module (only found in outputs) and update the input memory of all conjunctions
    modules.clone().iter().for_each(|(name, module)| {
        module.outputs().iter().for_each(|output| {
            if let utils::Module::Conjunction { input_memory, .. } = modules
                .entry(output.clone())
                .or_insert(utils::Module::Final { activated: false })
            {
                input_memory.insert(name.clone(), utils::Pulse::Low);
            }
        });
    });

    utils::Modules::new(modules)
}

#[aoc(day20, part1)]
fn part1(input: &utils::Modules) -> usize {
    let mut input = input.clone();

    let (n_low, n_high) = (0..1000).fold((0, 0), |(mut acc_low, mut acc_high), _| {
        let mut active_pulses = std::collections::VecDeque::default();
        active_pulses.push_back((
            "button".to_owned(),
            "broadcaster".to_owned(),
            utils::Pulse::Low,
        ));
        while let Some((source_module, target_module, pulse)) = active_pulses.pop_front() {
            input
                .get_mut(&target_module)
                .unwrap()
                .propagate_pulse(&source_module, pulse)
                .into_iter()
                .for_each(|(next_target_module, next_pulse)| {
                    let next_source_module = target_module.clone();
                    active_pulses.push_back((next_source_module, next_target_module, next_pulse));
                });
            match pulse {
                utils::Pulse::Low => acc_low += 1,
                utils::Pulse::High => acc_high += 1,
            }
        }
        (acc_low, acc_high)
    });

    n_low * n_high
}

#[aoc(day20, part2)]
fn part2(input: &utils::Modules) -> usize {
    let mut input = input.clone();

    // Find all modules that branch into a conjunction that is connected to the final "rx" module
    let mut final_module_sources = vec!["rx".to_string()];
    while final_module_sources.len() == 1 {
        let final_module_source = final_module_sources.pop().unwrap();
        match input[&final_module_source] {
            utils::Module::Conjunction { .. } | utils::Module::Final { .. } => {}
            _ => {
                unimplemented!("It is assumed that a single conjunction is connected to the final \"rx\" module");
            }
        }
        final_module_sources = input
            .iter()
            .filter_map(|(name, module)| {
                module
                    .outputs()
                    .contains(&final_module_source)
                    .then_some(name.clone())
            })
            .collect();
    }
    let mut final_module_source_loop_lengths: rustc_hash::FxHashMap<String, usize> =
        final_module_sources
            .iter()
            .map(|name| (name.clone(), 0))
            .collect();

    // Keep pressing the button until all loops lengths are determined
    'outer: for i in 1.. {
        let mut active_pulses = std::collections::VecDeque::default();
        active_pulses.push_back((
            "button".to_owned(),
            "broadcaster".to_owned(),
            utils::Pulse::Low,
        ));
        while let Some((source_module, target_module, pulse)) = active_pulses.pop_front() {
            for (next_target_module, next_pulse) in input
                .get_mut(&target_module)
                .unwrap()
                .propagate_pulse(&source_module, pulse)
            {
                let next_source_module = target_module.clone();
                active_pulses.push_back((next_source_module, next_target_module, next_pulse));

                // Only high pulses are considered because it is assumed that a single conjunction is connected to the final "rx" module
                if final_module_sources.contains(&source_module) && pulse == utils::Pulse::High {
                    final_module_source_loop_lengths.insert(source_module.clone(), i);

                    // Break once the length of all loops is determined
                    if final_module_source_loop_lengths.values().all(|x| *x > 1) {
                        break 'outer;
                    }
                }
            }
        }
    }

    // Find the least common multiple of all loop lengths
    final_module_source_loop_lengths
        .values()
        .fold(1, |acc, &x| num::integer::lcm(acc, x))
}

mod utils {
    #[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
    pub struct Modules(rustc_hash::FxHashMap<String, Module>);

    impl Modules {
        pub fn new(value: rustc_hash::FxHashMap<String, Module>) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Pulse {
        Low,
        High,
    }

    #[derive(Clone)]
    pub enum Module {
        Broadcast {
            outputs: smallvec::SmallVec<[String; 7]>,
        },
        FlipFlop {
            outputs: smallvec::SmallVec<[String; 7]>,
            state: bool,
        },
        Conjunction {
            outputs: smallvec::SmallVec<[String; 7]>,
            input_memory: rustc_hash::FxHashMap<String, Pulse>,
        },
        Final {
            activated: bool,
        },
    }

    impl Module {
        pub fn outputs(&self) -> &[String] {
            match self {
                Module::Broadcast { outputs }
                | Module::FlipFlop { outputs, .. }
                | Module::Conjunction { outputs, .. } => outputs,
                Module::Final { .. } => &[],
            }
        }

        pub fn propagate_pulse(
            &mut self,
            source_module: &str,
            pulse: Pulse,
        ) -> smallvec::SmallVec<[(String, Pulse); 7]> {
            match self {
                Module::Broadcast { outputs } => {
                    return outputs
                        .iter()
                        .map(|output| (output.to_owned(), pulse))
                        .collect();
                }
                Module::FlipFlop { outputs, state } => {
                    if pulse == Pulse::Low {
                        let new_pulse = if *state { Pulse::Low } else { Pulse::High };
                        *state = !*state;
                        return outputs
                            .iter()
                            .map(|output| (output.to_owned(), new_pulse))
                            .collect();
                    }
                }
                Module::Conjunction {
                    outputs,
                    input_memory,
                } => {
                    *input_memory.get_mut(source_module).unwrap() = pulse;
                    let pulse = if input_memory.values().all(|v| *v == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    return outputs
                        .iter()
                        .map(|output| (output.to_owned(), pulse))
                        .collect();
                }
                Module::Final { activated } => {
                    if pulse == Pulse::Low {
                        *activated = true;
                    }
                }
            }
            smallvec::SmallVec::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLES: [&str; 2] = [
        indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "},
        indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> outputs
        "},
    ];

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLES[0])), 32_000_000);
        assert_eq!(part1(&parse(SAMPLES[1])), 11_687_500);
    }

    #[test]
    #[ignore]
    fn part2_example() {}
}
