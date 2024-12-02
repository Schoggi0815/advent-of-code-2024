use std::fs;

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();

    let records = text_input.lines().map(|line| {
        line.rsplit(" ")
            .map(|num_text| num_text.parse::<i32>().expect("Not a number"))
            .collect::<Vec<i32>>()
    });

    let save_count = records.filter(error_damper).count();

    println!("{}", save_count);
}

fn error_damper(record: &Vec<i32>) -> bool {
    if is_save(record) {
        return true;
    }

    for i in 0..record.len() {
        let mut record_copy = record.clone();
        record_copy.remove(i);

        if is_save(&record_copy) {
            return true;
        }
    }

    false
}

fn is_save(record: &Vec<i32>) -> bool {
    let mut last_level = record[0];
    let direction = if record[0] < record[1] {
        Direction::Positive
    } else {
        Direction::Negative
    };

    for level in record.iter().skip(1) {
        let difference = (level - last_level).abs();
        if difference > 3 || difference < 1 {
            return false;
        }

        if direction == Direction::Positive && level < &last_level {
            return false;
        }

        if direction == Direction::Negative && level > &last_level {
            return false;
        }

        last_level = *level;
    }

    true
}

#[derive(PartialEq)]
enum Direction {
    Positive,
    Negative,
}
