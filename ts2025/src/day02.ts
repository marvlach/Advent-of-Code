import { readLines, mod, div } from "./utils";

type Range = { start: number; end: number };

async function processFile(filePath: string = "input.txt") {
  const ranges: Range[] = [];
  for await (const line of readLines(filePath)) {
    const parts = line.split(",");
    for (const part of parts) {
      const [start, end] = part.split("-").map(Number);
      ranges.push({ start, end });
    }
  }
  return ranges;
}

function isInvalidId1(id: number) {
  const str = id.toString();
  const len = str.length;
  if (len % 2 !== 0) return false;
  const firstHalf = str.slice(0, Math.floor(len / 2));
  const secondHalf = str.slice(Math.floor(len / 2));
  return firstHalf === secondHalf;
}

function splitIntoParts(str: string, splitPos: number): string[] {
  const parts = [];
  let idx = 0;
  while (idx < str.length) {
    parts.push(str.slice(idx, idx + splitPos));
    idx += splitPos;
  }
  return parts;
}

function isInvalidId2(id: number) {   
  const str = id.toString();          
  const len = str.length;
  for (let splitPos = 1; splitPos <= Math.ceil(len / 2); splitPos++) {
    const parts = splitIntoParts(str, splitPos);
    if (parts.length < 2) continue;
    const partSet = new Set(parts);
    const allSame = partSet.size === 1;
    if (allSame) {
      return true;
    }
  }
  return false;
}

function countInvalidInRange(range: Range, isInvalidId: (id: number) => boolean ) {
  let sum = 0;
  for (let id = range.start; id <= range.end; id++) {
    if (isInvalidId(id)) {
      sum += id;
    }
  }
  return sum;
}

async function part1() {
  const ranges = await processFile("input.txt");
  return ranges.reduce((acc, range) => acc + countInvalidInRange(range, isInvalidId1), 0);
}

async function part2() {
  const ranges = await processFile("input.txt");
  return ranges.reduce((acc, range) => acc + countInvalidInRange(range, isInvalidId2), 0);
}

part1().then((res) => console.log("Part 1:", res)); // 21898734247
part2().then((res) => console.log("Part 2:", res)); // 28915664389
