pub fn run() {
    let elves = std::fs::read_to_string("src/day1-input.txt").unwrap();

    let mut elf_calorie_sums: Vec<i32> = elves.split("\n\n").map(sum_elf_calories).collect();

    // order descending
    elf_calorie_sums.sort_by(|a, b| b.cmp(a));

    let max_calories = elf_calorie_sums.iter().max().unwrap();

    let sum_of_top_3: i32 = elf_calorie_sums[0..3]
        .iter()
        .copied()
        .reduce(|a, b| a + b)
        .unwrap_or(0);

    println!("Elf with most calories has {:?} calories", max_calories);
    println!("SUm of top 3 elves' calories {}", sum_of_top_3);
}

fn sum_elf_calories(elf: &str) -> i32 {
    return elf
        .lines()
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .reduce(|acc, next| -> i32 { acc + next })
        .unwrap_or(0);
}
