use crate::shared::inverting_op_amp::InvertingOpAmp;

pub struct OpAmp3 {
  op_amp: InvertingOpAmp,
}

impl OpAmp3 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: InvertingOpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let s_domain_coefficients = (-9671.1798839458, [1., 2162.8275013188, 199817.76619723]);
    self.op_amp.process(input, s_domain_coefficients)
  }
}
