use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use crate::math;

#[derive(Debug)]
enum ModuleType<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(Vec<(&'a str, Pulse)>),
}

#[derive(Debug)]
struct Module<'a> {
    connections: Vec<&'a str>,
    mod_type: ModuleType<'a>,
    name: &'a str,
}

#[derive(Debug, Clone)]
enum Pulse {
    High,
    Low,
}

pub fn solve(suffix: &str) -> anyhow::Result<()> {
    let input = fs::read_to_string(format!("input/d20{}", suffix))?;
    let input = input.replace(" ", "");

    let mut modules = Vec::new();

    for line in input.lines() {
        let (name, conns) = line.split_once("->").unwrap();

        let connections = conns.split(',').collect::<Vec<_>>();
        let mod_type = match name.chars().nth(0).unwrap() {
            'b' => ModuleType::Broadcast,
            '%' => ModuleType::FlipFlop(false),
            '&' => ModuleType::Conjunction(Vec::new()),
            _ => panic!("Unknown mod type"),
        };

        modules.push(Module {
            connections,
            mod_type,
            name: &name[1..],
        });
    }

    let cmodules = modules
        .iter()
        .filter(|m| matches!(m.mod_type, ModuleType::Conjunction(_)))
        .map(|m| m.name)
        .collect::<Vec<_>>();
    let mut mod_watch = HashMap::new();
    for cname in cmodules {
        let mut inputs = modules
            .iter()
            .filter(|m| m.connections.contains(&cname))
            .map(|m| (m.name, Pulse::Low))
            .collect::<Vec<_>>();

        if let ModuleType::Conjunction(ref mut memory) = modules
            .iter_mut()
            .find(|m| m.name == cname)
            .unwrap()
            .mod_type
        {
            if cname == "rg" {
                for name in &inputs {
                    mod_watch.insert(name.0, None);
                }
            }
            memory.append(&mut inputs);
        }
    }

    let mut stack = VecDeque::new();
    stack.push_back((Pulse::Low, "roadcaster", "button"));
    let mut pulses: (u64, u64) = (0, 0);
    let mut presses: u64 = 1;
    while stack.len() > 0 {
        let (pulse, target, src) = stack.pop_front().unwrap();

        match pulse {
            Pulse::High => pulses.1 += 1,
            Pulse::Low => pulses.0 += 1,
        }

        if matches!(pulse, Pulse::High) && target == "rg" {
            mod_watch.insert(src, Some(presses));

            if mod_watch.values().all(|v| v.is_some()) {
                break;
            }
        }

        if let Some(module) = modules.iter_mut().find(|m| m.name == target) {
            let new_pulse = match &mut module.mod_type {
                ModuleType::Broadcast => Some(pulse.clone()),
                ModuleType::FlipFlop(ref mut on) => {
                    if matches!(pulse, Pulse::Low) {
                        *on = !*on;

                        if *on {
                            Some(Pulse::High)
                        } else {
                            Some(Pulse::Low)
                        }
                    } else {
                        None
                    }
                }
                ModuleType::Conjunction(ref mut memory) => {
                    memory.iter_mut().find(|(name, _)| *name == src).unwrap().1 = pulse.clone();

                    let all_high = memory.iter().all(|(_, val)| matches!(*val, Pulse::High));
                    if all_high {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
            };

            if let Some(pulse) = new_pulse {
                for tgt in &module.connections {
                    // println!(
                    //     "{} -{:?}-> {}",
                    //     module.name, pulse, tgt
                    // );
                    stack.push_back((pulse.clone(), tgt, module.name));
                }
            }
        }

        if stack.len() == 0 {
            presses += 1;

            if presses == 1001 {
                println!("Part one {}", pulses.0 * pulses.1);
            }

            stack.push_back((Pulse::Low, "roadcaster", "button"));
        }
    }

    println!(
        "Part two {}",
        math::lcm(&mod_watch.values().map(|v| v.unwrap()).collect::<Vec<u64>>())
    );

    Ok(())
}
