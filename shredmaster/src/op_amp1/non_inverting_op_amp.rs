mod bilinear_transform;
use crate::shared::biquad_filter::BiquadFilter;
use bilinear_transform::BilinearTransform;

pub struct NonInvertingOpAmp {
  op_amp: BiquadFilter,
  bilinear_transform: BilinearTransform,
}

impl NonInvertingOpAmp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: BiquadFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
    }
  }

  pub fn reset(&mut self) {
    self.op_amp.reset();
  }

  pub fn process(&mut self, input: f32, s_domain_coefficients: ([f32; 3], [f32; 3])) -> f32 {
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.op_amp.process(input, z_domain_coefficients)
  }
}
