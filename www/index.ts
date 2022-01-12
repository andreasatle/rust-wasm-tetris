import { checkSnapshotValid } from "copy-webpack-plugin";
import init, { Orientation, Shape, World } from "tetris";
import { createUnparsedSourceFile } from "typescript";

// Define colors
const LIGHT_BLUE   = '#85C1E9';
const BLUE         = '#2980B9';
const DARK_BLUE    = '#1F618D';
const GREEN        = '#1ABC9C';
const RED          = '#CB4335';
const ALMOST_WHITE = '#F0F0F0';

// Define size of the game
const CELL_SIZE = 15;
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
        console.log('Key: ', e.code);
        console.log(world.keystroke(e.code));
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
    }
    // Draw the snake within the grid.
    function drawTetromino() {
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

    const gameLoop = () => {
        setTimeout(_ => {
            world.update();
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            drawWorld(GREEN);
            drawTetromino();
            gameLoop();
            //requestAnimationFrame(gameLoop);
        }, 800)
    }
    gameLoop()
})