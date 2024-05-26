mod utils;
use shredmaster::Shredmaster;
use utils::generate_signal;

fn main() {
  let mut shredmaster = Shredmaster::new(44100.);
  loop {
    let input = generate_signal();
    shredmaster.process(input, 0.5, 0.5, 0., 0.5, 0.5, false);
  }
}
