// import { readFileSync } from 'fs';
// import { join } from 'path';
// import { cwd } from 'process';

import { readFileSync } from 'fs';
import { join } from 'path';
import { cwd } from 'process';

/* 
enum Color {
  Red,
  Green,
  Yellow,
  Blue,
}

function main() {
  // map
  const list = [1, 2, 3].map((e) => e + 1);
  console.log(list);

  // read file
  readFileSync(join(cwd(), 'lines.txt'))
    .toString()
    .split('\n')
    .filter((_, i) => i % 2 === 0)
    .filter((_, i) => i > 1 && i < 4)
    .forEach((l) => console.log(l));

  printColor(0);
}

function printColor(color: Color) {
  switch (color) {
    case Color.Red:
      console.log('red');
      break;
    case Color.Green:
      console.log('green');
      break;
    case Color.Blue:
      console.log('blue');
      break;
    default:
      console.log('Color not found');
  }
}

main();
*/

/*
type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

function append(items: Item[]) {
  items.push('Hello fem!');
}

const items: Item[] = [];

console.log(items);
append(items);
console.log(items);

const numbers: number[] = [];

console.log(numbers);
append(numbers);
console.log(numbers);
*/

/*
function bar(sig: number | undefined): number | undefined {
  return sig === undefined ? undefined : sig * 5;
}


function practice(nums: number[], index: number): number {
  return (nums[index] ?? index) * 5;
}

*/

const path = process.argv[2];

if (!path) {
  throw new Error('please provide the path');
}

readFileSync(join(cwd(), path))
  .toString()
  .split('\n')
  .forEach((line) => {
    let print = parseInt(line);
    if (isNaN(print)) {
      console.log('line is not a number');
    } else {
      console.log(line);
    }
  });
