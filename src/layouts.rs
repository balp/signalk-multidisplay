use crate::communication::SignalKCommunicator;
use crate::datatypes::{CourseOverGround, DataValues, SpeedOverGround, SpeedThroughWater};
use eframe::egui;
use egui::{RichText, Ui};

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
            value: DataValues::CourseOverGround(CourseOverGround::default()),
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
                    DataValues::CourseOverGround(CourseOverGround::default()),
                    "COG",
                );
                ui.selectable_value(
                    value,
                    DataValues::SpeedOverGround(SpeedOverGround::default()),
                    "SOG",
                );
                ui.selectable_value(
                    value,
                    DataValues::SpeedThroughWater(SpeedThroughWater::default()),
                    "STW",
                );
            });
        value.add_config(ui);
    }
}

impl Layout for SingleValueLayout {
    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator) {
        const SIZE_OF_MAIN_TEXT: f32 = 150.0;
        const SIZE_OF_ABBREVIATION: f32 = 25.0;
        const SIZE_OF_UNIT: f32 = 50.0;
        const SIZE_OF_FULL_NAME: f32 = 75.0;
        ui.group(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let current_stw = self.value.formatted_value(communicator);
                    ui.label(
                        RichText::new(current_stw)
                            .size(SIZE_OF_MAIN_TEXT)
                            .monospace(),
                    );
                    ui.horizontal(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(self.value.abbreviation()).size(SIZE_OF_ABBREVIATION),
                            );
                            ui.label(RichText::new(self.value.unit_name()).size(SIZE_OF_UNIT));
                        });
                    });
                });
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(self.value.name()).size(SIZE_OF_FULL_NAME));
                });
            });
        });
    }
}
