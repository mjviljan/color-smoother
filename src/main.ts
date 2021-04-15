import init, { Universe } from "cell-smoother";

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

    // let str = `(${cell.toString().padStart(2, "0")}) `;
    if ((i + 1) % width === 0) {
      grid.appendChild(document.createElement("br"));
    }
  });

  return grid;
};

// must be a function as ESbuild doesn't support top-level `await` (needed in wasm initialization)
const run = async () => {
  // initialize Wasm object
  const { memory } = await init();

  const width = 30;
  const height = 20;
  const universe = Universe.new(width, height);

  let cellsPtr = universe.cells_ptr();
  let cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  const drawUniverse = (elem: HTMLElement) => {
    const universeElem = elem.getElementsByTagName("div");
    universeElem.item(0)?.remove();

    elem.appendChild(cellsToGrid(cells, width));
  };

  const container = document.getElementById("root");
  if (container) {
    const pre = document.createElement("pre");
    drawUniverse(pre);

    container.appendChild(pre);

    const evolveUniverse = () => {
      universe.evolve();
      drawUniverse(pre);
    };

    const evolveButton = document.getElementById("evolve");
    if (evolveButton) {
      evolveButton.onclick = evolveUniverse;
    }
  }
};

run();
