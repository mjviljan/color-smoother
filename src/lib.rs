use std::fmt::{Debug, Error, Formatter};

// Clone and Eq's only used in test but #![cfg_attr…] doesn't seem to work
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cell {
    value: u8,
}

impl Cell {
    pub fn new(value: u8) -> Cell {
        Cell { value }
    }
}

pub struct Universe {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(cells: Vec<Cell>) -> Universe {
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
mod universe_tests {
    use crate::{Cell, Universe};

    #[test]
    fn cells_with_power_of_two_length_creates_universe_with_root_of_length_sides() {
        let expected_side_length: u8 = 5;

        let cells = vec![Cell::new(1); expected_side_length.pow(2) as usize];
        let universe = Universe::new(cells);

        assert_eq!(universe.width, expected_side_length);
        assert_eq!(universe.height, expected_side_length);
    }

    #[test]
    #[should_panic]
    fn cells_with_non_power_of_two_length_fail_to_create_universe() {
        let cells = vec![Cell::new(1), Cell::new(2), Cell::new(3)];
        Universe::new(cells);
    }

    #[test]
    fn cells_of_universe_should_match_created_ones_before_evolution() {
        let cells = vec![Cell::new(1), Cell::new(2), Cell::new(3), Cell::new(4)];
        let universe = Universe::new(cells);

        let expected_cells: Vec<Cell> = vec![Cell::new(1), Cell::new(2), Cell::new(3), Cell::new(4)];
        assert_eq!(universe.cells(), &expected_cells);
    }

    #[test]
    fn cells_of_universe_should_evolve() {
        let cells = vec![Cell::new(1), Cell::new(3), Cell::new(3), Cell::new(3)];
        let universe = Universe::new(cells);

        // just debug print the universe for now (actual test TBD)
        println!("{:#?}", universe);
    }
}
