extern crate lv2;
extern crate shredmaster;
use lv2::prelude::*;
use shredmaster::{Params, Shredmaster};

#[derive(PortCollection)]
struct Ports {
  gain: InputPort<InPlaceControl>,
  bass: InputPort<InPlaceControl>,
  contour: InputPort<InPlaceControl>,
  treble: InputPort<InPlaceControl>,
  volume: InputPort<InPlaceControl>,
  brilliance: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
  enable: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-Shredmaster")]
struct DmShredmaster {
  shredmaster: Shredmaster,
  params: Params,
  // 0 = bypassed, 1 = engaged; ramps to avoid a click when switching
  wet_mix: f32,
  wet_mix_step: f32,
  is_initialized: bool,
}

impl Plugin for DmShredmaster {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      shredmaster: Shredmaster::new(sample_rate),
      params: Params::new(sample_rate),
      wet_mix: 1.,
      // 20 ms crossfade between dry and processed signal
      wet_mix_step: (0.02 * sample_rate).recip(),
      is_initialized: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(
      ports.gain.get(),
      ports.bass.get(),
      ports.contour.get(),
      ports.treble.get(),
      ports.volume.get(),
      ports.brilliance.get() == 1.,
    );

    let target = if ports.enable.get() > 0.5 { 1. } else { 0. };
    if !self.is_initialized {
      self.wet_mix = target;
      self.is_initialized = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let dry = input.get();
      let wet = self.shredmaster.process(dry, &mut self.params);

      if self.wet_mix < target {
        self.wet_mix = (self.wet_mix + self.wet_mix_step).min(target);
      } else if self.wet_mix > target {
        self.wet_mix = (self.wet_mix - self.wet_mix_step).max(target);
      }

      output.set(dry + (wet - dry) * self.wet_mix);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmShredmaster);
