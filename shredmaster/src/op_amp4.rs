use crate::shared::inverting_op_amp::InvertingOpAmp;

// Maybe this one can be replaced with a one-pole filter, because the highpass frequency is at 7.23Hz
pub struct OpAmp4 {
  op_amp: InvertingOpAmp,
}

impl OpAmp4 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: InvertingOpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, brilliance: bool) -> f32 {
    let s_domain_coefficients = if brilliance {
      // highpass cutoff at 7.23Hz & lowpass cutoff at 3386Hz
      (-21276.595744681, [1., 21322.050290135, 967117.98839458])
    } else {
      // highpass cutoff at 7.23Hz & lowpass cutoff at 1592Hz
      (-10000., [1., 10045.454545455, 454545.45454545])
    };
    self.op_amp.process(input, s_domain_coefficients)
  }
}
