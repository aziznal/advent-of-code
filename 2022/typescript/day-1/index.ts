import * as fs from "fs";

const elves = fs
  .readFileSync("day-1/input.txt", "utf8")
  .split("\n\n")
  .filter((elf) => elf.length > 0); // skip empty lines

const sumCalories = (elf: string): number => {
  return elf
    .split("\n")
    .map((num) => parseInt(num))
    .reduce((acc, next) => acc + next);
};

const elfCalories: number[] = elves.map(sumCalories);

// Part 1 solution
const mostCals = Math.max(...elfCalories);
console.log(`Elf with the most calories has ${mostCals} calories`);

// Part 2 solution
const sortedCals = elfCalories.sort((a, b) => b - a);

const nextElvesSum = sortedCals[0] + sortedCals[1] + sortedCals[2];

console.log(nextElvesSum);
