const SIDE_LENGTH: usize = 5;
const ALIVE: i32 = 1;
const DEAD: i32 = 0;
static ALIVE_CHAR: &'static str = "#";
static DEAD_CHAR: &'static str = "-";

#[derive(Debug)]
struct World {
    // TODO: Arbitrary length and width
    cells: [[i32; SIDE_LENGTH]; SIDE_LENGTH],
}

impl World {
    // TODO: new()
    // TODO: load() and save()
    fn evolve(&self) -> World {
        let mut new_world = World { cells: [[0_i32; SIDE_LENGTH]; SIDE_LENGTH] };
        let s = SIDE_LENGTH as i32;

        // Compute new world cells
        for (r, row) in self.cells.into_iter().enumerate() {
            for (c, cell) in row.into_iter().enumerate() {
                // Get neigbor indecies, wrapping edges
                let prev_r: usize = ((((r as i32) - 1) % s + s) % s) as usize;
                let next_r: usize = ((((r as i32) + 1) % s + s) % s) as usize;
                let prev_c: usize = ((((c as i32) - 1) % s + s) % s) as usize;
                let next_c: usize = ((((c as i32) + 1) % s + s) % s) as usize;

                // TODO: Eliminate the assumption that all cells contain 0 or 1.
                let num_living_neighbors =
                    self.cells[prev_r][prev_c] + self.cells[prev_r][c] + self.cells[prev_r][next_c] +
                    self.cells[r][prev_c]                              + self.cells[r][next_c] +
                    self.cells[next_r][prev_c] + self.cells[next_r][c] + self.cells[next_r][next_c];

                match cell {
                    &DEAD => match num_living_neighbors {
                        3 => new_world.cells[r][c] = ALIVE,     // New life
                        _ => new_world.cells[r][c] = DEAD,      // Nothing
                    },
                    &ALIVE => match num_living_neighbors {
                        2 | 3 => new_world.cells[r][c] = ALIVE, // Continued life
                        _ => new_world.cells[r][c] = DEAD,      // Overpopulation
                    },
                    &_ => continue, // TODO: Handle invalid data.
                }
            }
        }

        new_world
    }
}

impl PartialEq for World {
    fn eq(&self, other: &World) -> bool {
        self.cells == other.cells
    }
}
// TODO: impl std::fmt::Display

fn main() {
    // TODO: Command line interface
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn test_evolve() {
        // Blinker
        let blinker_generation_0 = World {
            cells: [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 1, 1, 1, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ]
        };

        let blinker_generation_1 = World {
            cells: [
                [0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0],
            ]
        };

        assert_eq!(blinker_generation_0.evolve(), blinker_generation_1);
        assert_eq!(blinker_generation_0.evolve().evolve(), blinker_generation_0);

        // Boat
        let boat_generation_0 = World {
            cells: [
                [0, 0, 0, 0, 0],
                [0, 1, 1, 0, 0],
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0],
            ]
        };

        assert_eq!(boat_generation_0.evolve(), boat_generation_0);
    }
}

