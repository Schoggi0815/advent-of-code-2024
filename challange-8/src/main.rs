use std::fs;

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();

    let antennas = text_input
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(index2, character)| {
                    if character == '.' {
                        return None;
                    }

                    Some(Antenna {
                        x: index2 as i32,
                        y: index as i32,
                        frequency: character,
                    })
                })
        })
        .collect::<Vec<Antenna>>();

    let y_bounds = text_input.lines().count() as i32;
    let x_bounds = text_input.lines().collect::<Vec<&str>>()[0].len() as i32;

    let mut antinodes: Vec<Antinode> = Vec::new();
    let mut antinodes2: Vec<Antinode> = Vec::new();

    for antenna in &antennas {
        for antenna2 in &antennas {
            if (antenna.x == antenna2.x && antenna.y == antenna2.y)
                || antenna.frequency != antenna2.frequency
            {
                continue;
            }

            let diff_x = antenna.x - antenna2.x;
            let diff_y = antenna.y - antenna2.y;

            let antinode = Antinode(antenna.x + diff_x, antenna.y + diff_y);

            if antinode.0 >= 0 && antinode.0 < x_bounds && antinode.1 >= 0 && antinode.1 < y_bounds
            {
                antinodes.push(antinode);
            }

            let mut antinode2 = Antinode(antenna.x, antenna.y);

            while antinode2.0 >= 0
                && antinode2.0 < x_bounds
                && antinode2.1 >= 0
                && antinode2.1 < y_bounds
            {
                antinodes2.push(antinode2.clone());

                antinode2 = Antinode(antinode2.0 + diff_x, antinode2.1 + diff_y);
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();

    antinodes2.sort();
    antinodes2.dedup();

    println!("{:?}", antinodes.len());
    println!("{:?}", antinodes2.len());
}

struct Antenna {
    x: i32,
    y: i32,
    frequency: char,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Antinode(i32, i32);
