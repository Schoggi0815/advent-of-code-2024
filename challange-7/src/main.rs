use std::fs;

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();

    let calibrations = text_input.lines().map(|line| {
        let inputs = line.split(": ").collect::<Vec<&str>>();

        let total = inputs[0].parse::<i64>().unwrap();
        let numbers = inputs[1]
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Calibration(total, numbers)
    });

    let result = calibrations
        .clone()
        .filter(|c| c.check_if_possible())
        .map(|c| c.0)
        .sum::<i64>();
    println!("{}", result);

    let result2 = calibrations
        .filter(|c| c.check_if_possible2())
        .map(|c| c.0)
        .sum::<i64>();
    println!("{}", result2);
}

struct Calibration(i64, Vec<i64>);

impl Calibration {
    fn check_if_possible(&self) -> bool {
        self.check_if_possible_rec(self.0, self.1[0], &self.1[1..])
    }

    fn check_if_possible_rec(
        &self,
        solution: i64,
        current_value: i64,
        following_numbers: &[i64],
    ) -> bool {
        if following_numbers.len() == 0 {
            return current_value == solution;
        }

        let next_number = following_numbers[0];
        self.check_if_possible_rec(
            solution,
            current_value + next_number,
            &following_numbers[1..],
        ) || self.check_if_possible_rec(
            solution,
            current_value * next_number,
            &following_numbers[1..],
        )
    }

    fn check_if_possible2(&self) -> bool {
        self.check_if_possible_rec2(self.0, self.1[0], &self.1[1..])
    }

    fn check_if_possible_rec2(
        &self,
        solution: i64,
        current_value: i64,
        following_numbers: &[i64],
    ) -> bool {
        if following_numbers.len() == 0 {
            return current_value == solution;
        }

        let next_number = following_numbers[0];
        self.check_if_possible_rec2(
            solution,
            current_value + next_number,
            &following_numbers[1..],
        ) || self.check_if_possible_rec2(
            solution,
            current_value * next_number,
            &following_numbers[1..],
        ) || self.check_if_possible_rec2(
            solution,
            concat(current_value, next_number),
            &following_numbers[1..],
        )
    }
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}
