use std::str::{FromStr, Lines};

fn main() {
    let input = std::fs::read_to_string("input/day05.txt").unwrap();

    let sum = calculate_part1(&input);
    println!("Part 1 answer: {sum}");
}

fn calculate_part1(input: &str) -> Id {
    let mut line_iter = input.lines();
    let seeds = get_numbers_in_seed_line(&mut line_iter);
    let mappings = get_mappings(&mut line_iter);
    get_min_location_id(seeds.iter().copied(), &mappings)
}

fn get_min_location_id(seed_iter: impl Iterator<Item = Id>, mappings: &[Mapping]) -> Id {
    seed_iter
        .map(|seed| mappings
            .iter()
            .fold(seed, |id, mapping| mapping.map(id)))
        .min()
        .expect("no seed -> location")
}

fn get_numbers_in_seed_line(line_iter: &mut Lines) -> Vec<Id> {
    let seed_line = line_iter.next().unwrap();
    seed_line[7..]
        .split(' ')
        .map(|seed_str| Id::from_str(seed_str).expect("not a number"))
        .collect::<Vec<Id>>()
}

fn get_mappings(line_iter: &mut Lines) -> Vec<Mapping> {
    let mut mappings = Vec::new();
    for ln in line_iter {
        if ln.is_empty() {
            continue;
        } else if ln.contains("map:") {
            mappings.push(Mapping {
                mapping_rules: Vec::new(),
            });
        } else {
            let rule_params: Vec<Id> = ln
                .split(' ')
                .map(|id_str| Id::from_str(id_str).expect(""))
                .collect();

            mappings
                .last_mut()
                .expect("")
                .mapping_rules
                .push(MappingRule {
                    destination_id: rule_params[0],
                    source_id: rule_params[1],
                    count: rule_params[2],
                });
        }
    }
    mappings
}

// Data Structures / Classes: 
type Id = u64;

struct MappingRule {
    destination_id: Id,
    source_id: Id,
    count: Id,
}

impl MappingRule { // validate if Id meets the conditions 
    fn validate(&self, input: Id) -> Option<Id> { // return Id or null/None
        if self.source_id <= input && 
           self.source_id + self.count > input 
        {
            Some(input + self.destination_id - self.source_id)
        } else {
            None
        }
    }
}

struct Mapping { // list of rules
    mapping_rules: Vec<MappingRule>,
}

impl Mapping { // filter list - get right id output for given id
    fn map(&self, input: Id) -> Id {
        self.mapping_rules
            .iter()
            .filter_map(|rule| rule.validate(input)) // use validation rules
            .next()
            .unwrap_or(input)
    }
}