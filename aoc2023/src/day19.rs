use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn parse(input: &str) -> utils::SortingSystem {
    let (workflows, part_ratings) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (workflow_name, rules) = line.trim_end_matches('}').split_once('{').unwrap();
            let rules = rules
                .split(',')
                .map(|rule| {
                    if let Some((condition, next_workflow)) = rule.split_once(':') {
                        let next_workflow = next_workflow.to_owned();
                        let category = condition.chars().next().unwrap().try_into().unwrap();
                        let value = condition[2..].parse().unwrap();
                        if condition.chars().nth(1).unwrap() == '<' {
                            utils::Condition::Less {
                                next_workflow,
                                category,
                                value,
                            }
                        } else {
                            utils::Condition::Greater {
                                next_workflow,
                                category,
                                value,
                            }
                        }
                    } else {
                        let next_workflow = rule.to_owned();
                        utils::Condition::None { next_workflow }
                    }
                })
                .collect();
            (workflow_name.to_owned(), rules)
        })
        .collect();

    let part_ratings = part_ratings
        .lines()
        .map(|line| {
            line.trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|l| {
                    let (category, value) = l.split_once('=').unwrap();
                    let category = category.chars().next().unwrap().try_into().unwrap();
                    let value = value.parse().unwrap();
                    (category, value)
                })
                .collect()
        })
        .collect();

    utils::SortingSystem {
        workflows,
        part_ratings,
    }
}

#[aoc(day19, part1)]
fn part1(input: &utils::SortingSystem) -> u64 {
    utils::count_accepted(input, "in")
}

#[aoc(day19, part2)]
fn part2(input: &utils::SortingSystem) -> u64 {
    use strum::IntoEnumIterator;
    utils::count_all_accepted_combinations_recursively(
        &input.workflows,
        utils::Category::iter().map(|ch| (ch, (1, 4000))).collect(),
        "in",
    )
}

mod utils {
    use strum::EnumIter;

    pub struct SortingSystem {
        pub workflows: rustc_hash::FxHashMap<String, smallvec::SmallVec<[Condition; 4]>>,
        pub part_ratings: Vec<rustc_hash::FxHashMap<Category, u64>>,
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub enum Condition {
        Less {
            next_workflow: String,
            category: Category,
            value: u64,
        },
        Greater {
            next_workflow: String,
            category: Category,
            value: u64,
        },
        None {
            next_workflow: String,
        },
    }

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
    pub enum Category {
        X = b'x',
        M = b'm',
        A = b'a',
        S = b's',
    }

    impl TryFrom<char> for Category {
        type Error = &'static str;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'x' => Ok(Category::X),
                'm' => Ok(Category::M),
                'a' => Ok(Category::A),
                's' => Ok(Category::S),
                _ => Err("Invalid category"),
            }
        }
    }

    pub fn count_accepted(sorting_system: &SortingSystem, first_workflow: &str) -> u64 {
        sorting_system
            .part_ratings
            .iter()
            .map(|rating| {
                let mut current_workflow = first_workflow;
                loop {
                    match current_workflow {
                        "A" => break rating.values().sum(),
                        "R" => break 0,
                        _ => {}
                    }
                    for condition in sorting_system.workflows.get(current_workflow).unwrap() {
                        match condition {
                            Condition::Less {
                                next_workflow,
                                category,
                                value,
                            } => {
                                if rating.get(category).unwrap() < value {
                                    current_workflow = next_workflow;
                                    break;
                                }
                            }
                            Condition::Greater {
                                next_workflow,
                                category,
                                value,
                            } => {
                                if rating.get(category).unwrap() > value {
                                    current_workflow = next_workflow;
                                    break;
                                }
                            }
                            Condition::None { next_workflow } => {
                                current_workflow = next_workflow;
                                break;
                            }
                        }
                    }
                }
            })
            .sum()
    }

    pub fn count_all_accepted_combinations_recursively(
        workflows: &rustc_hash::FxHashMap<String, smallvec::SmallVec<[Condition; 4]>>,
        mut ranges: rustc_hash::FxHashMap<Category, (u64, u64)>,
        current_workflow: &str,
    ) -> u64 {
        match current_workflow {
            "A" => {
                return ranges
                    .values()
                    .map(|(start, end)| end - start + 1)
                    .product()
            }
            "R" => return 0,
            _ => {}
        }

        workflows[current_workflow]
            .iter()
            .map(|condition| match condition {
                Condition::Less {
                    next_workflow,
                    category,
                    value,
                } => {
                    let &(start, end) = ranges.get(category).unwrap();

                    let within_bounds = (start, (value.saturating_sub(1)).min(end));
                    ranges.insert(*category, within_bounds);
                    let ranges_within_bounds = ranges.clone();

                    let outside_bounds = (*value.max(&start), end);
                    ranges.insert(*category, outside_bounds);

                    count_all_accepted_combinations_recursively(
                        workflows,
                        ranges_within_bounds,
                        next_workflow,
                    )
                }
                Condition::Greater {
                    next_workflow,
                    category,
                    value,
                } => {
                    let &(start, end) = ranges.get(category).unwrap();

                    let within_bounds = ((value.saturating_add(1)).max(start), end);
                    ranges.insert(*category, within_bounds);
                    let ranges_within_bounds = ranges.clone();

                    let outside_bounds = (start, *value.min(&end));
                    ranges.insert(*category, outside_bounds);

                    count_all_accepted_combinations_recursively(
                        workflows,
                        ranges_within_bounds,
                        next_workflow,
                    )
                }
                Condition::None { next_workflow } => count_all_accepted_combinations_recursively(
                    workflows,
                    ranges.clone(),
                    next_workflow,
                ),
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {r"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 19114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 167_409_079_868_000);
    }
}
