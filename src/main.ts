import init, { Universe } from "cell-smoother";

const cellsToGrid = (cells: Uint8Array, width: number): string =>
  cells.reduce((acc, cell, i) => {
    let str = `(${cell.toString().padStart(2, "0")}) `;
    if ((i + 1) % width === 0) {
      str += "\n";
    }

    return acc + str;
  }, "");

// must be a function as ESbuild doesn't support top-level `await` (needed in wasm initialization)
const run = async () => {
  // initialize Wasm object
  const { memory } = await init();

  const width = 4;
  const height = 4;
  const universe = Universe.new(width, height);

  const cellsPtr = universe.jscells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  const container = document.getElementById("root");
  if (container) {
    const pre = document.createElement("pre");
    pre.innerText = cellsToGrid(cells, width);

    container.appendChild(pre);
  }
};

run();
