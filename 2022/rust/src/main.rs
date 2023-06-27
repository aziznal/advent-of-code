fn main() {
    day1();
}

fn day1() {
    let elves = std::fs::read_to_string("src/day1-input.txt").unwrap();

    let elf_calorie_sums = elves.split("\n\n");

    return;
}

fn day1_sum_elf_calories(elf: String) -> u32 {
    let to_int = |string: String| -> i32 { string.parse().unwrap() };

    // elf.lines().reduce(|acc, next| to_int(acc) + to_int(next));

    return 1;
}

