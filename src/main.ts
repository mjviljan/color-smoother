import init, { Universe } from "cell-smoother";

const width = 30;
const height = 20;

let memory: WebAssembly.Memory;

const cellsToGrid = (cells: Uint8Array, width: number): HTMLElement => {
  const grid = document.createElement("div");

  cells.forEach((cellValue, i) => {
    const cellElement = document.createElement("span");
    cellElement.innerText = "▉";
    cellElement.setAttribute(
      "style",
      "color: rgb(0, " + cellValue * 16 + ", 0)"
    );
    grid.appendChild(cellElement);

    if ((i + 1) % width === 0) {
      grid.appendChild(document.createElement("br"));
    }
  });

  return grid;
};

const createUniverse = () => {
  const universe = Universe.new(width, height);

  const cellsPtr = universe.cells_ptr();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  return {
    universe,
    cells,
  };
};

const drawUniverse = (elem: HTMLElement, cells: Uint8Array) => {
  const universeElem = elem.getElementsByTagName("div");
  universeElem.item(0)?.remove();

  elem.appendChild(cellsToGrid(cells, width));
};

// must be a function as ESbuild doesn't support top-level `await` (needed in wasm initialization)
const run = async () => {
  // initialize Wasm object
  ({ memory } = await init());

  const container = document.getElementById("cells");
  if (container) {
    const pre = document.createElement("pre");
    let { universe, cells } = createUniverse();

    drawUniverse(pre, cells);

    container.appendChild(pre);

    const evolveUniverse = () => {
      universe.evolve();
      drawUniverse(pre, cells);
    };

    const evolveButton = document.getElementById("evolve");
    if (evolveButton) {
      evolveButton.onclick = evolveUniverse;
    }

    const resetButton = document.getElementById("reset");
    if (resetButton) {
      resetButton.onclick = () => {
        ({ universe, cells } = createUniverse());
        drawUniverse(pre, cells);
      };
    }
  }
};

run();
