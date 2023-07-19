use crate::communication::{SignalKCommunicator, SignalKError};
use eframe::egui;
use eframe::egui::{RichText, Vec2};
use egui::Ui;
use log::debug;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    server: String,
    view_config: bool,
    #[serde(skip)]
    communicator: Option<SignalKCommunicator>,
    #[serde(skip)]
    layout: SingleValueLayout,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            server: "http://192.168.1.22:3000/signalk".to_owned(),
            view_config: false,
            communicator: None,
            layout: SingleValueLayout::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            let mut restored_app: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            debug!("Restore object with server {}", restored_app.server);
            restored_app
        } else {
            debug!("Creating new instance.");
            Self::default()
        };
        let mut communicator = SignalKCommunicator::default();
        communicator.set_up_server_connections(app.server.to_string());
        app.communicator = Some(communicator);
        app
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        if let Some(ref mut sk_com) = self.communicator {
            sk_com.handle_data(ctx);
        }

        let Self {
            server,
            view_config,
            layout,
            ..
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Config").clicked() {
                        *view_config = !*view_config;
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        // Side panel for config? Maybe a different view?
        if *view_config {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Configuration");

                ui.vertical(|ui| {
                    ui.label("Server Address: ");
                    ui.text_edit_singleline(server);
                });

                ui.add_space(6.);
                layout.add_config(ui);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        egui::warn_if_debug_build(ui);
                    });
                });
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SignalK Multidisplay");
            if let Some(ref comm) = self.communicator {
                layout.draw_ui(ui, comm);
            }
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// Garmins data displays:
//  AIR - Air Temperature
//  AWA - Apparent Wind Angle
//  AWS - Apparent Wind Speed
//  BAR - Barmometer
//  BAT - Battery voltage
//  BSP - Boat Speed
//  BTW - Direction from location to desination
//  COG - Course over ground
//  CTS - Course to steer
//  DIS - Distance traveled
//  DPT - Depth of water
//  DRF - Speed of current
//  DTW - Distance to waypoint
//  ELV - Altitude
//  ERR - Error of current position
//  GWD - Direction of wind relative ground
//  GWS - Spped of wind relative ground
//  HDG - The direction boat ppoints
//  ODO - Running tally of distance
//  OTH - Opposite track direction
//  POS - Current position
//  RACE - Rracetimer
//  REF - A steer pilot reference
//  RUD - Rudder angle
//  SEA - Teamerature of sea water
//  SOG - Speed over ground
//  STR - The steep pilot
//  TRP - A running tally of distance travel since last reset
//  TWA - True wind angle from bow
//  TWD - True wind diection rel north
//  TWS - True wind speed relative vessel
//  UTC - Universal time coordinated
//  VMG - Speed towards desination
//  WND - Velocity made good upwind
//  XTE - Cross track error


#[derive(Debug, PartialEq)]
pub enum SpeedUnit {
    MeterPerSecond,
    Knot,
    MilesPerHour,
    KilometerPerHour,
}

impl SpeedUnit {
    fn abbreviation(&self) -> String {
        match self {
            SpeedUnit::MeterPerSecond => "m/s".to_string(),
            SpeedUnit::Knot => "kn".to_string(),
            SpeedUnit::MilesPerHour => "mph".to_string(),
            SpeedUnit::KilometerPerHour => "km/h".to_string(),
        }
    }
    fn add_config(&mut self, ui: &mut Ui) {
        ui.label("Unit of data");
        egui::ComboBox::from_label("Data type")
            .selected_text(self.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(self, SpeedUnit::MeterPerSecond, SpeedUnit::MeterPerSecond.abbreviation());
                ui.selectable_value(self, SpeedUnit::Knot, SpeedUnit::Knot.abbreviation());
                ui.selectable_value(self, SpeedUnit::MilesPerHour, SpeedUnit::MilesPerHour.abbreviation());
                ui.selectable_value(self, SpeedUnit::KilometerPerHour, SpeedUnit::KilometerPerHour.abbreviation());
            });
    }
    fn format(&self, value: Result<Option<f64>,SignalKError>) -> String {
        match value {
            Ok(val) => match val {
                None => "  -.-".to_owned(),
                Some(value) => {
                    match self {
                        SpeedUnit::MeterPerSecond => {
                            format!("{:5.2}", value)
                        }
                        SpeedUnit::Knot => {
                            let display_value = value * 3600. / 1851.85;
                            format!("{:5.1}", display_value)
                        }
                        SpeedUnit::MilesPerHour => {
                            let display_value = value * 3600. / 1609.344;
                            format!("{:5.1}", display_value)
                        }
                        SpeedUnit::KilometerPerHour => {
                            let display_value = value * 3.600;
                            format!("{:5.2}", display_value)
                        }
                    }

                }
            },
            Err(_) => "-----".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AngularUnit {
    Radians,
    Degrees,
}

impl AngularUnit {
    fn abbreviation(&self) -> String {
        match self {
            AngularUnit::Radians => "rad".to_string(),
            AngularUnit::Degrees => "deg".to_string(),
        }
    }

    fn add_config(&mut self, ui: &mut Ui) {
        ui.label("Unit of data");
        egui::ComboBox::from_label("Data type")
            .selected_text(self.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(self, AngularUnit::Degrees, AngularUnit::Degrees.abbreviation());
                ui.selectable_value(self, AngularUnit::Radians, AngularUnit::Radians.abbreviation());
            });
    }

    fn format(&self, value: Result<Option<f64>,SignalKError>) -> String {
        match value {
            Ok(val) => match val {
                None => "  -.-".to_owned(),
                Some(value) => match self {
                    AngularUnit::Radians => {
                        format!("{:5.3}", value)
                    }
                    AngularUnit::Degrees => {
                        let display_value = value * 180. / std::f64::consts::PI;
                        format!("{:5.1}", display_value)
                    }
                },
            },
            Err(_) => "-----".to_owned(),
        }

    }
}


#[derive(Debug, PartialEq)]
pub struct SpeedThroughWater {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl SpeedThroughWater {
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let stw = communicator.get_stw_from_signalk();
        self.display_unit.format(stw)
    }

    fn add_config(&mut self, ui: &mut Ui) {
        self.display_unit.add_config(ui);
    }
}

impl Default for SpeedThroughWater {
    fn default() -> Self {
        Self {
            name: "Water Speed".to_string(),
            abbreviation: "STW".to_string(),
            display_unit: SpeedUnit::MeterPerSecond,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SpeedOverGround {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl SpeedOverGround {
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let sog = communicator.get_sog_from_signalk();
        self.display_unit.format(sog)
    }

    fn add_config(&mut self, ui: &mut Ui) {
        self.display_unit.add_config(ui);
    }
}

impl Default for SpeedOverGround {
    fn default() -> Self {
        Self {
            name: "Speed Ground".to_string(),
            abbreviation: "SOG".to_string(),
            display_unit: SpeedUnit::Knot,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CourseOverGround {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl CourseOverGround {
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let stw = communicator.get_cog_from_signalk();
        self.display_unit.format(stw)
    }
    fn add_config(&mut self, ui: &mut Ui) {
        self.display_unit.add_config(ui);
    }
}

impl Default for CourseOverGround {
    fn default() -> Self {
        Self {
            name: "Course Over Ground".to_string(),
            abbreviation: "COG".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

pub trait Layout {
    fn draw_ui(&self, ui: &mut Ui, communicator: &SignalKCommunicator);
}


// Layout variants
// - Single Value
// - Dual Value
// - Triple Value
// - Four values


#[derive(Debug, PartialEq)]
enum LayoutData {
    LayoutSpeedThroughWater(SpeedThroughWater),
    LayoutSpeedOverGround(SpeedOverGround),
    LayoutCourseOverGround(CourseOverGround),
}

impl LayoutData {
    fn abbreviation(&self) -> String {
        match self {
            LayoutData::LayoutSpeedThroughWater(value) => {
                value.abbreviation.to_string()
            }
            LayoutData::LayoutSpeedOverGround(value) => {
                value.abbreviation.to_string()
            }
            LayoutData::LayoutCourseOverGround(value) => {
                value.abbreviation.to_string()
            }
        }
    }

    fn add_config(&mut self, ui: &mut Ui) {
        match self {
            LayoutData::LayoutSpeedThroughWater(layout) => {
                layout.add_config(ui);
            }
            LayoutData::LayoutSpeedOverGround(layout) => {
                layout.add_config(ui);
            }
            LayoutData::LayoutCourseOverGround(layout) => {
                layout.add_config(ui)
            }
        }
    }
}

pub struct SingleValueLayout {
    value: LayoutData,
}

impl Default for SingleValueLayout {
    fn default() -> Self {
        Self {
            value: LayoutData::LayoutCourseOverGround(CourseOverGround::default()),
        }
    }
}

impl SingleValueLayout {
    fn fmt_stw(&self, communicator: &SignalKCommunicator) -> String {
        match &self.value {
            LayoutData::LayoutSpeedThroughWater(value) => {
                value.fmt_value(communicator)
            }
            LayoutData::LayoutSpeedOverGround(value) => {
                value.fmt_value(communicator)
            }
            LayoutData::LayoutCourseOverGround(value) => {
                value.fmt_value(communicator)
            }
        }
    }
    fn value_abbreviation(&self) -> String {
        self.value.abbreviation()
    }

    fn value_unit_name(&self) -> String {
        match &self.value {
            LayoutData::LayoutSpeedThroughWater(value) => {
                value.display_unit.abbreviation()
            }
            LayoutData::LayoutSpeedOverGround(value) => {
                value.display_unit.abbreviation()
            }
            LayoutData::LayoutCourseOverGround(value) => {
                value.display_unit.abbreviation()
            }
        }
    }

    fn value_name(&self) -> String {
        match &self.value {
            LayoutData::LayoutSpeedThroughWater(value) => {
                value.name.to_string()
            }
            LayoutData::LayoutSpeedOverGround(value) => {
                value.name.to_string()
            }
            LayoutData::LayoutCourseOverGround(value) => {
                value.name.to_string()
            }
        }
    }
    fn add_config(&mut self, ui: &mut Ui) {
        let Self { value } = self;
        ui.label("Data to display");
        egui::ComboBox::from_label("Display Value")
            .selected_text(value.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(value, LayoutData::LayoutCourseOverGround(CourseOverGround::default()), "COG");
                ui.selectable_value(value, LayoutData::LayoutSpeedOverGround(SpeedOverGround::default()), "SOG");
                ui.selectable_value(value, LayoutData::LayoutSpeedThroughWater(SpeedThroughWater::default()), "STW");
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
                    let current_stw = self.fmt_stw(communicator);
                    ui.label(RichText::new(current_stw).size(300.0).monospace());
                    ui.horizontal(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(self.value_abbreviation()).size(50.0));
                            ui.label(RichText::new(self.value_unit_name()).size(100.0));
                        });
                    });
                });
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(self.value_name()).size(150.0));
                });
            });
            ui.set_min_size(Vec2::new(300.0, 150.0));
        });
    }
}
