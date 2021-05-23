import init, { Universe } from "cell-smoother";

const width = 30;
const height = 20;

let memory: WebAssembly.Memory;

const cellsToGrid = (cells: Uint8Array, width: number): HTMLElement => {
  const grid = document.createElement("div");
  grid.className = "grid-column";

  let gridRow = document.createElement("div");
  gridRow.className = "grid-row";

  cells.forEach((cellValue, i) => {
    const cellElement = document.createElement("div");
    // if the cell div is empty there's some ghost padding under each cell/row,
    // but adding some content (e.g. this space and CSS setting `pre`) works...
    cellElement.innerText = " ";
    cellElement.className = `cell cell-${cellValue}`;
    gridRow.appendChild(cellElement);

    if ((i + 1) % width === 0) {
      grid.appendChild(gridRow);
      gridRow = document.createElement("div");
      gridRow.className = "grid-row";
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
    let { universe, cells } = createUniverse();
    drawUniverse(container, cells);

    const evolveUniverse = () => {
      universe.evolve();
      drawUniverse(container, cells);
    };

    const evolveButton = document.getElementById("evolve");
    if (evolveButton) {
      evolveButton.onclick = evolveUniverse;
    }

    const resetButton = document.getElementById("reset");
    if (resetButton) {
      resetButton.onclick = () => {
        ({ universe, cells } = createUniverse());
        drawUniverse(container, cells);
      };
    }
  }
};

run();
