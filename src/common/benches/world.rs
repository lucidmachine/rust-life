#[macro_use]
extern crate bencher;
extern crate common;

use bencher::Bencher;
use common::world::World;

fn bench_new(b: &mut Bencher) {
    b.iter(|| { World::new(); });
}

fn bench_evolve(b: &mut Bencher) {
    let world = World {
        cells: [[0, 0, 0, 0, 0],
                [0, 1, 1, 0, 0],
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0]],
    };

    b.iter(|| { world.evolve(); })
}

benchmark_group!(benches, bench_new, bench_evolve);
benchmark_main!(benches);
