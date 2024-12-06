use std::fs;

use regex::Regex;

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();

    let line_length = text_input.lines().take(1).collect::<Vec<&str>>()[0].len();
    let columns = (0..line_length).map(|column| {
        text_input
            .lines()
            .map(move |line| line.chars().nth(column).unwrap())
    });
    let rows = text_input.lines().map(|line| line.chars());
    let diagonals = (0..line_length + text_input.lines().count()).map(|i| {
        text_input
            .lines()
            .enumerate()
            .map(move |(line_index, line)| {
                if line_index + line_length >= i {
                    line.chars().nth(line_index + line_length - i)
                } else {
                    None
                }
            })
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
    });

    let diagonals2 = (0..line_length + text_input.lines().count()).map(|i| {
        text_input
            .lines()
            .enumerate()
            .map(move |(line_index, line)| {
                if i >= line_index {
                    line.chars().nth(i - line_index)
                } else {
                    None
                }
            })
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
    });

    // println!(
    //     "{:?}",
    //     rows.map(|c| c.collect::<String>()).collect::<Vec<_>>()
    // );
    // println!(
    //     "{:?}",
    //     columns.map(|c| c.collect::<String>()).collect::<Vec<_>>()
    // );
    // println!(
    //     "{:?}",
    //     diagonals.map(|c| c.collect::<String>()).collect::<Vec<_>>()
    // );
    // println!(
    //     "{:?}",
    //     diagonals2
    //         .map(|c| c.collect::<String>())
    //         .collect::<Vec<_>>()
    // );

    let xmas_columns = get_xmas_count(columns);
    let xmas_rows = get_xmas_count(rows);
    let xmas_diagonals = get_xmas_count(diagonals);
    let xmas_diagonals2 = get_xmas_count(diagonals2);

    println!(
        "{}",
        xmas_columns + xmas_rows + xmas_diagonals + xmas_diagonals2
    );

    let x_masses = (1..text_input.lines().count() - 1).map(|line_index| {
        let text_input = text_input.clone();
        (1..line_length - 1).map(move |col_index| {
            let top_line = text_input.lines().nth(line_index - 1).unwrap();
            let middle_line = text_input.lines().nth(line_index).unwrap();
            let bottom_line = text_input.lines().nth(line_index + 1).unwrap();

            let mut text_1 = String::new();
            text_1.push(top_line.chars().nth(col_index - 1).unwrap());
            text_1.push(middle_line.chars().nth(col_index).unwrap());
            text_1.push(bottom_line.chars().nth(col_index + 1).unwrap());

            let mut text_2 = String::new();
            text_2.push(top_line.chars().nth(col_index + 1).unwrap());
            text_2.push(middle_line.chars().nth(col_index).unwrap());
            text_2.push(bottom_line.chars().nth(col_index - 1).unwrap());

            vec![text_1, text_2]
        })
    });

    let x_masses_count = x_masses.fold(0, |acc, x_masses| {
        acc + x_masses.fold(0, |acc, x_mas| {
            if (x_mas[0] == "MAS" || x_mas[0] == "SAM") && (x_mas[1] == "MAS" || x_mas[1] == "SAM")
            {
                acc + 1
            } else {
                acc
            }
        })
    });

    println!("{}", x_masses_count)
}

fn get_xmas_count<T: Iterator + Clone>(input: T) -> i32
where
    T::Item: Iterator<Item = char>,
{
    let xmas_regex = Regex::new(r"XMAS").expect("Invalid regex");
    let samx_regex = Regex::new(r"SAMX").expect("Invalid regex");

    let xmas_count: i32 = input
        .clone()
        .map(|line| {
            xmas_regex
                .captures_iter(line.collect::<String>().as_str())
                .count() as i32
        })
        .sum();

    let samx_count: i32 = input
        .map(|line| {
            samx_regex
                .captures_iter(line.collect::<String>().as_str())
                .count() as i32
        })
        .sum();

    xmas_count + samx_count
}
