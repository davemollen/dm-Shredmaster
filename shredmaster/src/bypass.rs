use crate::{
  params::{LinearSmooth, Smoother},
  shared::float_ext::FloatExt,
};

const FADE_TIME_IN_MS: f32 = 1.;

pub struct Bypass {
  mix_factor: LinearSmooth,
}

impl Bypass {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      mix_factor: LinearSmooth::new(sample_rate, 1000. / FADE_TIME_IN_MS),
    }
  }

  pub fn reset(&mut self) {
    self.mix_factor.reset(1.);
  }

  pub fn enabled(&mut self, target: f32) {
    self.mix_factor.set_target(target);
  }

  pub fn process(&mut self, dry: f32, wet: f32) -> f32 {
    let mix_factor = self.mix_factor.next();
    dry.mix(wet, mix_factor)
  }
}
