const SIDE_LENGTH: usize = 3;
static ALIVE_CHAR: &'static str = "#";
static DEAD_CHAR: &'static str = "-";

#[derive(Debug)]
struct World {
    cells: [[usize; SIDE_LENGTH]; SIDE_LENGTH],
}
impl World {
    fn evolve(&self) -> World {
        World { cells: self.cells }
    }
}
impl PartialEq for World {
    fn eq(&self, other: &World) -> bool {
        self.cells == other.cells
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests{
    use super::World;

    #[test]
    fn test_evolve() {
        let generation_0 = World { cells: [
            [0, 0, 0],
            [1, 1, 1],
            [0, 0, 0],
        ] };

        let expected_generation_1 = World { cells: [
            [0, 1, 0],
            [0, 1, 0],
            [0, 1, 0],
        ] };

        assert_eq!(generation_0.evolve(), expected_generation_1);
    }
}

