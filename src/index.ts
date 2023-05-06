import { readFileSync } from 'fs';
import { join } from 'path';
import { cwd } from 'process';

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
