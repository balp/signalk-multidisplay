use crate::app::AngularUnit::{Degrees, Radians};
use crate::communication::SignalKCommunicator;
use eframe::egui;
use eframe::egui::{RichText, Vec2};
use egui::Ui;
use ehttp;
use log::debug;
use serde_json;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    server: String,
    view_config: bool,
    #[serde(skip)]
    communicator: Option<SignalKCommunicator>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            server: "http://192.168.1.22:3000/signalk".to_owned(),
            view_config: false,
            communicator: None,
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
                ui.heading("Side Panel");

                ui.horizontal(|ui| {
                    ui.label("Server Address: ");
                    ui.text_edit_singleline(server);
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        egui::warn_if_debug_build(ui);
                    });
                });
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SignalK Multidisplay");
            let tmp = SingleValueLayout::default();
            if let Some(ref comm) = self.communicator {
                tmp.draw_ui(ui, comm);
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


pub enum SpeedUnit {
    MeterPerSecond,
    Knot,
    MilesPerHour,
    KilometerPerHour,
}

pub enum AngularUnit {
    Radians,
    Degrees,
}


pub struct SpeedThroughWater {
    name: String,
    unit_name: String,
    abbreviation: String,
}

impl SpeedThroughWater {
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let stw = communicator.get_stw_from_signalk();
        match stw {
            Ok(val) => match val {
                None => "  -.-".to_owned(),
                Some(value) => {
                    format!("{:5.1}", value)
                }
            },
            Err(_) => "-----".to_owned(),
        }
    }
}

impl Default for SpeedThroughWater {
    fn default() -> Self {
        Self {
            name: "Water Speed".to_string(),
            unit_name: "m/s".to_string(),
            abbreviation: "STW".to_string(),
        }
    }
}

pub struct SpeedOverGround {
    name: String,
    unit_name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl SpeedOverGround {
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let sog = communicator.get_sog_from_signalk();
        match sog {
            Ok(val) => match val {
                None => "  -.-".to_owned(),
                Some(value) => {
                    match self.display_unit {
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
    fn set_display_unit(&mut self, unit: SpeedUnit) {
        match unit {
            SpeedUnit::MeterPerSecond => { self.unit_name = "m/s".to_string(); }
            SpeedUnit::Knot => { self.unit_name = "kn".to_string(); }
            SpeedUnit::MilesPerHour => { self.unit_name = "mph".to_string(); }
            SpeedUnit::KilometerPerHour => { self.unit_name = "km/h".to_string(); }
        }
        self.display_unit = unit;
    }
}

impl Default for SpeedOverGround {
    fn default() -> Self {
        Self {
            name: "Speed Ground".to_string(),
            unit_name: "kn".to_string(),
            abbreviation: "SOG".to_string(),
            display_unit: SpeedUnit::Knot,
        }
    }
}

pub struct CourseOverGround {
    name: String,
    unit_name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl CourseOverGround {
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let mut stw = communicator.get_cog_from_signalk();
        match stw {
            Ok(val) => match val {
                None => "  -.-".to_owned(),
                Some(value) => match self.display_unit {
                    Radians => {
                        format!("{:5.3}", value)
                    }
                    Degrees => {
                        let display_value = value * 180. / std::f64::consts::PI;
                        format!("{:5.1}", display_value)
                    }
                },
            },
            Err(_) => "-----".to_owned(),
        }
    }
    fn set_display_unit(&mut self, unit: AngularUnit) {
        match unit {
            Radians => {
                self.unit_name = "rad".to_string();
            }
            Degrees => {
                self.unit_name = "deg".to_string();
            }
        }
        self.display_unit = unit;
    }
}

impl Default for CourseOverGround {
    fn default() -> Self {
        Self {
            name: "Course Over Ground".to_string(),
            unit_name: "deg".to_string(),
            abbreviation: "COG".to_string(),
            display_unit: Degrees,
        }
    }
}

pub trait Layout {
    fn draw_ui(self, ui: &mut Ui, communicator: &SignalKCommunicator);
}


// Layout variants
// - Single Value
// - Dual Value
// - Triple Value
// - Four values


#[derive(Default)]
pub struct SingleValueLayout {
    value: SpeedOverGround,
}

impl SingleValueLayout {
    fn fmt_stw(&self, communicator: &SignalKCommunicator) -> String {
        self.value.fmt_value(communicator)
    }
}

impl Layout for SingleValueLayout {
    fn draw_ui(self, ui: &mut Ui, communicator: &SignalKCommunicator) {
        ui.group(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let current_stw = self.fmt_stw(communicator);
                    ui.label(RichText::new(current_stw).size(300.0).monospace());
                    ui.horizontal(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(self.value.abbreviation.as_str()).size(50.0));
                            ui.label(RichText::new(self.value.unit_name.as_str()).size(100.0));
                        });
                    });
                });
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(self.value.name.as_str()).size(150.0));
                });
            });
            ui.set_min_size(Vec2::new(300.0, 150.0));
        });
    }
}
