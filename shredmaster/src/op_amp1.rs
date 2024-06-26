mod non_inverting_op_amp;
use non_inverting_op_amp::NonInvertingOpAmp;

const R1: f32 = 3300.;
const R2: f32 = 100000.;
const C1: f32 = 4.7e-8;
const C2: f32 = 1e-10;

pub struct OpAmp1 {
  op_amp: NonInvertingOpAmp,
}

impl OpAmp1 {
  const R1C1: f32 = R1 * C1;

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
    let r2 = (gain * R2).max(1.);

    let r2c2: f32 = r2 * C2;

    let a0 = Self::R1C1 * r2c2;
    let a1 = Self::R1C1 + r2c2;
    let b1 = r2 * C1 + a1;

    ([a0, b1, 1.], [a0, a1, 1.])
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
        [0.0, 0.004865099999999999, 1.0],
        [1.5510000000000001e-09, 0.0001651, 1.0]
      )
    );
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_five_tenths() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(0.5),
      (
        [0.0, 0.0025100999999999995, 1.0],
        [7.755000000000001e-10, 0.00016010000000000002, 1.0]
      )
    );
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_one_hundredth() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(0.01),
      (
        [0.0, 0.0002022, 1.0],
        [1.5510000000000003e-11, 0.0001552, 1.0]
      )
    );
  }
}
