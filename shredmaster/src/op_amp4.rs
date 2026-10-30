use crate::shared::{biquad_filter::BiquadFilter, inverting_op_amp::BilinearTransform};

/**
 * Inverting op-amp
 * Maybe this one can be replaced with a one-pole filter, because the highpass frequency is at 7.23Hz
 */
pub struct OpAmp4 {
  op_amp: BiquadFilter,
  bilinear_transform: BilinearTransform,
  prev_brilliance: Option<bool>,
  z_domain_coefficients: ([f32; 3], [f32; 3]),
}

impl OpAmp4 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: BiquadFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
      prev_brilliance: None,
      z_domain_coefficients: ([0.; 3], [0.; 3]),
    }
  }

  pub fn reset(&mut self) {
    self.op_amp.reset();
  }

  pub fn process(&mut self, input: f32, brilliance: bool) -> f32 {
    if self
      .prev_brilliance
      .map_or(true, |value| value != brilliance)
    {
      let s_domain_coefficients = if brilliance {
        // highpass cutoff at 7.23Hz & lowpass cutoff at 3386Hz
        (-21276.595744681, [1., 21322.050290135, 967117.98839458])
      } else {
        // highpass cutoff at 7.23Hz & lowpass cutoff at 1592Hz
        (-10000., [1., 10045.454545455, 454545.45454545])
      };
      self.z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
      self.prev_brilliance = Some(brilliance);
    }
    self.op_amp.process(input, self.z_domain_coefficients)
  }
}
