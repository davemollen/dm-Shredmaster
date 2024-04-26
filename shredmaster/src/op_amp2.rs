use crate::shared::inverting_op_amp::InvertingOpAmp;

pub struct OpAmp2 {
  op_amp: InvertingOpAmp,
}

impl OpAmp2 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: InvertingOpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let s_domain_coefficients = (-2594706.7981318, [1., 33082.511676181, 56113901.343681]);
    self.op_amp.process(input, s_domain_coefficients)
  }
}
