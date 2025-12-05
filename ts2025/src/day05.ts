import { readLines } from "./utils";

type Range = { start: number; end: number };

async function processFile(filePath: string = "input.txt") {
  const ranges: Range[] = [];
  const ingr: number[] = [];
  let readingRanges = true;
  for await (const line of readLines(filePath)) {
    if (line.trim() === "") {
      readingRanges = false;
      continue;
    }
    if (readingRanges) {
      const [start, end] = line.split("-").map(Number);
      ranges.push({ start, end });
    } else {
      ingr.push(Number(line.trim()));
    }
  }
  return { ranges, ingr };
}

function ingrIsInRange(ingr: number, range: Range): boolean {
  return ingr >= range.start && ingr <= range.end;
}

function partiallyExtendsLeft(range: Range, other: Range): boolean {
  return range.start < other.start && range.end >= other.start && range.end <= other.end;
}

function partiallyExtendsRight(range: Range, other: Range): boolean {
  return range.start >= other.start && range.start <= other.end && range.end > other.end;
}

function isFullyContained(range: Range, other: Range): boolean {
  return range.start >= other.start && range.end <= other.end;
}

function fullyContains(range: Range, other: Range): boolean {
  return range.start <= other.start && range.end >= other.end;
}

function isEqual(range: Range, other: Range): boolean {
  return range.start === other.start && range.end === other.end;
}

function isInvalidRange(range: Range): boolean {
  return range.end < range.start;
}

function mergeRangeWithOverlapingRanges(range: Range, overlap: Range[]): Range[] {
  if (isInvalidRange(range)) {
    return [];
  }

  for (let i = 0; i < overlap.length; i++) {
    const currentOverlap = overlap[i];

    if (isEqual(range, currentOverlap)) {
      return [];
    }

    if (partiallyExtendsLeft(range, currentOverlap)) {
      const leftPart = { start: range.start, end: currentOverlap.start - 1 };
      return [leftPart];
    }

    if (partiallyExtendsRight(range, currentOverlap)) {
      const rightPart = { start: currentOverlap.end + 1, end: range.end };
      return [rightPart];
    }

    if (isFullyContained(range, currentOverlap)) {
      return [];
    }

    if (fullyContains(range, currentOverlap)) {
      const leftPart = { start: range.start, end: currentOverlap.start - 1 };
      const rightPart = { start: currentOverlap.end + 1, end: range.end };
      return [leftPart, rightPart];
    }
  }

  overlap.push(range);
  return [];
}

function mergeRanges(ranges: Range[]): Range[] {
  const rangeQueue = [...ranges.slice(1)];
  const overlap = [ranges[0]];

  while (rangeQueue.length > 0) {
    const currentRange = rangeQueue.pop()!;
    const leftoverRanges = mergeRangeWithOverlapingRanges(currentRange, overlap);
    rangeQueue.push(...leftoverRanges);
  }
  return overlap;
}

async function part1() {
  const { ranges, ingr } = await processFile("input.txt");
  return ingr.filter((i) => ranges.some((r) => ingrIsInRange(i, r))).length;
}

async function part2() {
  const { ranges } = await processFile("input.txt");
  const mergedRanges = mergeRanges(ranges);
  return mergedRanges.reduce((acc, range) => acc + (range.end - range.start + 1), 0);
}

part1().then((res) => console.log("Part 1:", res)); // 509
part2().then((res) => console.log("Part 2:", res)); // 336790092076620
