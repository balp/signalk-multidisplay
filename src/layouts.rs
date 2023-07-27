use eframe::egui;
use egui::{RichText, Ui};

use crate::communication::SignalKCommunicator;
use crate::datatypes::{
    CourseOverGround, DataValues, SpeedOverGround, SpeedThroughWater, WaterTemperature,
};

/// The different types of layout that a page can have.
pub enum Layout {
    SingleValue(SingleValueLayout),
    DualValues(DualValuesLayout),
    // TripleValues,
    // FourValues,
}

impl LayoutComponent for Layout {
    fn add_config(&mut self, ui: &mut Ui) {
        match self {
            Self::SingleValue(layout) => layout.add_config(ui),
            Self::DualValues(layout) => layout.add_config(ui),
        }
    }

    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator) {
        match self {
            Self::SingleValue(layout) => layout.draw_ui(ui, communicator),
            Self::DualValues(layout) => layout.draw_ui(ui, communicator),
        }
    }
}

/// Items that can be displayed in the UI
pub trait LayoutComponent {
    /// This will draw the needed configuration parts for the component
    fn add_config(&mut self, ui: &mut Ui);

    /// This will draw the main ui of the component
    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator);
}

/// This is a component that can show a single value on the screen.
pub struct SingleValueLayout {
    id: usize,
    value: DataValues,
}

impl SingleValueLayout {
    pub fn new(id: usize, value: DataValues) -> Self {
        Self { id, value }
    }
}

impl Default for SingleValueLayout {
    fn default() -> Self {
        Self {
            id: 0,
            value: DataValues::CourseOverGround(CourseOverGround::default()),
        }
    }
}

impl LayoutComponent for SingleValueLayout {
    fn add_config(&mut self, ui: &mut Ui) {
        let Self { id, value } = self;
        ui.label("Single Value Layout");
        egui::ComboBox::new(format!("SingleValueLayout: {}", id), "Value")
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
                ui.selectable_value(
                    value,
                    DataValues::WaterTemperature(WaterTemperature::default()),
                    "SEA",
                );
            });
        value.add_config(*id, ui);
    }

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

pub struct DualValuesLayout {
    id: usize,
    top_value: DataValues,
    bottom_value: DataValues,
}

impl DualValuesLayout {
    pub fn new(id: usize, top_value: DataValues, bottom_value: DataValues) -> Self {
        Self {
            id,
            top_value,
            bottom_value,
        }
    }
}

impl LayoutComponent for DualValuesLayout {
    fn add_config(&mut self, ui: &mut Ui) {
        let Self {
            id,
            top_value,
            bottom_value,
        } = self;
        ui.label("Dual Value Layout");
        egui::ComboBox::new(format!("DualValuesLayout_top_{}", id), "Top Value")
            .selected_text(top_value.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    top_value,
                    DataValues::CourseOverGround(CourseOverGround::default()),
                    "COG",
                );
                ui.selectable_value(
                    top_value,
                    DataValues::SpeedOverGround(SpeedOverGround::default()),
                    "SOG",
                );
                ui.selectable_value(
                    top_value,
                    DataValues::SpeedThroughWater(SpeedThroughWater::default()),
                    "STW",
                );
                ui.selectable_value(
                    top_value,
                    DataValues::WaterTemperature(WaterTemperature::default()),
                    "SEA",
                );
            });
        top_value.add_config(*id, ui);
        egui::ComboBox::new(format!("DualValuesLayout_bottom_{}", id), "Bottom Value")
            .selected_text(bottom_value.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    bottom_value,
                    DataValues::CourseOverGround(CourseOverGround::default()),
                    "COG",
                );
                ui.selectable_value(
                    bottom_value,
                    DataValues::SpeedOverGround(SpeedOverGround::default()),
                    "SOG",
                );
                ui.selectable_value(
                    bottom_value,
                    DataValues::SpeedThroughWater(SpeedThroughWater::default()),
                    "STW",
                );
                ui.selectable_value(
                    bottom_value,
                    DataValues::WaterTemperature(WaterTemperature::default()),
                    "SEA",
                );
            });
        bottom_value.add_config(*id + 1, ui);
    }

    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator) {
        const SIZE_OF_MAIN_TEXT: f32 = 150.0;
        const SIZE_OF_ABBREVIATION: f32 = 25.0;
        const SIZE_OF_UNIT: f32 = 50.0;
        ui.group(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let value = self.top_value.formatted_value(communicator);
                    ui.label(RichText::new(value).size(SIZE_OF_MAIN_TEXT).monospace());
                    ui.horizontal(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(self.top_value.abbreviation())
                                    .size(SIZE_OF_ABBREVIATION),
                            );
                            ui.label(RichText::new(self.top_value.unit_name()).size(SIZE_OF_UNIT));
                        });
                    });
                });
            });
        });
        ui.group(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let value = self.bottom_value.formatted_value(communicator);
                    ui.label(RichText::new(value).size(SIZE_OF_MAIN_TEXT).monospace());
                    ui.horizontal(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(self.bottom_value.abbreviation())
                                    .size(SIZE_OF_ABBREVIATION),
                            );
                            ui.label(
                                RichText::new(self.bottom_value.unit_name()).size(SIZE_OF_UNIT),
                            );
                        });
                    });
                });
            });
        });
    }
}
