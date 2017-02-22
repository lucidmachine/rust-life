#[macro_use]
extern crate bencher;
extern crate common;

use bencher::Bencher;
use common::world::World;

fn bench_new(b: &mut Bencher) {
    b.iter(||{
        World::new();
    });
}

benchmark_group!(benches, bench_new);
benchmark_main!(benches);