fn main() {
    // read input from the input dir/file
    let input = std::fs::read_to_string("input/day01").unwrap(); 
    // call the function for every line
    let sum1: usize = input.lines().map(part1_calibration_value).sum(); 

    println!("Part1: {sum1}");
}

fn part1_calibration_value(line: &str) -> usize {
    let first_char = line
        .chars()
        .find(|c| c.is_numeric())
        .unwrap();
    let last_char = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap();

    format!("{first_char}{last_char}").parse().unwrap()
}
