use crate::communication::SignalKCommunicator;
use crate::dataunits::{AngularUnit, DataUnit, PressureUnit, SpeedUnit, TemperatureUnit};
use egui::Ui;

pub trait DataValue {
    fn name(&self) -> String;
    fn unit_name(&self) -> String;
    fn abbreviation(&self) -> String;
    fn add_config(&mut self, index: usize, ui: &mut Ui);
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String;
}


#[derive(Debug, PartialEq)]
pub struct AirTemperature {
    name: String,
    abbreviation: String,
    display_unit: TemperatureUnit,
}

impl DataValue for AirTemperature {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let temp =
            communicator.get_f64_for_path("self.environment.outside.temperature".to_string());
        self.display_unit.format(temp)
    }
}

impl Default for AirTemperature {
    fn default() -> Self {
        Self {
            name: "Air Temperature".to_string(),
            abbreviation: "AIR".to_string(),
            display_unit: TemperatureUnit::Celsius,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ApparentWindAngle {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl DataValue for ApparentWindAngle {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let temp = communicator.get_f64_for_path("self.environment.wind.angleApparent".to_string());
        self.display_unit.format(temp)
    }
}

impl Default for ApparentWindAngle {
    fn default() -> Self {
        Self {
            name: "Apparent Wind Angle".to_string(),
            abbreviation: "AWA".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ApparentWindSpeed {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl DataValue for ApparentWindSpeed {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let temp = communicator.get_f64_for_path("self.environment.wind.angleApparent".to_string());
        self.display_unit.format(temp)
    }
}

impl Default for ApparentWindSpeed {
    fn default() -> Self {
        Self {
            name: "Apparent Wind Speed".to_string(),
            abbreviation: "AWS".to_string(),
            display_unit: SpeedUnit::MeterPerSecond,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Barometer {
    name: String,
    abbreviation: String,
    display_unit: PressureUnit,
}

impl DataValue for Barometer {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let temp = communicator.get_f64_for_path("self.environment.wind.angleApparent".to_string());
        self.display_unit.format(temp)
    }
}

impl Default for Barometer {
    fn default() -> Self {
        Self {
            name: "Barometer".to_string(),
            abbreviation: "BAR".to_string(),
            display_unit: PressureUnit::HectoPascal,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SpeedThroughWater {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl DataValue for SpeedThroughWater {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let stw = communicator.get_f64_for_path("self.environment.outside.pressure".to_string());
        self.display_unit.format(stw)
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

impl DataValue for SpeedOverGround {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let sog = communicator.get_f64_for_path("self.navigation.speedOverGround".to_string());
        self.display_unit.format(sog)
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

impl DataValue for CourseOverGround {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let mut cog =
            communicator.get_f64_for_path("self.navigation.courseOverGroundMagnetic".to_string());
        if cog.is_err() {
            cog = communicator.get_f64_for_path("self.navigation.courseOverGroundTrue".to_string());
        }
        self.display_unit.format(cog)
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
    name: String,
    abbreviation: String,
    display_unit: TemperatureUnit,
}

impl DataValue for WaterTemperature {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn unit_name(&self) -> String {
        self.display_unit.abbreviation()
    }

    fn abbreviation(&self) -> String {
        self.abbreviation.to_string()
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let temp = communicator.get_f64_for_path("self.environment.water.temperature".to_string());
        self.display_unit.format(temp)
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
