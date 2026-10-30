use crate::shared::{biquad_filter::BiquadFilter, inverting_op_amp::BilinearTransform};

// Inverting op-amp
pub struct OpAmp2 {
  op_amp: BiquadFilter,
  z_domain_coefficients: ([f32; 3], [f32; 3]),
}

impl OpAmp2 {
  pub fn new(sample_rate: f32) -> Self {
    let bilinear_transform = BilinearTransform::new(sample_rate);
    let s_domain_coefficients = (-2594706.7981318, [1., 33082.511676181, 56113901.343681]);
    let z_domain_coefficients = bilinear_transform.process(s_domain_coefficients);

    Self {
      op_amp: BiquadFilter::new(),
      z_domain_coefficients,
    }
  }

  pub fn reset(&mut self) {
    self.op_amp.reset();
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.op_amp.process(input, self.z_domain_coefficients)
  }
}
