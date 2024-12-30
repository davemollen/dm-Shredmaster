mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub gain: LinearSmooth,
  pub bass: LinearSmooth,
  pub contour: LinearSmooth,
  pub treble: LinearSmooth,
  pub volume: LinearSmooth,
  pub brilliance: bool,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      gain: LinearSmooth::new(sample_rate, 20.),
      bass: LinearSmooth::new(sample_rate, 20.),
      contour: LinearSmooth::new(sample_rate, 20.),
      treble: LinearSmooth::new(sample_rate, 20.),
      volume: LinearSmooth::new(sample_rate, 20.),
      brilliance: false,
      is_initialized: false,
    }
  }

  pub fn set(
    &mut self,
    gain: f32,
    bass: f32,
    contour: f32,
    treble: f32,
    volume: f32,
    brilliance: bool,
  ) {
    self.brilliance = brilliance;

    let bass = bass * bass * bass;
    let volume = volume * volume * volume;

    if self.is_initialized {
      self.gain.set_target(gain);
      self.bass.set_target(bass);
      self.contour.set_target(contour);
      self.treble.set_target(treble);
      self.volume.set_target(volume);
    } else {
      self.gain.reset(gain);
      self.bass.reset(bass);
      self.contour.reset(contour);
      self.treble.reset(treble);
      self.volume.reset(volume);
      self.is_initialized = true;
    }
  }
}
