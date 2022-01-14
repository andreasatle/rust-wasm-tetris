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
pub enum GameStatus {
    StartNewGame,
    StartNewTetromino,    
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Shape {
    I, J, L, O, S, T, Z,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
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
    cells: u32,
}

#[wasm_bindgen]
impl Tetromino {
    pub fn new(shape: Shape, orientation: Orientation) -> Tetromino {
        Tetromino {
            shape,
            orientation,
            cells: 0,
        }
    }
    pub fn shape(&self) -> Shape {
        self.shape
    }
    pub fn orientation(&self) -> Orientation {
        self.orientation
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
    
    pub fn update(&mut self) {
        self.cells = match (self.shape, self.orientation) {
            (Shape::I, Orientation::Up) => (1<<4) + (1<<5) + (1<<6) + (1<<7),
            (Shape::I, Orientation::Right) => (1<<2) + (1<<6) + (1<<10) + (1<<14),
            (Shape::I, Orientation::Down) => (1<<8) + (1<<9) + (1<<10) + (1<<11),
            (Shape::I, Orientation::Left) => (1<<1) + (1<<5) + (1<<9) + (1<<13),
            (Shape::J, Orientation::Up) => (1<<0) + (1<<4) + (1<<5) + (1<<6),
            (Shape::J, Orientation::Right) => (1<<1) + (1<<2) + (1<<5) + (1<<9),
            (Shape::J, Orientation::Down) => (1<<4) + (1<<5) + (1<<6) + (1<<10),
            (Shape::J, Orientation::Left) => (1<<1) + (1<<5) + (1<<8) + (1<<9),
            (Shape::L, Orientation::Up) => (1<<2) + (1<<4) + (1<<5) + (1<<6),
            (Shape::L, Orientation::Right) => (1<<1) + (1<<5) + (1<<9) + (1<<10),
            (Shape::L, Orientation::Down) => (1<<4) + (1<<5) + (1<<6) + (1<<8),
            (Shape::L, Orientation::Left) => (1<<0) + (1<<1) + (1<<5) + (1<<9),
            (Shape::O, _) => (1<<1) + (1<<2) + (1<<5) + (1<<6),
            (Shape::S, Orientation::Up) => (1<<1) + (1<<2) + (1<<4) + (1<<5),
            (Shape::S, Orientation::Right) => (1<<1) + (1<<5) + (1<<6) + (1<<10),
            (Shape::S, Orientation::Down) => (1<<5) + (1<<6) + (1<<8) + (1<<9),
            (Shape::S, Orientation::Left) => (1<<0) + (1<<4) + (1<<5) + (1<<9),
            (Shape::T, Orientation::Up) => (1<<1) + (1<<4) + (1<<5) + (1<<6),
            (Shape::T, Orientation::Right) => (1<<1) + (1<<5) + (1<<6) + (1<<9),
            (Shape::T, Orientation::Down) => (1<<4) + (1<<5) + (1<<6) + (1<<9),
            (Shape::T, Orientation::Left) => (1<<1) + (1<<4) + (1<<5) + (1<<9),
            (Shape::Z, Orientation::Up) => (1<<0) + (1<<1) + (1<<5) + (1<<6),
            (Shape::Z, Orientation::Right) => (1<<2) + (1<<5) + (1<<6) + (1<<9),
            (Shape::Z, Orientation::Down) => (1<<4) + (1<<5) + (1<<9) + (1<<10),
            (Shape::Z, Orientation::Left) => (1<<1) + (1<<4) + (1<<5) + (1<<8),
        };
    }
    /// Return the pointer to the tetromino.
    pub fn cells(&self) -> u32 {
        self.cells
    }
}

/// ColumnMask uses 3 bits per column of each row.
/// There are 8 different states for each cell on the play board, represented by the 3 bits.
/// The states are the seven different types of tetrominos, or no tetromino.
type ColumnMask = u64;

#[wasm_bindgen]
pub struct World {
    /// Width of the tetris-game.
    width : u32,
    /// Height of the tetris-game.
    height: u32,
    /// The current tetromino in the game.
    tetromino: Tetromino,
    /// The horizontal shift of current tetromino.
    shift: i32,
    /// The vertical drop of the current tetromino.
    drop: i32,
    /// The uncleared old tetrominos in the game.
    old_cells: Vec<Option<Shape>>,
}


#[wasm_bindgen]
impl World {

    pub fn new(width: u32, height: u32, shape: Shape, orientation: Orientation) -> World {
        let shift = (width/2-2) as i32;
        let drop = 0 as i32;
        let mut world = World {
            width,
            height,
            tetromino: Tetromino::new(shape, orientation),
            shift,
            drop,
            old_cells: vec![None;(width*height) as usize],
        };
        world.old_cells[25] = Some(Shape::L);
        world.old_cells[74] = Some(Shape::J);
        world.old_cells[103] = Some(Shape::Z);
        world.update_tetromino_cells();
        if !world.check_tetromino(world.shift, world.drop) {
            output_js("Turn over.".to_string());
        }
        world
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
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
    
    pub fn update_tetromino_cells(&mut self) {
        self.tetromino.update();
    }

    /// Return the pointer to the tetromino.
    pub fn tetromino_cells(&self) -> u32 {
        self.tetromino.cells()
    }

    pub fn update(&mut self) {

        self.soft_drop();
        self.update_tetromino_cells();
    }

    pub fn shift_left(&mut self) {
        if self.check_tetromino(self.shift-1, self.drop) {
            self.shift -= 1;
        }
    }

    pub fn shift_right(&mut self) {
        if self.check_tetromino(self.shift+1, self.drop) {
            self.shift += 1;
        }
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
        if self.check_tetromino(self.shift, self.drop+1) {
            self.drop += 1;
        }
    }

    pub fn tetromino_shift(&self) -> i32 {
        self.shift
    }

    pub fn tetromino_drop(&self) -> i32 {
        self.drop
    }

    /// Return the pointer to the snake body.
    pub fn old_cells(&mut self) -> *const Option<Shape> {
        self.old_cells.as_ptr()
    }

    fn check_tetromino(&self, shift: i32, drop: i32) -> bool {
        let mut mask = 1;
        let mut index: i32;
        for row in 0..4 as i32 {
            index = (drop+row)*self.width as i32 + shift;
            for col in 0..4 as i32 {
                if (mask & self.tetromino.cells) == mask {
                    //output_js(format!("shift: {}, drop: {}, row: {}, col: {}, cell: {:?}", shift, drop, row, col, self.old_cells[(index+shift+col) as usize]));
                    if (shift + (col as i32) < 0) || (shift+col as i32 >= self.width as i32) || ((drop+row) >= self.height as i32) {
                        return false;
                    }
                    if index+col >= (self.width * self.height) as i32 {
                        return false;
                    }
                    match self.old_cells[(index + col) as usize] {
                        Some(_) => return false,
                        None => {},
                    }
                }
                mask <<= 1;
            }
        }
        true
    }

    fn check_tetromino_rotation(&self) -> bool {
        for row in 0..4 as i32 {
            for col in 0..4 as i32 {
            }
        }
        true
    }
    pub fn tetromino_shape(&self) -> Shape {
        self.tetromino.shape()
    }
    pub fn tetromino_orientation(&self) -> Orientation {
        self.tetromino.orientation()
    }

}


