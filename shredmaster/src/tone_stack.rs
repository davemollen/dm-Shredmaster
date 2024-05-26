use crate::shared::{
  bilinear_transform::BilinearTransform, third_order_iir_filter::ThirdOrderIIRFilter,
};

pub struct ToneStack {
  filter: ThirdOrderIIRFilter,
  bilinear_transform: BilinearTransform,
}

impl ToneStack {
  const R1: f32 = 22000.;
  const R2: f32 = 100000.;
  const R3: f32 = 1000.;
  const R4: f32 = 6800.;
  const C1: f32 = 2.2e-8;
  const C2: f32 = 2.2e-7;
  const C3: f32 = 2.2e-8;

  pub fn new(sample_rate: f32) -> Self {
    Self {
      filter: ThirdOrderIIRFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, bass: f32, treble: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(bass, treble);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, bass: f32, treble: f32) -> ([f32; 4], [f32; 4]) {
    let r2 = Self::R2 * bass;

    let c1c2 = Self::C1 * Self::C2;
    let c1c3 = Self::C1 * Self::C3;
    let c2c3 = Self::C2 * Self::C3;
    let c1c2c3 = c1c2 * Self::C3;

    let r1_a = treble * Self::R1;
    let r1_b = (1. - treble) * Self::R1;

    let b0 = c1c2c3 * r2 * Self::R3 * Self::R4
      + r1_a * c1c2c3 * r2 * Self::R4
      + r1_a * c1c2c3 * r2 * Self::R3
      + c1c2c3 * r2 * Self::R3 * r1_b;
    let b1 = c1c3 * r2 * Self::R4
      + c1c2 * r2 * Self::R4
      + c2c3 * r2 * Self::R3
      + c1c3 * r2 * Self::R3
      + c1c3 * Self::R3 * Self::R4
      + c1c2 * Self::R3 * Self::R4
      + r1_a * c1c3 * Self::R4
      + r1_a * c1c2 * r2
      + r1_a * c1c2 * Self::R4
      + r1_a * c1c3 * Self::R3
      + r1_a * c1c2 * Self::R3
      + c1c2 * r2 * r1_b
      + c1c3 * Self::R3 * r1_b
      + c1c2 * Self::R3 * r1_b;
    let b2 = Self::C2 * r2
      + Self::C1 * r2
      + Self::C3 * Self::R3
      + Self::C2 * Self::R3
      + Self::C1 * Self::R3
      + r1_a * Self::C1;
    let a0 = b0 + c1c2c3 * r2 * Self::R4 * r1_b;
    let a1 = b1 + c2c3 * r2 * Self::R4 + c1c3 * Self::R4 * r1_b + c1c2 * Self::R4 * r1_b;
    let a2 = Self::C3 * Self::R4
      + Self::C2 * r2
      + Self::C1 * r2
      + Self::C2 * Self::R4
      + Self::C3 * Self::R3
      + Self::C2 * Self::R3
      + Self::C1 * Self::R3
      + r1_a * Self::C1
      + Self::C1 * r1_b;

    ([b0, b1, b2, 0.], [a0, a1, a2, 1.])
  }
}

#[cfg(test)]
mod tests {
  use super::ToneStack;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_one() {
    let tone_stack = ToneStack::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [5.515663999999999e-10, 7.9519264e-06, 0.012606, 0.0],
      [
        9.498015999999999e-10,
        9.9957616e-06,
        0.014493599999999999,
        1.0,
      ],
    );
    assert_eq!(tone_stack.get_s_domain_coefficients(0.5, 0.5), coeffs)
  }
}
