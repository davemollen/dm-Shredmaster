mod ramp_smooth;
use ramp_smooth::RampSmooth;

const RAMP_FREQ: f32 = 20.;

pub struct SmoothParameters {
  smooth_gain: RampSmooth,
  smooth_bass: RampSmooth,
  smooth_contour: RampSmooth,
  smooth_treble: RampSmooth,
  smooth_volume: RampSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_gain: RampSmooth::new(sample_rate, RAMP_FREQ),
      smooth_bass: RampSmooth::new(sample_rate, RAMP_FREQ),
      smooth_contour: RampSmooth::new(sample_rate, RAMP_FREQ),
      smooth_treble: RampSmooth::new(sample_rate, RAMP_FREQ),
      smooth_volume: RampSmooth::new(sample_rate, RAMP_FREQ),
    }
  }

  pub fn initialize(&mut self, gain: f32, bass: f32, contour: f32, treble: f32, volume: f32) {
    self.smooth_gain.initialize(gain);
    self.smooth_bass.initialize(bass);
    self.smooth_contour.initialize(contour);
    self.smooth_treble.initialize(treble);
    self.smooth_volume.initialize(volume);
  }

  pub fn process(
    &mut self,
    gain: f32,
    bass: f32,
    contour: f32,
    treble: f32,
    volume: f32,
  ) -> (f32, f32, f32, f32, f32) {
    (
      self.smooth_gain.process(gain),
      self.smooth_bass.process(bass),
      self.smooth_contour.process(contour),
      self.smooth_treble.process(treble),
      self.smooth_volume.process(volume),
    )
  }
}
// pub struct SmoothParameters<const T: usize> {
//   filters: [RampSmooth; T],
// }

// impl<const T: usize> SmoothParameters<T> {
//   pub fn new(sample_rate: f32) -> Self {
//     Self {
//       filters: [RampSmooth::new(sample_rate, RAMP_FREQ); T],
//     }
//   }

//   pub fn process(&mut self, params: [f32; T]) -> [f32; T] {
//     self
//       .filters
//       .iter_mut()
//       .zip(params)
//       .map(|(filter, param)| filter.process(param))
//       .collect::<Vec<f32>>()
//       .try_into()
//       .unwrap()
//   }
// }
