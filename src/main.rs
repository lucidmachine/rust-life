mod world;

use world::World;

fn main() {
    // TODO: Command line interface
    println!("Dummy world test.");

    let mut world = World {
        cells: [[0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0]],
    };

    for i in 0..10 {
        println!("== Generation {} ==", i);
        println!("{}", world);
        world = world.evolve();
    }
}
