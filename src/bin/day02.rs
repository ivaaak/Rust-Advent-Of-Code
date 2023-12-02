fn main() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();

    let games: Vec<Game> = input
        .lines()
        .map(|l| l.into())
        .collect();

    let part1: usize = games
        .iter()
        .filter(|&g| g.isValid(12, 13, 14))
        .map(|g| g.id)
        .sum();

    let part2: usize = games
        .iter()
        .map(Game::multiplyCubes) // call multiplyCubes for every game
        .sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

struct Game {
    id: usize,
    max_red_cubes: usize,
    max_green_cubes: usize,
    max_blue_cubes: usize,
}

impl Game {
    pub fn isValid(&self, reds: usize, greens: usize, blues: usize) -> bool {
        self.max_red_cubes <= reds &&
        self.max_green_cubes <= greens &&
        self.max_blue_cubes <= blues
    }

    pub fn multiplyCubes(&self) -> usize {
        self.max_red_cubes * self.max_green_cubes * self.max_blue_cubes
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (meta, sets) = line.split_once(':').unwrap();

        let (_, id) = meta.split_once(' ').unwrap();

        let id: usize = id.parse().unwrap();

        let sets = sets
            .trim()
            .split(';')
            .map(|s| s.split(',')  
                      .map(str::trim));

        let mut game = Game {
            id,
            max_red_cubes: 0,
            max_green_cubes: 0,
            max_blue_cubes: 0,
        };

        for set in sets {
            for draw in set {
                let (number, color) = draw.split_once(' ').unwrap();
                let number: usize = number.parse().unwrap();

                match color { // get max value for each color
                    "red" => game.max_red_cubes = number.max(game.max_red_cubes),
                    "green" => game.max_green_cubes = number.max(game.max_green_cubes),
                    "blue" => game.max_blue_cubes = number.max(game.max_blue_cubes),
                    _ => unreachable!(),
                }
            }
        }

        game
    }
}
