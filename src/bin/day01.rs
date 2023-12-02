fn main() {
    // read input from the input dir/file
    let input = std::fs::read_to_string("input/day01.txt").unwrap(); 

    // call the functions for every line
    let sum1: usize = input.lines().map(part1_calibration_value).sum(); 
    let sum2: usize = input.lines().map(part2_calibration_value).sum(); 

    println!("Part1 answer: {sum1}");
    println!("Part2 answer: {sum2}");
}

fn part1_calibration_value(line: &str) -> usize {
    let first_char = line
        .chars()
        .find(|char| char.is_numeric())
        .unwrap();

    let last_char = line
        .chars()
        .rev() // reverse to get last char first
        .find(|char| char.is_numeric())
        .unwrap();

    format!("{first_char}{last_char}").parse().unwrap()
}

fn part2_calibration_value(line: &str) -> usize {
    let line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e"); 
    // fix cases where string/number intertwine
    // normalize the input

    part1_calibration_value(&line) // call function from part1
}