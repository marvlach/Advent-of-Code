import { readLines } from "./utils";

async function processFile(filePath: string = "input.txt") {
  const widths: number[] = [];
  const lines: string[] = [];
  let opers: string[];
  for await (const line of readLines(filePath)) {
    if (line.includes("+")) {
      const parts = line
        .split(/[\+\*]+/)
        .filter((part) => part !== "")
        .map((part) => part.length);
      parts[parts.length - 1] += 1;
      widths.push(...parts);

      opers = line
        .trim()
        .split(" ")
        .filter((part) => part !== "")
        .map((part) => part.trim());
      continue;
    }
    lines.push(line);
  }
  return { widths, lines, opers: opers! };
}

function doAlignedOperation(nums: string[], oper: string, width: number): number {
  let acc = oper === "+" ? 0 : 1;
  for (let pos = width - 1; pos >= 0; pos--) {
    const vals = Number(nums.map((num) => num[pos]).join(""));
    acc = oper === "+" ? acc + vals : acc * vals;
  }
  return acc;
}

async function part1() {
  const { widths, lines, opers } = await processFile("input.txt");
  const results: number[] = [];
  let startIdx = 0;
  const nCols = widths.length;
  const nRows = lines.length;
  for (let col = 0; col < nCols; col++) {
    let acc = opers[col] === "+" ? 0 : 1;
    const width = widths[col];
    for (let row = 0; row < nRows; row++) {
      const valStr = lines[row].slice(startIdx, startIdx + width).trim();
      const val = Number(valStr);
      acc = opers[col] === "+" ? acc + val : acc * val;
    }
    results.push(acc);
    startIdx += width + 1;
  }
  return results.reduce((a, b) => a + b, 0);
}

async function part2() {
  const { widths, lines, opers } = await processFile("input.txt");

  const results: number[] = [];
  let startIdx = 0;
  const nCols = widths.length;
  for (let col = 0; col < nCols; col++) {
    const width = widths[col];
    const operands = lines.map((line) => line.slice(startIdx, startIdx + width));
    const res = doAlignedOperation(operands, opers[col], width);
    results.push(res);
    startIdx += width + 1;
  }
  return results.reduce((a, b) => a + b, 0);
}

part1().then((res) => console.log("Part 1:", res)); // 4648618073226
part2().then((res) => console.log("Part 2:", res)); // 7329921182115
