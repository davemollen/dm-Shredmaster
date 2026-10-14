extern crate lv2;
extern crate shredmaster;
use lv2::prelude::*;
use shredmaster::{Bypass, Params, Shredmaster};

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
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-Shredmaster")]
struct DmShredmaster {
  shredmaster: Shredmaster,
  params: Params,
  bypass: Bypass,
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
      bypass: Bypass::new(sample_rate),
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.shredmaster.reset();
    self.bypass.reset();
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
    self.bypass.enabled(ports.enabled.get());

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let dry = input.get();
      let wet = self.shredmaster.process(dry, &mut self.params);
      let shredmaster_output = self.bypass.process(dry, wet);
      output.set(shredmaster_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmShredmaster);
