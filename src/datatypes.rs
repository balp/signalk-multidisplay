use crate::communication::SignalKCommunicator;
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
    SpeedThroughWater(SpeedThroughWater),
    SpeedOverGround(SpeedOverGround),
    CourseOverGround(CourseOverGround),
    WaterTemperature(WaterTemperature),
}

impl DataValues {
    pub fn abbreviation(&self) -> String {
        match self {
            DataValues::SpeedThroughWater(value) => value.abbreviation.to_string(),
            DataValues::SpeedOverGround(value) => value.abbreviation.to_string(),
            DataValues::CourseOverGround(value) => value.abbreviation.to_string(),
            DataValues::WaterTemperature(value) => value.abbreviation.to_string(),
        }
    }

    pub fn add_config(&mut self, index: usize, ui: &mut Ui) {
        match self {
            DataValues::SpeedThroughWater(value) => value.add_config(index, ui),
            DataValues::SpeedOverGround(value) => value.add_config(index, ui),
            DataValues::CourseOverGround(value) => value.add_config(index, ui),
            DataValues::WaterTemperature(value) => value.add_config(index, ui),
        }
    }

    pub fn formatted_value(&self, communicator: &SignalKCommunicator) -> String {
        match &self {
            DataValues::SpeedThroughWater(value) => value.fmt_value(communicator),
            DataValues::SpeedOverGround(value) => value.fmt_value(communicator),
            DataValues::CourseOverGround(value) => value.fmt_value(communicator),
            DataValues::WaterTemperature(value) => value.fmt_value(communicator),
        }
    }

    pub fn name(&self) -> String {
        match &self {
            DataValues::SpeedThroughWater(value) => value.name.to_string(),
            DataValues::SpeedOverGround(value) => value.name.to_string(),
            DataValues::CourseOverGround(value) => value.name.to_string(),
            DataValues::WaterTemperature(value) => value.name.to_string(),
        }
    }

    pub fn unit_name(&self) -> String {
        match &self {
            DataValues::SpeedThroughWater(value) => value.display_unit.abbreviation(),
            DataValues::SpeedOverGround(value) => value.display_unit.abbreviation(),
            DataValues::CourseOverGround(value) => value.display_unit.abbreviation(),
            DataValues::WaterTemperature(value) => value.display_unit.abbreviation(),
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
    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        egui::ComboBox::new(format!("data_type_{}", index), "Unit")
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
    fn format(&self, value: Result<f64, signalk::SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                SpeedUnit::MeterPerSecond => {
                    format!("{:5.2}", val)
                }
                SpeedUnit::Knot => {
                    let display_value = val * 3600. / 1851.85;
                    format!("{:5.1}", display_value)
                }
                SpeedUnit::MilesPerHour => {
                    let display_value = val * 3600. / 1609.344;
                    format!("{:5.1}", display_value)
                }
                SpeedUnit::KilometerPerHour => {
                    let display_value = val * 3.600;
                    format!("{:5.2}", display_value)
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
    pub(crate) fn abbreviation(&self) -> String {
        match self {
            AngularUnit::Radians => "rad".to_string(),
            AngularUnit::Degrees => "deg".to_string(),
        }
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        egui::ComboBox::new(format!("angular_{}", index), "Unit")
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

    fn format(&self, value: Result<f64, signalk::SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                AngularUnit::Radians => {
                    format!("{:5.3}", val)
                }
                AngularUnit::Degrees => {
                    let display_value = val * 180. / std::f64::consts::PI;
                    format!("{:3.0}", display_value)
                }
            },
            Err(_) => "-----".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    pub(crate) fn abbreviation(&self) -> String {
        match self {
            TemperatureUnit::Celsius => "°C".to_string(),
            TemperatureUnit::Fahrenheit => "°F".to_string(),
            TemperatureUnit::Kelvin => "°K".to_string(),
        }
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        egui::ComboBox::new(format!("angular_{}", index), "Unit")
            .selected_text(self.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    self,
                    TemperatureUnit::Celsius,
                    TemperatureUnit::Celsius.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    TemperatureUnit::Fahrenheit,
                    TemperatureUnit::Fahrenheit.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    TemperatureUnit::Kelvin,
                    TemperatureUnit::Kelvin.abbreviation(),
                );
            });
    }

    fn format(&self, value: Result<f64, signalk::SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                TemperatureUnit::Celsius => {
                    let display_value = val - 273.15;
                    format!("{:4.1}", display_value)
                }
                TemperatureUnit::Fahrenheit => {
                    let display_value = 9.0 / 5.0 * (val - 273.15) + 32.0;
                    format!("{:4.1}", display_value)
                }
                TemperatureUnit::Kelvin => {
                    format!("{:5.1}", val)
                }
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
        let stw = communicator.get_f64_for_path("self.navigation.speedThroughWater".to_string());
        self.display_unit.format(stw)
    }

    pub(crate) fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
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
        let sog = communicator.get_f64_for_path("self.navigation.speedOverGround".to_string());
        self.display_unit.format(sog)
    }

    pub(crate) fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
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
        let mut cog =
            communicator.get_f64_for_path("self.navigation.courseOverGroundMagnetic".to_string());
        if cog.is_err() {
            cog = communicator.get_f64_for_path("self.navigation.courseOverGroundTrue".to_string());
        }
        self.display_unit.format(cog)
    }
    pub(crate) fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
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

#[derive(Debug, PartialEq)]
pub struct WaterTemperature {
    pub(crate) name: String,
    pub(crate) abbreviation: String,
    pub(crate) display_unit: TemperatureUnit,
}

impl WaterTemperature {
    pub(crate) fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let temp = communicator.get_f64_for_path("self.environment.water.temperature".to_string());
        self.display_unit.format(temp)
    }
    pub(crate) fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }
}

impl Default for WaterTemperature {
    fn default() -> Self {
        Self {
            name: "Water Temperature".to_string(),
            abbreviation: "SEA".to_string(),
            display_unit: TemperatureUnit::Celsius,
        }
    }
}
