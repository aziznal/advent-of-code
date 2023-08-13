import * as fs from "fs";

const pairs = fs
  .readFileSync("day-2/input.txt", "utf8")
  .split("\n")
  .filter((elf) => elf !== ""); // skip empty lines

console.log(pairs.map(getPairScore1).reduce((acc, next) => acc + next));
console.log(pairs.map(getPairScore2).reduce((acc, next) => acc + next));

// part 1 solution
function getPairScore1(pair: string): number {
  switch (pair) {
    case "A X": return 4;
    case "A Y": return 8;
    case "A Z": return 3;

    case "B X": return 1;
    case "B Y": return 5;
    case "B Z": return 9;

    case "C X": return 7;
    case "C Y": return 2;
    case "C Z": return 6;

    default: return 0;
  }
}

function getPairScore2(pair: string): number {
  switch (pair) {
    case "A X": return 3;
    case "A Y": return 4;
    case "A Z": return 8;
                        
    case "B X": return 1;
    case "B Y": return 5;
    case "B Z": return 9;
                        
    case "C X": return 2;
    case "C Y": return 6;
    case "C Z": return 7;

    default: return 0;
  }
}
