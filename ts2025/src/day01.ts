import { readLines, mod, div } from "./utils";

type Rotation = { dir: "L" | "R"; tick: number };

async function processFile(filePath: string = "input.txt") {
  const rots: Rotation[] = [];
  for await (const line of readLines(filePath)) {
    const dir = line[0] as "L" | "R";
    const tick = Number(line.slice(1));
    rots.push({ dir, tick });
  }
  return rots;
}

async function part1() {
  const rots = await processFile("input.txt");
  let curr = 50;
  let count0 = 0;

  for (const rot of rots) {
    const mult = rot.dir === "L" ? -1 : 1;
    curr = mod(curr + mult * rot.tick, 100);
    if (curr === 0) count0++;
  }
  return count0;
}

async function part2() {
  const rots = await processFile("input.txt");
  let curr = 50;
  let count0 = 0;
  for (const rot of rots) {
    const mult = rot.dir === "L" ? -1 : 1;
    const next = mod(curr + mult * rot.tick, 100);
    if (rot.dir === "R") {
      count0 += div(curr + mult * rot.tick, 100);
    }
    if (rot.dir === "L") {
      let t0 = curr === 0 ? 100 : curr;
      if (rot.tick >= t0) {
        count0 += 1 + div(rot.tick - t0, 100);
      }
    }
    curr = next;
  }
  return count0;
}

part1().then((res) => console.log("Part 1:", res)); // 1071
part2().then((res) => console.log("Part 2:", res)); // 6700
