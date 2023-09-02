// returns item type priority by getting its ASCII value
fn get_priority(item_type: &char) -> u32 {
    if item_type.is_uppercase() {
        return *item_type as u32 - 38;
    }

    return *item_type as u32 - 96;
}

fn get_compartments(item_list: &str) -> (&str, &str) {
    item_list.split_at(item_list.len() / 2)
}

fn get_common_item((left_comp, right_comp): (&str, &str)) -> char {
    return left_comp
        .split("")
        .filter(|x| right_comp.contains(x))
        .filter(|x| *x != "")
        .collect::<std::collections::HashSet<&str>>()
        .into_iter()
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect::<Vec<char>>()[0];
}

// https://adventofcode.com/2022/day/3#part2
pub fn run() {
    // part_1();
    part_2();
}

fn part_1() {
    // get all lines
    let rucksacks = std::fs::read_to_string("src/day3-input.txt").unwrap();

    // convert to a list of lists where each sublist is a line split in half
    let rucksacks: Vec<&str> = rucksacks.split('\n').filter(|x| *x != "").collect();

    let priority_sums = rucksacks
        .iter()
        .map(|x| get_compartments(x))
        // find the common item type in each half of each sublist in the list
        .map(get_common_item)
        // convert all common item types into their priority
        .map(|x| get_priority(&x))
        .reduce(|acc, next| acc + next);

    println!("{:#?}", priority_sums);
}

fn part_2() {
    println!("Hello Part 2!");
}
