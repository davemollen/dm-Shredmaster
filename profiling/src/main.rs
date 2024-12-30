mod utils;
use shredmaster::{Params, Shredmaster};
use utils::generate_signal;

fn main() {
  let mut shredmaster = Shredmaster::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0., 0.5, 0.5, false);

  loop {
    let input = generate_signal();
    shredmaster.process(input, &mut params);
  }
}
