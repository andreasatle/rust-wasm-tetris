import init, { greet } from "tetris";

init().then(_ => {
    console.log('Hello, World from typescript!');
    console.log(greet());
})