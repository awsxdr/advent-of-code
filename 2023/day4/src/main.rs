struct ScratchCard {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    //let input = std::fs::read_to_string("./small_input.txt").unwrap();

    let result = 
        input.lines()
        .map(|l| ScratchCard::parse(l))
        .map(|c| c.get_value())
        .sum::<u32>();

    println!("{}", result);
}

impl ScratchCard {
    fn parse(line: &str) -> Self {
        let line_parts = line.split(|c| c == '|' || c == ':').collect::<Vec<&str>>();

        let get_numbers = |index: usize| 
            line_parts.get(index).unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

        let winning_numbers = get_numbers(1);
        let numbers = get_numbers(2);

        Self {
            winning_numbers,
            numbers
        }
    }

    fn get_value(&self) -> u32 {
        Self::value_from_count(
            self.winning_numbers.iter()
                .filter(|n| self.numbers.contains(n))
                .count())
    }

    fn value_from_count(count: usize) -> u32 {
        if count == 0 {
            0
        } else {
            1 << (count - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn parse_extracts_expected_values() {
        let input = "Game   2: 10  2 12 40 |  1  2 11 44";
        let winning_numbers = vec!(10, 2, 12, 40);
        let numbers = vec!(1, 2, 11, 44);

        let card = ScratchCard::parse(input);

        assert_eq!(card.winning_numbers, winning_numbers);
        assert_eq!(card.numbers, numbers);
    }

    #[test_case(vec!(10, 2, 12, 40), vec!(1, 3, 11, 44), 0 ; "when no numbers match")]
    #[test_case(vec!(10, 2, 12, 40), vec!(1, 2, 11, 44), 1 ; "when one number matches")]
    #[test_case(vec!(10, 2, 12, 40), vec!(2, 40, 12, 10), 8 ; "when four numbers match")]
    fn get_value_returns_expected_value(winning_numbers: Vec<u32>, numbers: Vec<u32>, expected_value: u32) {
        let card = ScratchCard {
            winning_numbers,
            numbers
        };

        assert_eq!(card.get_value(), expected_value);
    }
}