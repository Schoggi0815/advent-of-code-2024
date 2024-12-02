use std::fs;

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();

    let numbers = text_input
        .lines()
        .map(|line| {
            line.rsplit("   ")
                .map(|num_text| num_text.parse::<i32>().expect("Not a number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut left_numbers = numbers.iter().map(|n| n[0]).collect::<Vec<i32>>();
    let mut right_numbers = numbers.iter().map(|n| n[1]).collect::<Vec<i32>>();
    left_numbers.sort();
    right_numbers.sort();

    let mut difference = 0;

    for i in 0..left_numbers.len() {
        difference += (left_numbers[i] - right_numbers[i]).abs();
    }

    let similarity = left_numbers
        .iter()
        .map(|num| right_numbers.iter().filter(|n| *n == num).count() as i32 * num)
        .fold(0, |acc, count| acc + count);

    println!("difference: {}, similarity: {}", difference, similarity);
}
