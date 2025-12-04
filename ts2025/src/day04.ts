import { readLines } from "./utils";

async function processFile(filePath: string = "input.txt") {
  const rolls: Set<string> = new Set();
  let i = 0;
  let nCols = 0;
  for await (const line of readLines(filePath)) {
    const cols = line.split("");
    for (let j = 0; j < cols.length; j++) {
      const key = `${i}_${j}`;
      if (cols[j] === "@") {
        rolls.add(key);
      }
    }
    nCols = cols.length;
    i += 1;
  }

  return { nRows: i, nCols, rolls };
}

function getNeighbors(row: number, col: number, nRows: number, nCols: number): string[] {
  const neighbors: string[] = [];
  for (let dx = -1; dx <= 1; dx++) {
    for (let dy = -1; dy <= 1; dy++) {
      if (dx === 0 && dy === 0) {
        continue;
      }
      const newRow = row + dx;
      const newCol = col + dy;
      if (newRow >= 0 && newRow < nRows && newCol >= 0 && newCol < nCols) {
        neighbors.push(`${newRow}_${newCol}`);
      }
    }
  }
  return neighbors;
}

function countRollNeighbours(rolls: Set<string>, nRows: number, nCols: number) {
  let count = 0;
  const toBeRemoved: string[] = [];
  for (const key of rolls.keys()) {
    const [row, col] = key.split("_").map(Number);
    const neighbors = getNeighbors(row, col, nRows, nCols);
    const numOfRollNeighbors = neighbors.reduce((acc, nKey) => (rolls.has(nKey) ? acc + 1 : acc), 0);
    if (numOfRollNeighbors < 4) {
      toBeRemoved.push(key);
      count += 1;
    }
  }
  return { count, toBeRemoved };
}

function keepRemoving(rolls: Set<string>, nRows: number, nCols: number) {
  let total = 0;
  while (true) {
    const { count, toBeRemoved } = countRollNeighbours(rolls, nRows, nCols);
    if (count === 0) {
      break;
    }
    total += count;
    for (const key of toBeRemoved) {
      rolls.delete(key);
    }
  }
  return total;
}

async function part1() {
  const { nRows, nCols, rolls } = await processFile("input.txt");
  const { count } = countRollNeighbours(rolls, nRows, nCols);
  return count;
}

async function part2() {
  const { nRows, nCols, rolls } = await processFile("input.txt");
  const totalRemoved = keepRemoving(rolls, nRows, nCols);
  return totalRemoved;
}

part1().then((res) => console.log("Part 1:", res)); // 1478
part2().then((res) => console.log("Part 2:", res)); // 9120
