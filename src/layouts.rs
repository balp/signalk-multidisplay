use crate::communication::SignalKCommunicator;
use crate::datatypes::{CourseOverGround, DataValues, SpeedOverGround, SpeedThroughWater};
use eframe::egui;
use egui::{RichText, Ui, Vec2};
use emath::vec2::Vec2;

// Layout variants
// - Single Value
// - Dual Value
// - Triple Value
// - Four values

pub trait Layout {
    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator);
}

pub struct SingleValueLayout {
    value: DataValues,
}

impl Default for SingleValueLayout {
    fn default() -> Self {
        Self {
            value: DataValues::ValueCourseOverGround(CourseOverGround::default()),
        }
    }
}

impl SingleValueLayout {
    pub(crate) fn add_config(&mut self, ui: &mut Ui) {
        let Self { value } = self;
        ui.label("Data to display");
        egui::ComboBox::from_label("Display Value")
            .selected_text(value.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    value,
                    DataValues::ValueCourseOverGround(CourseOverGround::default()),
                    "COG",
                );
                ui.selectable_value(
                    value,
                    DataValues::ValueSpeedOverGround(SpeedOverGround::default()),
                    "SOG",
                );
                ui.selectable_value(
                    value,
                    DataValues::ValueSpeedThroughWater(SpeedThroughWater::default()),
                    "STW",
                );
            });
        value.add_config(ui);
    }
}

impl Layout for SingleValueLayout {
    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator) {
        ui.group(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let current_stw = self.value.formatted_value(communicator);
                    ui.label(RichText::new(current_stw).size(300.0).monospace());
                    ui.horizontal(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(self.value.abbreviation()).size(50.0));
                            ui.label(RichText::new(self.value.unit_name()).size(100.0));
                        });
                    });
                });
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(self.value.name()).size(150.0));
                });
            });
            ui.set_min_size(Vec2::new(300.0, 150.0));
        });
    }
}
