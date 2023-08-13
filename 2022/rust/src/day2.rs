pub fn run() {
    let pairs = std::fs::read_to_string("src/day2-input.txt").unwrap();

    let part_1_pair_sums: i32 = pairs
        .split("\n")
        .filter(|pair| !pair.is_empty())
        .map(get_part1_score)
        .sum();

    println!("{:?}", part_1_pair_sums);

    let part_2_pair_sums: i32 = pairs
        .split("\n")
        .filter(|pair| !pair.is_empty())
        .map(get_part2_score)
        .sum();

    println!("{:?}", part_2_pair_sums)
}

fn get_part1_score(selection: &str) -> i32 {
    match selection {
        "A X" => 4, // rock vs rock         = DRAW = 1 + 3
        "A Y" => 8, // rock vs paper        = WIN  = 2 + 6
        "A Z" => 3, // rock vs scissors     = LOSE = 3 + 0

        "B X" => 1, // paper vs rock        = LOSE = 1 + 0
        "B Y" => 5, // paper vs paper       = DRAW = 2 + 3
        "B Z" => 9, // paper vs scissors    = WIN  = 3 + 6

        "C X" => 7, // scissors vs rock     = WIN  = 1 + 6
        "C Y" => 2, // scissors vs paper    = LOSE = 2 + 0
        "C Z" => 6, // scissors vs scissors = DRAW = 3 + 3

        _ => 0,
    }
}

fn get_part2_score(selection: &str) -> i32 {
    match selection {
        "A X" => 3, // rock + LOSE      = scissors   = 0 + 3
        "A Y" => 4, // rock + DRAW      = rock       = 3 + 1
        "A Z" => 8, // rock + WIN       = paper      = 6 + 2

        "B X" => 1, // paper + LOSE     = rock       = 0 + 1
        "B Y" => 5, // paper + DRAW     = paper      = 3 + 2
        "B Z" => 9, // paper + WIN      = scissors   = 6 + 3

        "C X" => 2, // scissors + LOSE  = paper      = 0 + 2
        "C Y" => 6, // scissors + DRAW  = scissors   = 3 + 3
        "C Z" => 7, // scissors + WIN   = rock       = 6 + 1

        _ => 0,
    }
}
