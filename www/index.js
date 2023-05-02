import {Universe, Cell, OrganismType} from "game_of_life";
import { memory } from "game_of_life/game_of_life_bg";

const CELL_SIZE = 3;
const GRID_COLOR = "#E1E0E7";
const DEAD_COLOR = "#E1E0E7";
const ALIVE_COLOR = "#0D043E";

let universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;
let paused = false;
let speedFactor = 0.166;
let selectedOrganism = OrganismType.Glider;
let mouseDown = false;
let painting = false;
let paintMode = false;

const renderLoop = () => {
    if (!paused) {
        setTimeout(() => {
            drawGrid();
            drawCells();
            universe.tick();
            animationId = requestAnimationFrame(renderLoop);
        }, speedFactor * 100);
    }
};

const playPauseButton = document.getElementById("play-pause");
const organismButton = document.getElementById("organism");
organismButton.textContent = "👾";
const clearButton = document.getElementById("clear");
clearButton.textContent = "❌";
const speed = document.getElementById("speed");
const revertButton = document.getElementById("revert");
revertButton.textContent = "⏪";
const organismSelect = document.getElementById("organisms");
const paintButton = document.getElementById("paint");
const xCoord = document.getElementById("xCoord");
const yCoord = document.getElementById("yCoord");
const stepButton = document.getElementById("step");
stepButton.textContent = "⏭️";

const play = () => {
    paused = false;
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    paused = true;
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

const paintModeOn = () => {
    paintMode = true;
    paintButton.textContent = "◻️";
};

const paintModeOff = () => {
    paintMode = false;
    paintButton.textContent = "🖌️";
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Alive) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Dead) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

function paintPixel(event) {
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
    if (row >= 0 && row < height && col >= 0 && col < width) {
        if (paintMode && painting) {
            pause();
            universe.fill_cell(row, col);
            drawCells();
        } else if (paintMode && !painting) {
            pause();
            universe.toggle_cell(row, col);
            drawCells();
        } else {
            xCoord.value = col;
            yCoord.value = row;
        }
    }
}

playPauseButton.addEventListener("click", event => {
    if (paused) {
        play();
    } else {
        pause();
    }
});

paintButton.addEventListener("click", event => {
    if (paintMode) {
        paintModeOff();
    } else {
        paintModeOn();
    }
});

clearButton.addEventListener("click", event => {
    universe.clear();
    drawGrid();
    drawCells();
});

revertButton.addEventListener("click", event => {
    universe = Universe.new();
    drawGrid();
    drawCells();
});

stepButton.addEventListener("click", event => {
    pause();
    universe.tick();
    drawGrid();
    drawCells();
});

organismButton.addEventListener("click", event => {
    universe.generate_organism(selectedOrganism, parseInt(yCoord.value), parseInt(xCoord.value));
    if (paused) {
        play();
    }
});

xCoord.addEventListener("input", event => {
    if (event.target.value >= 0 && event.target.value < (width - 1)) {
        xCoord.value = event.target.value;
    } else {
        xCoord.value = width - 1;
    }
});

yCoord.addEventListener("input", event => {
    if (event.target.value >= 0 && event.target.value < (height - 1)) {
        yCoord.value = event.target.value;
    } else {
        yCoord.value = height - 1;
    }
});

speed.addEventListener("input", event => {
    speedFactor = 1 / event.target.value;
});

organismSelect.addEventListener("change", event => {
    selectedOrganism = OrganismType[event.target.value];
});

canvas.addEventListener("mousedown", event => {
    mouseDown = true;
    paintPixel(event);
});

canvas.addEventListener("mousemove", event => {
    if(paintMode && mouseDown) {
        painting = true;
        paintPixel(event);
    }
});

canvas.addEventListener("mouseup", event => {
    mouseDown = false;
    painting = false;
});

canvas.addEventListener("mouseleave", event => {
    painting = false;
});

canvas.addEventListener("mouseenter", event => {
    if(paintMode && mouseDown) {
        painting = true;
        paintPixel(event);
    }
});

function setCoordsDefaults() {
    xCoord.value = 50;
    xCoord.min = 0;
    xCoord.max = width - 1;
    yCoord.value = 50;
    yCoord.min = 0;
    yCoord.max = height - 1;
}

function domReady(fn) {
    document.addEventListener("DOMContentLoaded", fn);
    if (document.readyState === "interactive" || document.readyState === "complete" ) {
        fn();
    }
}

domReady(() => {
    for (let organismType in OrganismType) {
        if (isNaN(organismType)) {
            let option = document.createElement("option");
            option.text = organismType;
            option.value = organismType;
            organismSelect.add(option);
        }
    }
    setCoordsDefaults();
});

play();
paintModeOff();