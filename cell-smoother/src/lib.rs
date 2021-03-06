use rand::Rng;
use wasm_bindgen::prelude::*;

#[cfg(test)]
use std::fmt::{Debug, Error, Formatter};

#[wasm_bindgen]
#[cfg_attr(test, derive(Debug, Clone, Eq, PartialEq))]
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
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    fn get_cell_index(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        let cell_count: usize = width * height;
        let mut cells: Vec<Cell> = Vec::with_capacity(cell_count);

        let mut rng = rand::thread_rng();
        for _ in 0..cell_count {
            cells.push(Cell::new(rng.gen_range(0, 16)));
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn evolve(&mut self) {
        let mut new_cells: Vec<Cell> = Vec::with_capacity(self.cells.len());

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

        // Here a simple assignment
        // ```
        // self.cells = new_cells;
        // ```
        // would work also but that will update also the pointer to `self.cells`
        // (as the new cells are stored elsewhere in memory) and thus the pointer
        // shared with Wasm would become outdated. To keep the pointer valid
        // the new cells are copied to the old allocated memory here.
        for i in 0..self.cells.len() {
            self.cells[i].value = new_cells[i].value;
        }
    }
}

#[cfg(test)]
impl Universe {
    pub fn set_cells(&mut self, cells: &Vec<Cell>) {
        assert_eq!(cells.len(), (&self.width * &self.height) as usize);

        for i in 0..self.cells.len() {
            self.cells[i] = Cell::new(cells[i].value);
        }
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

    fn create_cells(values: Vec<u8>) -> Vec<Cell> {
        values.into_iter().map(|v| Cell::new(v)).collect()
    }

    #[test]
    fn universe_should_be_created_with_right_amount_of_cells() {
        let width: usize = 20;
        let height: usize = 20;
        let universe = Universe::new(width, height);

        assert_eq!(universe.cells().len(), (width as u16 * height as u16) as usize);
    }

    #[test]
    fn universe_should_be_created_with_random_cells() {
        let width: usize = 25;
        let height: usize = 25;
        let first_universe = Universe::new(width, height);
        let second_universe = Universe::new(width, height);

        // in theory this can sometimes fail as the values as set randomly but
        // this should be very unlikely due to the size of the test universe
        assert_ne!(first_universe.cells, second_universe.cells);
    }

    #[test]
    fn universe_should_be_created_with_cell_values_ranging_from_0_to_15() {
        let width: usize = 25;
        let height: usize = 25;
        let universe = Universe::new(width, height);

        let mut counts: Vec<u16> = vec![0; 16];
        for i in 0..universe.cells.len() {
            let val = universe.cells[i].value as usize;
            counts[val] = counts[val] + 1;
        }

        // Check that each value is at least in one cell; in theory this can sometimes
        // fail as the values as set randomly but this should be very unlikely
        // due to the size of the test universe.
        for i in 0..counts.len() {
            assert!(counts[i] > 0);
        }

        // there is no need to check for values below zero or above 15 here separately
        // as such values would cause a panic in the first `for` loop
    }

    #[test]
    fn cells_of_universe_should_match_created_ones_before_evolution() {
        let mut universe = Universe::new(2, 2);
        let cells = create_cells(vec![1, 2, 3, 4]);
        universe.set_cells(&cells);

        let expected_cells = create_cells(vec![1, 2, 3, 4]);
        assert_eq!(universe.cells(), &expected_cells);
    }

    #[test]
    fn cells_of_universe_should_evolve() {
        let mut universe = Universe::new(2, 2);
        let cells = create_cells(vec![1, 3, 3, 3]);
        universe.set_cells(&cells);

        let expected_cells = create_cells(vec![2, 2, 2, 3]);

        universe.evolve();

        assert_eq!(universe.cells(), &expected_cells);
    }
}
