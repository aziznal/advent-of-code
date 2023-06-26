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

console.log(Math.max(...elves.map(sumCalories)));
