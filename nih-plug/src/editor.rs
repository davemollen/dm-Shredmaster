#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path = "./editor/components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
mod ui_data;
use crate::shredmaster_parameters::ShredmasterParameters;
use nih_plug::params::Param;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, vizia_assets, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (440, 200))
}

pub(crate) fn create(
  params: Arc<ShredmasterParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      vizia_assets::register_roboto(cx);
      vizia_assets::register_roboto_bold(cx);
      cx.set_default_font(&[vizia_assets::ROBOTO]);
      cx.add_stylesheet(STYLE).ok();

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.gain.name(),
            UiData::params,
            params.gain.as_ptr(),
            |params| &params.gain,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.bass.name(),
            UiData::params,
            params.bass.as_ptr(),
            |params| &params.bass,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.contour.name(),
            UiData::params,
            params.contour.as_ptr(),
            |params| &params.contour,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.treble.name(),
            UiData::params,
            params.treble.as_ptr(),
            |params| &params.treble,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.volume.name(),
            UiData::params,
            params.volume.as_ptr(),
            |params| &params.volume,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamCheckbox::new(
            cx,
            params.brilliance.name(),
            UiData::params,
            params.brilliance.as_ptr(),
            |params| &params.brilliance,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          )
          .top(Pixels(-4.0));
        })
        .child_space(Stretch(1.0));

        Label::new(cx, "ShredMaster")
          .font_size(32.0)
          .font_weight(FontWeightKeyword::ExtraBold)
          .color("#C9C06A")
          .border_color("#C9C06A")
          .border_width(Pixels(1.0))
          .child_space(Stretch(1.0))
          .width(Pixels(200.0))
          .height(Pixels(36.0))
          .top(Pixels(32.0))
          .bottom(Pixels(32.0))
          .left(Stretch(1.0))
          .right(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#161616");
    },
  )
}
