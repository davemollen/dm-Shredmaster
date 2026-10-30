use crate::shared::{biquad_filter::BiquadFilter, inverting_op_amp::BilinearTransform};

// Inverting op-amp
pub struct OpAmp3 {
  op_amp: BiquadFilter,
  z_domain_coefficients: ([f32; 3], [f32; 3]),
}

impl OpAmp3 {
  pub fn new(sample_rate: f32) -> Self {
    let bilinear_transform = BilinearTransform::new(sample_rate);
    let s_domain_coefficients = (-9671.1798839458, [1., 2162.8275013188, 199817.76619723]);
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
