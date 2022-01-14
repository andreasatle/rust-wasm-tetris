import { checkSnapshotValid } from "copy-webpack-plugin";
import init, { Orientation, Shape, World } from "tetris";
import { createUnparsedSourceFile } from "typescript";

// Definition of colors
const LIGHT_BLUE   = '#85C1E9';
const CYAN = '#00FFFF';
const BLUE = '#0000FF';
const ORANGE = '#FFA500';
const YELLOW = '#FFFF00';
const GREEN = '#00FF00';
const PURPLE = '#800080';
const RED = '#FF0000';

const NUM_SHAPES = 7;
// Define colors
const colors =[CYAN,BLUE,ORANGE,YELLOW,GREEN,PURPLE,RED];

// Define size of the game
const CELL_SIZE = 20;
const WORLD_WIDTH = 10;
const WORLD_HEIGHT = 22;

// Setup the canvas from HTML.
const canvas = <HTMLCanvasElement> document.getElementById("tetris-canvas");
const ctx = canvas.getContext("2d");
canvas.width = WORLD_WIDTH*CELL_SIZE;
canvas.height = WORLD_HEIGHT*CELL_SIZE;
init().then(wasm => {
    // Create an instance of the world (the game).
    const world = World.new(WORLD_WIDTH, WORLD_HEIGHT, Shape.I, Orientation.Up);
    // Listen for key-strokes.
    document.addEventListener("keydown", (e) => {
        world.keystroke(e.code);
        drawGame();
    })
    
    //Draw the grid.
    function drawWorld(color: string) {
        ctx.strokeStyle = color;
        ctx.beginPath();
        for (let x = 0; x <= WORLD_WIDTH; x++) {
            ctx.moveTo(CELL_SIZE*x, 0);
            ctx.lineTo(CELL_SIZE*x, canvas.height);
        }
        for (let y = 0; y <= WORLD_HEIGHT; y++) {
            ctx.moveTo(0, CELL_SIZE*y);
            ctx.lineTo(canvas.width, CELL_SIZE*y);
        }
        ctx.stroke();

        const cells = new Uint8Array(wasm.memory.buffer, world.old_cells(), WORLD_WIDTH*WORLD_HEIGHT);
        for (let row = 0; row < WORLD_HEIGHT; row++) {
            let pos = row*WORLD_WIDTH;
            for (let col = 0; col < WORLD_WIDTH; col++) {
                if (cells[pos] < NUM_SHAPES) {
                    ctx.fillStyle = colors[cells[pos]];
                    ctx.beginPath();
                    ctx.fillRect(col*CELL_SIZE, row*CELL_SIZE, CELL_SIZE, CELL_SIZE);
                }
                pos++;
            }
        }
        ctx.stroke();
    }
    // Draw the snake within the grid.
    function drawTetromino() {
        console.log(world.tetromino_shape(), world.tetromino_orientation())
        const shift = world.tetromino_shift();
        const drop = world.tetromino_drop();
        const tetCells = world.tetromino_cells();
        ctx.beginPath();
        let bit = 1;
        for (let i = 0; i < 4; i++) {
            for (let j = 0; j < 4; j++) {
                if ((tetCells & bit) == bit) {
                    ctx.fillRect((shift+j)*CELL_SIZE, (drop+i)*CELL_SIZE, CELL_SIZE, CELL_SIZE);
                }
                bit <<= 1;
            }
        }
        ctx.stroke()
    }

    function drawGame() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawWorld(LIGHT_BLUE);
        drawTetromino();
    }
    
    const gameLoop = () => {
        setTimeout(_ => {
            world.update();
            drawGame();
            gameLoop();
        }, 800)
    }
    gameLoop()
})