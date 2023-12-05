use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day04.txt").unwrap();

    let get_matching_cards: Vec<usize> = input.lines().map(get_matching_cards).collect(); 
    // call get_matching_cards() for every line of input, collect in a list/Vec

    let sum: usize = get_matching_cards.into_iter().map(calculate_card_score).sum();
    // call calculate_card_score for each element, sum values

    println!("Part1 answer: {sum}");
}

fn get_matching_cards(line: &str) -> usize {
    let (_, numbers) = line.split_once(':').unwrap(); 
    // remove text before winning numbers

    let (winning, numbers_to_check) = numbers.split_once('|').unwrap();
    // split numbers into winning and 

    let winning_set: HashSet<usize> = winning.split(' ').flat_map(str::parse).collect();
    let numbers_to_check_set: HashSet<usize> = numbers_to_check.split(' ').flat_map(str::parse).collect();

    winning_set.intersection(&numbers_to_check_set).count()
}

fn calculate_card_score(matches: usize) -> usize {
    if matches > 0 {
        (1..matches).fold(1, |acc, _| acc * 2)
    } else {
        0
    }
}