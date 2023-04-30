import {Universe, Cell, OrganismType} from "game_of_life";
import { memory } from "game_of_life/game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

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
let paint;

const renderLoop = () => {
    if (!paused) {
        setTimeout(() => {
            drawGrid();
            drawCells();
            universe.tick();
            animationId = requestAnimationFrame(renderLoop);
        }, speedFactor * 200);
    }
};

const playPauseButton = document.getElementById("play-pause");
const organismButton = document.getElementById("organism");
organismButton.textContent = "👾";
const clearButton = document.getElementById("clear");
clearButton.textContent = "❌";
const speed = document.getElementById("speed");
const revertButton = document.getElementById("revert");
revertButton.textContent = "↩";
const organismSelect = document.getElementById("organisms");

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

playPauseButton.addEventListener("click", event => {
    if (paused) {
        play();
    } else {
        pause();
    }
});

clearButton.addEventListener("click", event => {
    universe.clear();
    drawGrid();
    drawCells();
    //pause();
});

revertButton.addEventListener("click", event => {
    universe = Universe.new();
    drawGrid();
    drawCells();
    //pause();
});

speed.addEventListener("input", event => {
    speedFactor = 1 / event.target.value;
});

organismButton.addEventListener("click", event => {
    universe.generate_organism(selectedOrganism, 50, 50)
});

organismSelect.addEventListener("change", event => {
    selectedOrganism = OrganismType[event.target.value];
});

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
    if (paint && row >= 0 && row < height && col >= 0 && col < width) {
        let cell = universe.get_cell(row, col);
        if (cell === Cell.Dead) {
            universe.toggle_cell(row, col);
        }
    }
    drawCells();
    drawGrid();
}

canvas.addEventListener("mousedown", event => {
    paint = true;
    pause();
    paintPixel(event);
});

canvas.addEventListener("mousemove", event => {
    if(paint) {
        paintPixel(event);
    }
});

canvas.addEventListener("mouseup", event => {
    paint = false;
});

canvas.addEventListener("mouseleave", event => {
    paint = false;
});

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
});

play();
