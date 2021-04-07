use wasm_bindgen::prelude::*;

#[cfg(test)]
use std::fmt::{Debug, Error, Formatter};

#[cfg_attr(test, derive(Debug, Clone, Eq, PartialEq))]
#[wasm_bindgen]
pub struct Cell {
    value: u8,
}

impl Cell {
    pub fn new(value: u8) -> Cell {
        Cell { value }
    }

    fn sum_of_cells(cells: &Vec<&Cell>) -> u8 {
        cells.iter().fold(0, |acc, cell| acc + cell.value)
    }

    fn weighted_average_of_neighbours(
        cardinal_neighbours: &Vec<&Cell>,
        diagonal_neighbours: &Vec<&Cell>,
    ) -> f32 {
        let cardinal_sum: u8 = Cell::sum_of_cells(&cardinal_neighbours);
        let diagonal_sum: u8 = Cell::sum_of_cells(&diagonal_neighbours);

        // weighted average: cardinal neighbours have double the weight of diagonal neighbours
        (cardinal_sum * 2 + diagonal_sum) as f32
            / (cardinal_neighbours.len() * 2 + diagonal_neighbours.len()) as f32
    }

    pub fn evolve(
        &self,
        cardinal_neighbours: &Vec<&Cell>,
        diagonal_neighbours: &Vec<&Cell>,
    ) -> Cell {
        let weighted_average =
            Cell::weighted_average_of_neighbours(&cardinal_neighbours, &diagonal_neighbours);
        let rounded_average = f32::round(weighted_average) as u8;

        if rounded_average > self.value {
            Cell {
                value: self.value + 1,
            }
        } else if rounded_average < self.value {
            Cell {
                value: self.value - 1,
            }
        } else {
            Cell { value: self.value }
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn _new(cells: Vec<Cell>) -> Universe {
        let root = f32::sqrt(cells.len() as f32);

        if root.fract() != 0.0 {
            panic!("Can't create a square universe with given cells");
        }

        // safe because root can't have fractions here
        unsafe {
            let side: u8 = root.to_int_unchecked();

            Universe {
                width: side,
                height: side,
                cells,
            }
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    fn get_cell_index(&self, col: u8, row: u8) -> usize {
        (row * self.width + col) as usize
    }

    pub fn evolve(&mut self) {
        let mut new_cells: Vec<Cell> = Vec::with_capacity((self.width * self.height) as usize);
        for row in 0..self.height {
            for col in 0..self.width {
                let cell_index = self.get_cell_index(col, row);

                let mut cardinal_neighbours: Vec<&Cell> = Vec::new();
                let mut diagonal_neighbours: Vec<&Cell> = Vec::new();

                if row > 0 {
                    cardinal_neighbours.push(&self.cells[self.get_cell_index(col, row - 1)]);

                    if col > 0 {
                        diagonal_neighbours
                            .push(&self.cells[self.get_cell_index(col - 1, row - 1)]);
                    }
                    if col < self.width - 1 {
                        diagonal_neighbours
                            .push(&self.cells[self.get_cell_index(col + 1, row - 1)]);
                    }
                }
                if row < self.height - 1 {
                    cardinal_neighbours.push(&self.cells[self.get_cell_index(col, row + 1)]);

                    if col > 0 {
                        diagonal_neighbours
                            .push(&self.cells[self.get_cell_index(col - 1, row + 1)]);
                    }
                    if col < self.width - 1 {
                        diagonal_neighbours
                            .push(&self.cells[self.get_cell_index(col + 1, row + 1)]);
                    }
                }
                if col > 0 {
                    cardinal_neighbours.push(&self.cells[self.get_cell_index(col - 1, row)]);
                }
                if col < self.width - 1 {
                    cardinal_neighbours.push(&self.cells[self.get_cell_index(col + 1, row)]);
                }

                new_cells.push(
                    self.cells[cell_index].evolve(&cardinal_neighbours, &diagonal_neighbours),
                );
            }
        }

        self.cells = new_cells;
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u8, height: u8) -> Universe {
        let cell_count = width * height;
        let mut cells: Vec<Cell> = Vec::with_capacity(cell_count as usize);

        for i in 0..cell_count {
            cells.push(Cell::new(i));
        }

        Universe::_new(cells)
    }

    pub fn jscells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

#[cfg(test)]
impl Debug for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut s: String = "".to_owned();

        for row in 0..self.height {
            s.push_str("\n|");
            for col in 0..self.width {
                s.push_str(&format!(
                    "{:>2}|",
                    self.cells[(row * self.width + col) as usize].value
                ));
            }
        }

        f.debug_struct(&s).finish()
    }
}

#[cfg(test)]
mod cell_tests {
    use crate::Cell;

    #[test]
    fn cell_evolves_to_higher_value_if_rounded_average_of_neighbours_is_higher() {
        let original_cell = Cell::new(5);

        // these must be created as variables before referring to them in `vec!`,
        // otherwise they don't live long enough for the reference to work
        let five_cell = Cell::new(5);
        let six_cell = Cell::new(6);

        // weighted average of neighbours' values is 5.5
        let cardinal_neighbours = vec![&six_cell, &five_cell];
        let diagonal_neighbours = vec![&six_cell, &five_cell];
        let new_cell = original_cell.evolve(&cardinal_neighbours, &diagonal_neighbours);

        assert_eq!(new_cell.value, 6);
    }

    #[test]
    fn cell_evolves_to_lower_value_if_rounded_average_of_neighbours_is_lower() {
        let original_cell = Cell::new(5);

        let four_cell = Cell::new(4);
        let five_cell = Cell::new(5);

        // weighted average of neighbours' values is ~4.4
        let cardinal_neighbours = vec![&five_cell, &four_cell];
        let diagonal_neighbours = vec![&five_cell, &four_cell, &four_cell];
        let new_cell = original_cell.evolve(&cardinal_neighbours, &diagonal_neighbours);

        assert_eq!(new_cell.value, 4);
    }

    #[test]
    fn average_of_neighbours_is_weighted_with_cardinal_neighbours_having_more_weight() {
        let original_cell = Cell::new(5);

        let one_cell = Cell::new(1);
        let two_cell = Cell::new(2);
        let eight_cell = Cell::new(8);
        let nine_cell = Cell::new(9);

        // average of the neighbours' values is 4.4 but their weighted average is ~5.57
        let cardinal_neighbours = vec![&nine_cell, &eight_cell];
        let diagonal_neighbours = vec![&two_cell, &two_cell, &one_cell];
        let new_cell = original_cell.evolve(&cardinal_neighbours, &diagonal_neighbours);

        assert_eq!(new_cell.value, 6);
    }
}

#[cfg(test)]
mod universe_tests {
    use crate::{Cell, Universe};

    #[test]
    fn cells_with_power_of_two_length_creates_universe_with_root_of_length_sides() {
        let expected_side_length: u8 = 5;

        let cells = vec![Cell::new(1); expected_side_length.pow(2) as usize];
        let universe = Universe::_new(cells);

        assert_eq!(universe.width, expected_side_length);
        assert_eq!(universe.height, expected_side_length);
    }

    #[test]
    #[should_panic]
    fn cells_with_non_power_of_two_length_fail_to_create_universe() {
        let cells = vec![Cell::new(1), Cell::new(2), Cell::new(3)];
        Universe::_new(cells);
    }

    #[test]
    fn cells_of_universe_should_match_created_ones_before_evolution() {
        let cells = vec![Cell::new(1), Cell::new(2), Cell::new(3), Cell::new(4)];
        let universe = Universe::_new(cells);

        let expected_cells: Vec<Cell> =
            vec![Cell::new(1), Cell::new(2), Cell::new(3), Cell::new(4)];
        assert_eq!(universe.cells(), &expected_cells);
    }

    #[test]
    fn cells_of_universe_should_evolve() {
        let cells = vec![Cell::new(1), Cell::new(3), Cell::new(3), Cell::new(3)];
        let expected_cells = vec![Cell::new(2), Cell::new(2), Cell::new(2), Cell::new(3)];

        let mut universe = Universe::_new(cells);
        universe.evolve();

        assert_eq!(universe.cells(), &expected_cells);
    }
}
