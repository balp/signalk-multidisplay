use crate::communication::{SignalKCommunicator, SignalKError};
use egui::Ui;

// Garmins data displays:
//  AIR - Air Temperature
//  AWA - Apparent Wind Angle
//  AWS - Apparent Wind Speed
//  BAR - Barometer
//  BAT - Battery voltage
//  BSP - Boat Speed
//  BTW - Direction from location to designation
//  COG - Course over ground
//  CTS - Course to steer
//  DIS - Distance traveled
//  DPT - Depth of water
//  DRF - Speed of current
//  DTW - Distance to waypoint
//  ELV - Altitude
//  ERR - Error of current position
//  GWD - Direction of wind relative ground
//  GWS - Speed of wind relative ground
//  HDG - The direction the boat points
//  ODO - Running tally of distance
//  OTH - Opposite track direction
//  POS - Current position
//  RACE - Race-timer
//  REF - A steer pilot reference
//  RUD - Rudder angle
//  SEA - Temperature of sea water
//  SOG - Speed over ground
//  STR - The steep pilot
//  TRP - A running tally of distance travel since last reset
//  TWA - True wind angle from bow
//  TWD - True wind direction rel north
//  TWS - True wind speed relative vessel
//  UTC - Universal time coordinated
//  VMG - Speed towards designation
//  WND - Velocity made good upwind
//  XTE - Cross track error

#[derive(Debug, PartialEq)]
pub enum DataValues {
    ValueSpeedThroughWater(SpeedThroughWater),
    ValueSpeedOverGround(SpeedOverGround),
    ValueCourseOverGround(CourseOverGround),
}

impl DataValues {
    pub fn abbreviation(&self) -> String {
        match self {
            DataValues::ValueSpeedThroughWater(value) => value.abbreviation.to_string(),
            DataValues::ValueSpeedOverGround(value) => value.abbreviation.to_string(),
            DataValues::ValueCourseOverGround(value) => value.abbreviation.to_string(),
        }
    }

    pub fn add_config(&mut self, ui: &mut Ui) {
        match self {
            DataValues::ValueSpeedThroughWater(layout) => layout.add_config(ui),
            DataValues::ValueSpeedOverGround(layout) => layout.add_config(ui),
            DataValues::ValueCourseOverGround(layout) => layout.add_config(ui),
        }
    }

    pub fn formatted_value(&self, communicator: &SignalKCommunicator) -> String {
        match &self {
            DataValues::ValueSpeedThroughWater(value) => value.fmt_value(communicator),
            DataValues::ValueSpeedOverGround(value) => value.fmt_value(communicator),
            DataValues::ValueCourseOverGround(value) => value.fmt_value(communicator),
        }
    }

    pub fn name(&self) -> String {
        match &self.value {
            DataValues::ValueSpeedThroughWater(value) => value.name.to_string(),
            DataValues::ValueSpeedOverGround(value) => value.name.to_string(),
            DataValues::ValueCourseOverGround(value) => value.name.to_string(),
        }
    }

    pub fn unit_name(&self) -> String {
        match &self {
            DataValues::ValueSpeedThroughWater(value) => value.display_unit.abbreviation(),
            DataValues::ValueSpeedOverGround(value) => value.display_unit.abbreviation(),
            DataValues::ValueCourseOverGround(value) => value.display_unit.abbreviation(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SpeedUnit {
    MeterPerSecond,
    Knot,
    MilesPerHour,
    KilometerPerHour,
}

impl SpeedUnit {
    pub(crate) fn abbreviation(&self) -> String {
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
                ui.selectable_value(
                    self,
                    SpeedUnit::MeterPerSecond,
                    SpeedUnit::MeterPerSecond.abbreviation(),
                );
                ui.selectable_value(self, SpeedUnit::Knot, SpeedUnit::Knot.abbreviation());
                ui.selectable_value(
                    self,
                    SpeedUnit::MilesPerHour,
                    SpeedUnit::MilesPerHour.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    SpeedUnit::KilometerPerHour,
                    SpeedUnit::KilometerPerHour.abbreviation(),
                );
            });
    }
    fn format(&self, value: Result<Option<f64>, SignalKError>) -> String {
        match value {
            Ok(val) => match val {
                None => "  -.-".to_owned(),
                Some(value) => match self {
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
                },
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
    pub(crate) fn abbreviation(&self) -> String {
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
                ui.selectable_value(
                    self,
                    AngularUnit::Degrees,
                    AngularUnit::Degrees.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    AngularUnit::Radians,
                    AngularUnit::Radians.abbreviation(),
                );
            });
    }

    fn format(&self, value: Result<Option<f64>, SignalKError>) -> String {
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
    pub(crate) name: String,
    pub(crate) abbreviation: String,
    pub(crate) display_unit: SpeedUnit,
}

impl SpeedThroughWater {
    pub(crate) fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let stw = communicator.get_stw_from_signalk();
        self.display_unit.format(stw)
    }

    pub(crate) fn add_config(&mut self, ui: &mut Ui) {
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
    pub(crate) name: String,
    pub(crate) abbreviation: String,
    pub(crate) display_unit: SpeedUnit,
}

impl SpeedOverGround {
    pub(crate) fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let sog = communicator.get_sog_from_signalk();
        self.display_unit.format(sog)
    }

    pub(crate) fn add_config(&mut self, ui: &mut Ui) {
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
    pub(crate) name: String,
    pub(crate) abbreviation: String,
    pub(crate) display_unit: AngularUnit,
}

impl CourseOverGround {
    pub(crate) fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let stw = communicator.get_cog_from_signalk();
        self.display_unit.format(stw)
    }
    pub(crate) fn add_config(&mut self, ui: &mut Ui) {
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
