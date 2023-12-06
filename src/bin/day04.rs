use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day04.txt").unwrap();

    let get_matching_cards: Vec<usize> = input.lines().map(get_matching_cards).collect(); 
    // call get_matching_cards() for every line of input, collect in a list/Vec

    let sum: usize = get_matching_cards.clone().into_iter().map(calculate_card_score).sum();
    // call calculate_card_score for each element, sum values
    let sum2: usize = calculate_card_count(&get_matching_cards);

    println!("Part 1 answer: {sum}");
    println!("Part 2 answer: {sum2}");

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

fn calculate_card_count(matching_cards: &Vec<usize>) -> usize {
    let mut card_counts = vec![1; matching_cards.len()];
    for (i, matches) in matching_cards.iter().enumerate() {
        for j in i + 1..=i + matches {
            if card_counts.get(j).is_none() {
                break;
            }
            card_counts[j] += card_counts[i];
        }
    }

    let p2: usize = card_counts.iter().sum();
    p2
}