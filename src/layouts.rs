use crate::communication::SignalKCommunicator;
use crate::datatypes::{CourseOverGround, DataValues, SpeedOverGround, SpeedThroughWater};
use eframe::egui;
use egui::{RichText, Ui};

/// The different types of layout that a page can have.
pub enum Layout {
    SingleValue(SingleValueLayout),
    DualValues,
    TripleValues,
    FourValues,
}

impl LayoutComponent for Layout {
    fn add_config(&mut self, ui: &mut Ui) {
        if let Self::SingleValue(layout) = self {
            layout.add_config(ui);
        }
    }

    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator) {
        if let Self::SingleValue(layout) = self {
            layout.draw_ui(ui, communicator);
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
    value: DataValues,
}

impl SingleValueLayout {
    pub fn new(value: DataValues) -> Self {
        Self { value }
    }
}

impl Default for SingleValueLayout {
    fn default() -> Self {
        Self {
            value: DataValues::CourseOverGround(CourseOverGround::default()),
        }
    }
}

impl LayoutComponent for SingleValueLayout {
    fn add_config(&mut self, ui: &mut Ui) {
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
