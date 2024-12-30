#![feature(portable_simd)]
mod clipper;
mod contour;
mod op_amp1;
mod op_amp2;
mod op_amp3;
mod op_amp4;
mod params;
mod tone_stack;
mod shared {
  pub mod bilinear_transform;
  pub mod float_ext;
  pub mod inverting_op_amp;
  pub mod third_order_iir_filter;
}
pub use params::Params;
use {
  clipper::Clipper, contour::Contour, op_amp1::OpAmp1, op_amp2::OpAmp2, op_amp3::OpAmp3,
  op_amp4::OpAmp4, params::Smoother, tone_stack::ToneStack,
};

pub struct Shredmaster {
  op_amp1: OpAmp1,
  op_amp2: OpAmp2,
  clipper: Clipper,
  tone_stack: ToneStack,
  op_amp3: OpAmp3,
  contour: Contour,
  op_amp4: OpAmp4,
}

impl Shredmaster {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp1: OpAmp1::new(sample_rate),
      op_amp2: OpAmp2::new(sample_rate),
      clipper: Clipper::new(),
      tone_stack: ToneStack::new(sample_rate),
      op_amp3: OpAmp3::new(sample_rate),
      contour: Contour::new(sample_rate),
      op_amp4: OpAmp4::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let gain = params.gain.next();
    let bass = params.bass.next();
    let contour = params.contour.next();
    let treble = params.treble.next();
    let volume = params.volume.next();
    let brilliance = params.brilliance;

    let op_amp1_output = self.op_amp1.process(input, gain);
    let op_amp2_output = self.op_amp2.process(op_amp1_output);
    let clipper_output = self.clipper.process(op_amp2_output);
    let tone_stack_output = self.tone_stack.process(clipper_output, bass, treble);
    let op_amp3_output = self.op_amp3.process(tone_stack_output);
    let contour_output = self.contour.process(op_amp3_output, contour);
    let op_amp4_output = self.op_amp4.process(contour_output, brilliance);
    op_amp4_output * volume
  }
}
