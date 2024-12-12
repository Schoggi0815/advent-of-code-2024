use std::{collections::HashSet, fs};

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();

    let mut input = text_input.split("\n\n");
    let first_input = input.next().unwrap();
    let second_input = input.next().unwrap();

    let rules = first_input
        .lines()
        .map(|line| {
            line.split("|")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| Rule(v[0], v[1]));

    let rule_map: HashSet<Rule> = rules.collect();

    // print!("{:?}", rules.collect::<Vec<Rule>>());

    let commands = second_input.lines().map(|line| {
        Command(
            line.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    });

    // print!("{:?}", commands.collect::<Vec<Command>>());

    let result_1 = commands
        .clone()
        .filter(|c| c.conforms_rules(&rule_map))
        .map(|c| c.get_middle_value())
        .sum::<i32>();

    println!("{}", result_1);

    let result_2 = commands
        .filter(|c| !c.conforms_rules(&rule_map))
        .map(|c| c.repair_command(&rule_map).get_middle_value())
        .sum::<i32>();

    println!("{}", result_2);
}

#[derive(Debug, PartialEq, Eq, Hash)]

struct Rule(i32, i32);

#[derive(Debug)]
struct Command(Vec<i32>);

impl Command {
    fn conforms_rules(&self, rule_map: &HashSet<Rule>) -> bool {
        for i in 0..self.0.len() {
            let current = self.0[i];
            let following = self.0[(i + 1)..self.0.len()].to_vec();
            for next in following {
                if rule_map.contains(&Rule(next, current)) {
                    return false;
                }
            }
        }

        true
    }

    fn get_middle_value(&self) -> i32 {
        self.0[self.0.len() / 2]
    }

    fn repair_command(mut self, rule_map: &HashSet<Rule>) -> Self {
        for i in 0..self.0.len() {
            let current = self.0[i];
            let following = self.0[(i + 1)..self.0.len()].to_vec();
            for next_i in 0..following.len() {
                let next = following[next_i];
                if rule_map.contains(&Rule(next, current)) {
                    self.0.swap(i, next_i + i + 1);
                    return self.repair_command(rule_map);
                }
            }
        }

        self
    }
}
