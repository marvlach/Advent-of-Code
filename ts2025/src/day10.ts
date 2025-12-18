import { readLines } from "./utils";

type Machine = { indicator: string; buttons: number[][]; joltage: string };
type State = { curr: string; depth: number };

async function processFile(filePath: string = "input.txt") {
  const machines: Machine[] = [];
  for await (const line of readLines(filePath)) {
    const [indicator, ...rest] = line.trim().split(" ");
    const buttons = [];
    for (let i = 0; i < rest.length - 1; i++) {
      buttons.push(
        rest[i]
          .slice(1, rest[i].length - 1)
          .split(",")
          .map(Number)
      );
    }
    const joltage = rest[rest.length - 1]
      .slice(1, rest[rest.length - 1].length - 1)
      .split(",")
      .join("_");
    machines.push({
      indicator: indicator
        .slice(1, indicator.length - 1)
        .replace(/\./g, "0")
        .replace(/#/g, "1")
        .split("")
        .join("_"),
      buttons,
      joltage,
    });
  }
  return machines;
}

function stateArrToStr(arr: number[]) {
  return arr.join("_");
}

function stateStrToArr(str: string) {
  return str.split("_").map(Number);
}

function pressButton(state: number[], button: number[], onChange: (n: number) => number) {
  const newState = state.slice();
  button.forEach((ind) => {
    newState[ind] = onChange(newState[ind]);
  });
  return newState;
}

function minButtons(
  machine: Machine,
  onChange: (n: number) => number,
  shouldTerminate: (str: string, machine: Machine) => boolean,
  shouldContinue: (str: string, machine: Machine) => boolean
) {
  console.log(machine);
  const initStateStr = machine.indicator
    .split("_")
    .map(() => "0")
    .join("_");
  const queue: State[] = [{ curr: initStateStr, depth: 0 }];
  const visited: Set<string> = new Set([initStateStr]);

  while (queue.length) {
    const { curr: currState, depth: currDepth } = queue.shift()!;
    const currArr = stateStrToArr(currState);
    const currStr = currState;
    // console.log("State", currStr, "Array", currArr, "Depth", currDepth);

    if (shouldContinue(currStr, machine)) {
      continue;
    }
    if (shouldTerminate(currStr, machine)) {
      console.log(currDepth);
      return currDepth;
    }
    const nextStateStr = machine.buttons
      .map((button) => pressButton(currArr, button, onChange))
      .map((arr) => stateArrToStr(arr));
    nextStateStr.forEach((str) => {
      if (!visited.has(str)) {
        // console.log("   Adding", { curr: str, depth: currDepth + 1 })
        visited.add(str);
        queue.push({ curr: str, depth: currDepth + 1 });
      }
    });
  }
  console.log(-1);
  return -1;
}

async function part1() {
  const machines = await processFile("input.txt");
  const res = machines.reduce(
    (acc, m) =>
      acc +
      minButtons(
        m,
        (n) => Number(!n),
        (str, machine) => str === machine.indicator,
        (str, machine) => false
      ),
    0
  );
  return res;
}

async function part2() {
  const machines = await processFile("input.txt");
  const res = machines.reduce(
    (acc, m) =>
      acc +
      minButtons(
        m,
        (n) => n + 1,
        (str, machine) => str === machine.joltage,
        (str, machine) => {
          const counters = str.split("_").map(Number);
          const targets = machine.joltage.split("_").map(Number);
          const shouldSkip = counters.some((_, index) => counters[index] > targets[index]);
          return shouldSkip && str !== machine.joltage;
        }
      ),
    0
  );
  return res;
}

part1().then((res) => console.log("Part 1:", res)); // 491
part2().then((res) => console.log("Part 2:", res)); // will never run
