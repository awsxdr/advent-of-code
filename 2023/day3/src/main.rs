use regex::Regex;

#[derive(Debug)]
struct MapNumber {
    value: u32,
    x: usize,
    y: usize,
    width: usize,
}

#[derive(Debug)]
struct MapSymbol {
    x: usize,
    y: usize,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    //let input = std::fs::read_to_string("./small_input.txt").unwrap();

    let numbers_regex = Regex::new(r"\d+").unwrap();
    let numbers: Vec<MapNumber> = input.lines().into_iter().enumerate().flat_map(|(y, line)|
        numbers_regex.captures_iter(line).map(|capture| {
            let group = capture.get(0).unwrap();
            let x = group.start();
            MapNumber::parse(group.as_str(), x, y)
        })
        .collect::<Vec<MapNumber>>()
    ).collect();

    let symbols_regex = Regex::new(r"[^\.\d]").unwrap();
    let symbols: Vec<MapSymbol> = input.lines().into_iter().enumerate().flat_map(|(y, line)|
        symbols_regex.captures_iter(line).map(|capture| {
            let x = capture.get(0).unwrap().start();
            MapSymbol::new(x, y)
        })
        .collect::<Vec<MapSymbol>>()
    ).collect();

    let valid_numbers: Vec<&MapNumber> = numbers.iter()
        .filter(|n| symbols.iter().any(|s| n.is_next_to(s)))
        .collect();

    let result = valid_numbers.iter().map(|n| n.value).sum::<u32>();

    println!("{}", result);
}

impl MapNumber {
    fn parse(value: &str, x: usize, y: usize) -> Self {
        let width = value.len();
        let value = value.parse::<u32>().unwrap();

        Self {
            value,
            x,
            y,
            width,
        }
    }

    fn is_next_to(&self, symbol: &MapSymbol) -> bool {
        let min_x = if self.x > 0 { self.x - 1 } else { self.x };
        let min_y = if self.y > 0 { self.y - 1 } else { self.y };
        let max_x = self.x + self.width;
        let max_y = self.y + 1;

        symbol.x >= min_x && symbol.y >= min_y && symbol.x <= max_x && symbol.y <= max_y
    }
}

impl MapSymbol {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, 2, 1, 1 ; "when symbol to top left")]
    #[test_case(2, 2, 3, 1 ; "when symbol above")]
    #[test_case(2, 2, 5, 1 ; "when symbol to top right")]
    #[test_case(2, 2, 1, 2 ; "when symbol to left")]
    #[test_case(2, 2, 5, 2 ; "when symbol to right")]
    #[test_case(2, 2, 1, 3 ; "when symbol to bottom left")]
    #[test_case(2, 2, 3, 3 ; "when symbol below")]
    #[test_case(2, 2, 5, 3 ; "when symbol to bottom right")]
    #[test_case(0, 0, 3, 0 ; "when number in top left and symbol to right")]
    #[test_case(0, 0, 1, 1 ; "when number in top left and symbol below")]
    #[test_case(0, 0, 3, 1 ; "when number in top left and symbol to bottom right")]
    fn is_next_to_returns_true(number_x: usize, number_y: usize, symbol_x: usize, symbol_y: usize) {
        let number = MapNumber::parse("123", number_x, number_y);
        let symbol = MapSymbol::new(symbol_x, symbol_y);

        assert!(number.is_next_to(&symbol));
    }

    #[test_case(2, 2, 0, 0 ; "when symbol too far to top left")]
    #[test_case(2, 2, 2, 0 ; "when symbol too far above")]
    #[test_case(2, 2, 4, 0 ; "when symbol too far to top right")]
    #[test_case(2, 2, 0, 2 ; "when symbol too far to left")]
    #[test_case(2, 2, 6, 2 ; "when symbol too far to right")]
    #[test_case(2, 2, 0, 4 ; "when symbol too far to bottom left")]
    #[test_case(2, 2, 2, 4 ; "when symbol too far below")]
    #[test_case(2, 2, 4, 4 ; "when symbol too far to bottom right")]
    fn is_next_to_returns_false(number_x: usize, number_y: usize, symbol_x: usize, symbol_y: usize) {
        let number = MapNumber::parse("123", number_x, number_y);
        let symbol = MapSymbol::new(symbol_x, symbol_y);

        assert!(!number.is_next_to(&symbol));
    }
}