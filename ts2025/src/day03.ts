import { readLines } from "./utils";

async function processFile(filePath: string = "input.txt") {
  const banks: number[][] = [];
  for await (const line of readLines(filePath)) {
    const bank = line.split("").map(Number);
    banks.push(bank);
  }
  return banks;
}

function maxBank(bank: number[], slots: number): number {
  const m: number[] = [];
  const mi = [];
  let start = 0;
  let end = bank.length - (slots - 1);
  for (let s = 0; s < slots; s++) {
    let max = 0;
    let maxi = -1;
    for (let i = start; i < end; i++) {
      if (bank[i] > max) {
        max = bank[i];
        maxi = i;
      }
    }
    m.push(max);
    mi.push(maxi);
    start = maxi + 1;
    end += 1;
  }
  return Number(m.join(""));
}

async function part1() {
  const banks = await processFile("input.txt");
  const slots = 2;
  return banks.reduce((acc, bank) => acc + maxBank(bank, slots), 0); // 17383
}

async function part2() {
  const banks = await processFile("input.txt");
  const slots = 12;
  return banks.reduce((acc, bank) => acc + maxBank(bank, slots), 0); // 172601598658203
}

part1().then((res) => console.log("Part 1:", res));
part2().then((res) => console.log("Part 2:", res));
