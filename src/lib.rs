mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
#[repr(u8)] // to specify the alignment to 1 byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index (&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        /*
        The function return the number of live neighbor cells,
        given a cell index.

        The reason to use self.height - 1 and self.width - 1 is because
        instead of doing:
        neighbor_row = (row - 1) < 0 ? (row - 1 + self.height) : (row - 1)
        it is easier to do:
        neighbor_row = (row + self.height - 1) % self.height
        */
        let self_index = self.get_index(row, col);
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().clone() {
            for delta_col in [self.width - 1, 0, 1].iter().clone() {
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                if idx == self_index {
                    continue;
                }
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    /*
    These are the public functions exported to Javascript.
    */

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            }
            else {
                Cell::Dead
            }
        }).collect();

        Universe { width, height, cells}
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick (&mut self) {
        let mut next_universe = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        (Cell::Dead, 3) => Cell::Alive,
                        (otherwise, _) => otherwise,
                    };

                    next_universe[idx] = next_cell;
            }
        }

        self.cells = next_universe;
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
