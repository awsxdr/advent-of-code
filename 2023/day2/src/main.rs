use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct GameInfo {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let lines = input.split('\n');

    let game_ids = 
        lines
            .map(GameInfo::parse)
            .filter_map(|g| if g.max_red <= MAX_RED && g.max_green <= MAX_GREEN && g.max_blue <= MAX_BLUE { Some(g.id) } else { None })
            .collect::<Vec<u32>>();

    println!("{:?}", game_ids.iter().sum::<u32>());
}

impl GameInfo {
    fn parse(line: &str) -> Self {
        let game_regex = Regex::new(r"Game (\d+)").unwrap();
        let red_regex = Regex::new(r"(\d+) red").unwrap();
        let green_regex = Regex::new(r"(\d+) green").unwrap();
        let blue_regex = Regex::new(r"(\d+) blue").unwrap();

        let id = game_regex.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();

        let get_max_color = |regex: Regex| -> u32 {
            regex.captures_iter(line)
                .filter_map(|c| c.get(1))
                .map(|d| d.as_str().parse::<u32>().unwrap())
                .max()
                .unwrap()
        };

        let max_red = get_max_color(red_regex);
        let max_green = get_max_color(green_regex);
        let max_blue = get_max_color(blue_regex);

        Self {
            id,
            max_red,
            max_green,
            max_blue,
        }
    }
}