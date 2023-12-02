use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let lines = input.split('\n');

    let result = lines
        .into_iter()
        .map(get_digits)
        .map(|d| format!("{}{}", d.first().unwrap(), d.last().unwrap()).parse::<u32>().unwrap())
        .sum::<u32>();

    println!("{}", result);
}

fn get_digits(line: &str) -> Vec<u32> {
    let mappings = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    let mut digits: Vec<(usize, u32)> = 
        mappings.into_iter()
        .flat_map(|(k, v)| line.match_indices(k).map(|(i, _)| (i, v)).collect::<Vec<(usize, u32)>>())
        .chain(line.char_indices().into_iter().filter_map(|(i, c)| if c.is_numeric() { Some((i, c.to_string().parse::<u32>().unwrap())) } else { None }))
        .collect();

    digits.sort_by(|(i, _), (j, _)| i.cmp(j));

    digits.into_iter().map(|(_, v)| v).collect()
}

