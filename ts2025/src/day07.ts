import { readLines } from "./utils";

type Position = { row: number; col: number };

async function processFile(filePath: string = "input.txt") {
  const splitters: Map<string, number> = new Map();
  let start: Position = { row: -1, col: -1 };
  let row = 0;
  let nCols = 0;
  let nRows = 0;
  for await (const line of readLines(filePath)) {
    for (let col = 0; col < line.length; col++) {
      const char = line[col];
      if (char === "S") {
        start = { row, col };
      } else if (char === "^") {
        splitters.set(`${row}_${col}`, 0);
      }
    }
    nCols = line.length;
    row++;
  }
  nRows = row;
  return { splitters, start, nRows, nCols };
}

function propagateSignal(start: Position, splitters: Map<string, number>, nRows: number, nCols: number) {
  const visited: Set<string> = new Set();
  const queue: Position[] = [start];
  let countSplits = 0;
  while (queue.length > 0) {
    const current = queue.shift()!;
    const posKey = `${current.row}_${current.col}`;
    if (visited.has(posKey)) {
      continue;
    }
    if (current.row >= nRows || current.col < 0 || current.col >= nCols) {
      continue;
    }
    if (splitters.has(posKey)) {
      const leftPos = { row: current.row, col: current.col - 1 };
      const rightPos = { row: current.row, col: current.col + 1 };
      queue.push(leftPos);
      queue.push(rightPos);
      countSplits++;
    } else {
      queue.push({ row: current.row + 1, col: current.col });
      visited.add(posKey);
    }
  }
  return { visited, countSplits};
}

function countPossiblePaths(current: Position, splitters: Map<string, number>, nRows: number, nCols: number, memo: Map<string, number>): number {
  const posKey = `${current.row}_${current.col}`;
  if (memo.has(posKey)) {
    return memo.get(posKey)!;
  }
  if (current.row >= nRows || current.col < 0 || current.col >= nCols) {
    return 0;
  }
  if (current.row === nRows - 1) {
    return 1;
  }
  let totalPaths = 0;
  if (splitters.has(posKey)) {
    const leftPos = { row: current.row, col: current.col - 1 };
    const rightPos = { row: current.row, col: current.col + 1 };
    totalPaths += countPossiblePaths(leftPos, splitters, nRows, nCols, memo);
    totalPaths += countPossiblePaths(rightPos, splitters, nRows, nCols, memo);
  } else {
    const downPos = { row: current.row + 1, col: current.col };
    totalPaths += countPossiblePaths(downPos, splitters, nRows, nCols, memo);
  }
  memo.set(posKey, totalPaths);
  return totalPaths;
}

async function part1() {
  const { splitters, start, nRows, nCols } = await processFile("input.txt");
  const { countSplits } = propagateSignal(start, splitters, nRows, nCols);
  return countSplits;
}

async function part2() {
  const { splitters, start, nRows, nCols } = await processFile("input.txt");
  const res = countPossiblePaths(start, splitters, nRows, nCols, new Map());
  return res;
}

part1().then((res) => console.log("Part 1:", res)); // 1566
part2().then((res) => console.log("Part 2:", res)); // 5921061943075
