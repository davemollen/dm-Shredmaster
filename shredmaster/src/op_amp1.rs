mod non_inverting_op_amp;
use non_inverting_op_amp::NonInvertingOpAmp;

pub struct OpAmp1 {
  op_amp: NonInvertingOpAmp,
}

impl OpAmp1 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: NonInvertingOpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, distortion: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(distortion);
    self.op_amp.process(input, s_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, gain: f32) -> ([f32; 3], [f32; 3]) {
    let r1 = 3300.;
    let c1 = 4.7e-8;
    let r2 = (gain * 100000.).max(1.);
    let c2 = 1e-10;

    let r1c1 = r1 * c1;
    let r2c2 = r2 * c2;

    let a0 = r1c1 * r2c2;
    let mut a1 = r1c1 + r2c2;
    let b2a2 = 1. / a0;
    let b1 = (r2 * c1 + a1) / a0;
    a1 /= a0;

    return ([1., b1, b2a2], [1., a1, b2a2]);
  }
}

#[cfg(test)]
mod tests {
  use super::OpAmp1;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_one() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(1.),
      (
        [1.0, 3136750.4835589933, 644745325.5963894],
        [1.0, 106447.45325596389, 644745325.5963894]
      )
    );
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_five_tenths() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(0.5),
      (
        [1.0, 3236750.4835589933, 1289490651.1927788],
        [1.0, 206447.4532559639, 1289490651.1927788]
      )
    );
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_one_hundredth() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(0.01),
      (
        [1.0, 13036750.483558992, 64474532559.63893],
        [1.0, 10006447.453255963, 64474532559.63893]
      )
    );
  }
}
