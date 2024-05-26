#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use shredmaster::Shredmaster;
use utils::generate_signal_stream;

fn shredmaster_bench(c: &mut Criterion) {
  let mut shredmaster = Shredmaster::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("shredmaster", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        shredmaster.process(*signal, 0.5, 0.5, 0., 0.5, 0.5, false);
      }
    })
  });
}

criterion_group!(benches, shredmaster_bench);
criterion_main!(benches);
