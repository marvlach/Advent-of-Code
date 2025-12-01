// read file line by line and yield
import fs from "fs";
import readline from "readline";

export async function* readLines(filePath: string): AsyncGenerator<string> {
  const fileStream = fs.createReadStream(filePath);

  const rl = readline.createInterface({
    input: fileStream,
    crlfDelay: Infinity,
  });
  for await (const line of rl) {
    yield line;
  }
}

export function mod(n: number, m: number) {
  return ((n % m) + m) % m;
}

export function div(n: number, m: number) {
  return Math.floor(n / m);
}
