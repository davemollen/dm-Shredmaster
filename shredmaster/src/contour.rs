use crate::shared::{
  bilinear_transform::BilinearTransform, third_order_iir_filter::ThirdOrderIIRFilter,
};

const R1: f32 = 100.;
const R2: f32 = 33000.;
const R3: f32 = 33000.;
const R4: f32 = 100000.;
const C1: f32 = 1e-9;
const C2: f32 = 1e-7;
const C3: f32 = 4.7e-8;

pub struct Contour {
  filter: ThirdOrderIIRFilter,
  bilinear_transform: BilinearTransform,
}

impl Contour {
  const C1C2: f32 = C1 * C2;
  const C1C3: f32 = C1 * C3;
  const C2C3: f32 = C2 * C3;
  const C1C3R1: f32 = Self::C1C3 * R1;
  const C1C2C3: f32 = Self::C1C2 * C3;
  const C1C3R2: f32 = Self::C1C3 * R2;
  const C2C3R1: f32 = Self::C2C3 * R1;
  const C1C2R1: f32 = Self::C1C2 * R1;
  const C1C2C3R1: f32 = Self::C1C2C3 * R1;
  const C1C2C3R1R3: f32 = Self::C1C2C3R1 * R3;
  const C1C2C3R1R2: f32 = Self::C1C2C3R1 * R2;

  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      filter: ThirdOrderIIRFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32, contour: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(contour);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, contour: f32) -> ([f32; 4], [f32; 4]) {
    let r4_a = contour * R4;
    let r4_b = (1. - contour) * R4;

    let r3r4_a = R3 * r4_a;
    let r2r4_a = R2 * r4_a;

    let b0 =
      r4_b * Self::C1C2C3 * R3 * r4_a + r4_b * Self::C1C2C3 * r2r4_a + Self::C1C2C3 * r2r4_a * R3;
    let b1 = r4_b * Self::C2C3 * r4_a
      + r4_b * Self::C1C3 * R3
      + Self::C1C2 * r3r4_a
      + r4_b * Self::C1C3R2
      + Self::C1C2 * r2r4_a
      + Self::C1C3R2 * R3;
    let b2 = r4_b * C3 + C2 * r4_a + C1 * R3 + C1 * R2;

    let a0 = b0
      + r4_b * Self::C1C2C3R1R3
      + Self::C1C2C3R1 * r3r4_a
      + r4_b * Self::C1C2C3R1R2
      + Self::C1C2C3R1R2 * r4_a
      + Self::C1C2C3R1R2 * R3;
    let a1 = b1
      + Self::C2C3 * r2r4_a
      + r4_b * Self::C2C3R1
      + Self::C2C3R1 * r4_a
      + Self::C1C2R1 * R3
      + Self::C1C3R1 * R3
      + Self::C1C2R1 * R2
      + Self::C1C3R1 * R2
      + Self::C2C3R1 * R2;
    let a2 = b2 + C3 * R2 + C2 * R1 + C3 * R1;

    ([b0, b1, b2, 1.], [a0, a1, a2, 1.])
  }
}

#[cfg(test)]
mod tests {
  use super::Contour;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_zero() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [0.0, 3.61383e-07, 0.004765999999999999, 1.],
      [
        3.61383e-12,
        4.2486319999999997e-07,
        0.006331699999999999,
        1.0,
      ],
    );
    assert_eq!(contour.get_s_domain_coefficients(1.), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_a_half() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1.031415e-09, 1.2286283000000001e-05, 0.007416, 1.],
      [
        1.0350288300000003e-09,
        2.01047632e-05,
        0.008981699999999999,
        1.,
      ],
    );
    assert_eq!(contour.get_s_domain_coefficients(0.5), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_one() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [5.1183e-10, 7.11183e-07, 0.010066, 1.],
      [5.154438299999999e-10, 1.62846632e-05, 0.0116317, 1.],
    );
    assert_eq!(contour.get_s_domain_coefficients(0.001), coeffs)
  }
}
