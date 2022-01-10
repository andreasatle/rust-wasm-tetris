use wasm_bindgen::prelude::*;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/utils.js")]
extern "C" {
    fn output_js(msg: String);
}
#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, from Rust!".to_string()
}

type TetrisCell = usize;
type TetrisCells = [TetrisCell; 4];

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Shape {
    I, O, J, L, S, Z, T,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Tetromino {
    shape: Shape,
    orientation: Orientation,
    position: usize,
}

#[wasm_bindgen]
impl Tetromino {
    pub fn new(shape: Shape, orientation: Orientation, position: TetrisCell) -> Tetromino {
        Tetromino{
            shape,
            orientation,
            position,
        }
    }

    pub fn rotate_right(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    pub fn rotate_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        }
    }


}

#[wasm_bindgen]
pub struct World {
    width : usize,
    height: usize,
    tetromino: Tetromino,
    tet_cells: TetrisCells,
}

#[wasm_bindgen]
impl World {

    pub fn new(width: usize, height: usize) -> World {
        let mut world = World {
            width,
            height,
            tetromino: Tetromino{
                shape: Shape::O,
                orientation: Orientation::Up,
                position: 5*height + width/2,
            },
            tet_cells: [0;4],
        };
        world.update_tet_cells();
        world
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn keystroke(&mut self, key: &str) {
        match key {
            "ArrowLeft" | "Numpad4" => self.shift_left(),
            "ArrowRight" | "Numpad6" => self.shift_right(),
            "ArrowUp" | "KeyX" | "Numpad1" | "Numpad5" | "Numpad9" => self.rotate_right(),
            "ControlLeft" | "ControlRight" | "KeyZ" | "Numpad3" | "Numpad7" => self.rotate_left(),
            "ArrowDown" | "Numpad2" => self.soft_drop(),

            // "ArrowLeft" | "Numpad4" => "Left Shift".to_string(),
            // "ArrowRight" | "Numpad6" => "Right Shift".to_string(),
            // "ArrowUp" | "KeyX" | "Numpad1" | "Numpad5" | "Numpad9" => "Rotate Right".to_string(),
            // "ControlLeft" | "ControlRight" | "KeyZ" | "Numpad3" | "Numpad7" => "Rotate Left".to_string(),
            // "ArrowDown" | "Numpad2" => "Soft Drop".to_string(),

            // "Space" | "Numpad8" => "Hard Drop".to_string(),
            // "ShiftLeft" | "ShiftRight" | "KeyC" => "Hold".to_string(),
            // "Escape" | "Numpad0" => "Pause".to_string(),

            _ => {},//key.to_string()
        }
    }
    
    pub fn update_tet_cells(&mut self) {
        let pos = self.tetromino.position;
        let row = self.width();
        match (self.tetromino.shape, self.tetromino.orientation) {
            (Shape::I, Orientation::Up) => self.tet_cells = [pos-2,pos-1,pos,pos+1],
            (Shape::I, Orientation::Right) => self.tet_cells = [pos-2*row,pos-row,pos,pos+row],
            (Shape::I, Orientation::Down) => self.tet_cells = [pos-1,pos,pos+1,pos+2],
            (Shape::I, Orientation::Left) => self.tet_cells = [pos-row,pos,pos+row,pos+2*row],
            (Shape::J, Orientation::Up) => self.tet_cells = [pos-1-row,pos-1,pos,pos+1],
            (Shape::J, Orientation::Right) => self.tet_cells = [pos+row,pos,pos-row,pos-row+1],
            (Shape::J, Orientation::Down) => self.tet_cells = [pos-1,pos,pos+1,pos+row+1],
            (Shape::J, Orientation::Left) => self.tet_cells = [pos+row-1,pos+row,pos,pos-row],
            (Shape::L, Orientation::Up) => self.tet_cells = [pos-1,pos,pos+1,pos-row+1],
            (Shape::L, Orientation::Right) => self.tet_cells = [pos-row,pos,pos+row,pos+row+1],
            (Shape::L, Orientation::Down) => self.tet_cells = [pos+row-1,pos-1,pos,pos+1],
            (Shape::L, Orientation::Left) => self.tet_cells = [pos-row-1,pos-row,pos,pos+row],
            (Shape::O, _) => self.tet_cells = [pos,pos+row,pos+row-1,pos-1],
            _ => {},
        }
    }

    /// Return the pointer to the tetromino.
    pub fn tet_cells(&self) -> *const TetrisCell {
        self.tet_cells.as_ptr()
    }

    pub fn update(&mut self) {
        self.update_tet_cells();
    }

    pub fn shift_left(&mut self) {
        // Check that shift is valid 2DO 2DO
        self.tetromino.position -= 1;
    }

    pub fn shift_right(&mut self) {
        // Check that shift is valid 2DO 2DO
        self.tetromino.position += 1;
    }

    pub fn rotate_left(&mut self) {
        // Check that rotate is valid 2DO 2DO
        self.tetromino.rotate_left();
    }

    pub fn rotate_right(&mut self) {
        // Check that rotate is valid 2DO 2DO
        self.tetromino.rotate_right();
    }

    pub fn soft_drop(&mut self) {
        self.tetromino.position += self.width()
    }

    pub fn row(&self, idx: usize) -> usize {
        idx / self.width
    }

    pub fn col(&self, idx: usize) -> usize {
        idx % self.width
    }
}
