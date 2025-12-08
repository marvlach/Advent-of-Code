import { readLines } from "./utils";

type Position = { x: number; y: number; z: number };
type JBox = { id: string; pos: Position };
type Distance = { key: string; dist: number };

async function processFile(filePath: string = "input.txt") {
  let row = 0;
  const jboxes: JBox[] = [];
  for await (const line of readLines(filePath)) {
    const [x, y, z] = line.trim().split(",").map(Number);
    const jbox = { id: row.toString(), pos: { x, y, z } };
    jboxes.push(jbox);
    row++;
  }
  return { jboxes };
}

function distance(a: Position, b: Position): number {
  return (a.x - b.x) ** 2 + (a.y - b.y) ** 2 + (a.z - b.z) ** 2;
}

function allDistances(jboxes: JBox[]): Distance[] {
  const distances: Distance[] = [];
  for (let i = 0; i < jboxes.length; i++) {
    for (let j = i + 1; j < jboxes.length; j++) {
      const dist = distance(jboxes[i].pos, jboxes[j].pos);
      distances.push({ key: `${jboxes[i].id}_${jboxes[j].id}`, dist });
    }
  }
  return distances.sort((a, b) => a.dist - b.dist);
}

function connect(jboxes: JBox[], sortedDistances: Distance[], totalConnections: number, part: 1 | 2) {
  const circuits = jboxes.reduce((acc, jbox, i) => {
    acc[jbox.id] = { circuitId: i.toString(), jboxesInCircuit: [jbox.id] };
    return acc;
  }, {} as { [key: string]: { circuitId: string; jboxesInCircuit: string[] } });

  let nConnected = 0;
  let distIndex = 0;
  while (true) {
    if (distIndex >= totalConnections && part === 1) {
      break;
    }
    if (new Set(Object.values(circuits).map((c) => c.circuitId)).size === 1 && part === 2) {
      break;
    }

    const distEntry = sortedDistances[distIndex];
    const [idA, idB] = distEntry.key.split("_");
    const { circuitId: circuitIdA, jboxesInCircuit: jboxesInCircuitA } = circuits[idA];
    const { circuitId: circuitIdB, jboxesInCircuit: jboxesInCircuitB } = circuits[idB];
    if (circuitIdA !== circuitIdB) {
      const minCircuitId = Math.min(Number(circuitIdA), Number(circuitIdB)).toString();
      const jboxesInCircuit = [...jboxesInCircuitA, ...jboxesInCircuitB];
      circuits[idA] = { circuitId: minCircuitId, jboxesInCircuit };
      circuits[idB] = { circuitId: minCircuitId, jboxesInCircuit };
      for (const jboxId of jboxesInCircuitA) {
        circuits[jboxId] = { circuitId: minCircuitId, jboxesInCircuit };
      }
      for (const jboxId of jboxesInCircuitB) {
        circuits[jboxId] = { circuitId: minCircuitId, jboxesInCircuit };
      }
      nConnected++;
    }
    distIndex++;
  }
  return {
    circuits: Object.fromEntries(Object.entries(circuits).map(([jboxId, { circuitId }]) => [jboxId, circuitId])),
    lastDistIndex: distIndex - 1,
  };
}

function getCircuitMembers(circuits: { [key: string]: string }) {
  const circuitMembers = new Map<string, string[]>();
  for (const [jBoxId, circuitId] of Object.entries(circuits)) {
    if (!circuitMembers.has(circuitId)) {
      circuitMembers.set(circuitId, [jBoxId]);
    } else {
      circuitMembers.get(circuitId)!.push(jBoxId);
    }
  }
  return circuitMembers;
}

async function part1() {
  const { jboxes } = await processFile("input.txt");
  const sortedDistances = allDistances(jboxes);
  const { circuits } = connect(jboxes, sortedDistances, 1000, 1);
  const circuitMembers = getCircuitMembers(circuits);
  const sorted = Array.from(circuitMembers.values())
    .map((members) => members.length)
    .sort((a, b) => b - a);
  const [a, b, c] = sorted;
  return a * b * c;
}

async function part2() {
  const { jboxes } = await processFile("input.txt");
  const sortedDistances = allDistances(jboxes);
  const { lastDistIndex } = connect(jboxes, sortedDistances, 1000, 2);
  const [a, b] = sortedDistances[lastDistIndex].key.split("_");
  const jboxA = jboxes[Number(a)];
  const jboxB = jboxes[Number(b)];
  return jboxA.pos.x * jboxB.pos.x;
}

part1().then((res) => console.log("Part 1:", res)); // 42315
part2().then((res) => console.log("Part 2:", res)); // 8079278220
