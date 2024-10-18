use datavalue_derive::DataValue;
use egui::Ui;

use crate::communication::SignalKCommunicator;
use crate::dataunits::{
    AngularUnit, DataUnit, DateTimeUnit, DistanceUnit, PositionUnit, PressureUnit, SpeedUnit,
    TemperatureUnit, VoltageUnit,
};

pub trait DataValue {
    fn name(&self) -> String;
    fn unit_name(&self) -> String;
    fn abbreviation(&self) -> String;
    fn add_config(&mut self, index: usize, ui: &mut Ui);
    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String;
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.outside.temperature")]
pub struct AirTemperature {
    name: String,
    abbreviation: String,
    display_unit: TemperatureUnit,
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

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.wind.angleApparent")]
pub struct ApparentWindAngle {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
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

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.wind.speedApparent")]
pub struct ApparentWindSpeed {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
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

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.outside.pressure")]
pub struct Barometer {
    name: String,
    abbreviation: String,
    display_unit: PressureUnit,
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
// #[data_value(data_path = "self.electrical.batteries.house.voltage")]
pub struct Battery {
    name: String,
    abbreviation: String,
    display_unit: VoltageUnit,
    path: String,
}

impl DataValue for Battery {
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
        let Self { path, .. } = self;
        ui.vertical(|ui| {
            ui.label("Battery path: ");
            ui.text_edit_singleline(path);
        });
        self.display_unit.add_config(index, ui);
    }

    fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
        let temp = communicator.get_f64_for_path(self.path.clone());
        self.display_unit.format(temp)
    }
}

impl Default for Battery {
    fn default() -> Self {
        Self {
            name: "Battery".to_string(),
            abbreviation: "BAT".to_string(),
            display_unit: VoltageUnit::Volt,
            path: "self.electrical.batteries.house.voltage".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.course.nextPoint.bearingTrue")]
pub struct BearingTrue {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl Default for BearingTrue {
    fn default() -> Self {
        Self {
            name: "Bearing True".to_string(),
            abbreviation: "BTW".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.log")]
pub struct DistanceTraveled {
    name: String,
    abbreviation: String,
    display_unit: DistanceUnit,
}

impl Default for DistanceTraveled {
    fn default() -> Self {
        Self {
            name: "Distance traveled".to_string(),
            abbreviation: "DIS".to_string(),
            display_unit: DistanceUnit::NauticalMile,
        }
    }
}

#[derive(Debug, PartialEq)]
// #[data_value(data_path = "self.environment.depth.belowSurface")]
pub struct DepthOfWater {
    name: String,
    abbreviation: String,
    display_unit: DistanceUnit,
    path: String,
}
impl DataValue for DepthOfWater {
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
        let temp = communicator.get_f64_for_path(self.path.clone());
        self.display_unit.format(temp)
    }
}
impl Default for DepthOfWater {
    fn default() -> Self {
        Self {
            name: "Depth Of Water".to_string(),
            abbreviation: "DPT".to_string(),
            display_unit: DistanceUnit::Meters,
            path: "environment.depth.belowKeel".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.current.drift")]
pub struct SpeedOfCurrent {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl Default for SpeedOfCurrent {
    fn default() -> Self {
        Self {
            name: "Speed of current".to_string(),
            abbreviation: "DRF".to_string(),
            display_unit: SpeedUnit::Knot,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.course.nextPoint.distance")]
pub struct DistanceToWaypoint {
    name: String,
    abbreviation: String,
    display_unit: DistanceUnit,
}

impl Default for DistanceToWaypoint {
    fn default() -> Self {
        Self {
            name: "Distance To Waypoint".to_string(),
            abbreviation: "DTW".to_string(),
            display_unit: DistanceUnit::NauticalMile,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.position.altitude")]
pub struct Altitude {
    name: String,
    abbreviation: String,
    display_unit: DistanceUnit,
}

impl Default for Altitude {
    fn default() -> Self {
        Self {
            name: "Altitude".to_string(),
            abbreviation: "ELV".to_string(),
            display_unit: DistanceUnit::Meters,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.wind.speedOverGround")]
pub struct DirectionOfWindRelativeGround {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl Default for DirectionOfWindRelativeGround {
    fn default() -> Self {
        Self {
            name: "Direction Of Wind Relative Ground".to_string(),
            abbreviation: "GWD".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.headingTrue")]
pub struct HeadingTrue {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl Default for HeadingTrue {
    fn default() -> Self {
        Self {
            name: "The direction the boat points".to_string(),
            abbreviation: "HDG".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.log")]
pub struct Odometer {
    name: String,
    abbreviation: String,
    display_unit: DistanceUnit,
}

impl Default for Odometer {
    fn default() -> Self {
        Self {
            name: "Running tally of distance".to_string(),
            abbreviation: "ODO".to_string(),
            display_unit: DistanceUnit::NauticalMile,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.position")]
pub struct Trip {
    name: String,
    abbreviation: String,
    display_unit: DistanceUnit,
}

impl Default for Trip {
    fn default() -> Self {
        Self {
            name: "A running tally of distance travel since last reset".to_string(),
            abbreviation: "TRP".to_string(),
            display_unit: DistanceUnit::NauticalMile,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.wind.angleTrueGround")]
pub struct TrueWindAngleFromBow {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl Default for crate::datavalues::TrueWindAngleFromBow {
    fn default() -> Self {
        Self {
            name: "True wind angle from bow".to_string(),
            abbreviation: "TWA".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.wind.directionTrue")]
pub struct TrueWindDirectionRelNorth {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl Default for crate::datavalues::TrueWindDirectionRelNorth {
    fn default() -> Self {
        Self {
            name: "True wind direction rel north".to_string(),
            abbreviation: "TWD".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.log")]
pub struct Position {
    name: String,
    abbreviation: String,
    display_unit: PositionUnit,
}
impl Position {
    pub fn fmt_position(&self, communicator: &SignalKCommunicator) -> String {
        if let Some(ref storage) = communicator.signalk_data {
            let sk_data = storage.get();
            if let Some(vessel) = sk_data.get_self() {
                if let Some(navigation) = &vessel.navigation {
                    return self.display_unit.format_pos(&navigation.position);
                }
            }
        }
        "--.---\n--.---".to_string()
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            name: "Current Position".to_string(),
            abbreviation: "POS".to_string(),
            display_unit: PositionUnit::DecimalDegrees,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.steering.rudderAngle")]
pub struct RudderAngle {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
}

impl Default for crate::datavalues::RudderAngle {
    fn default() -> Self {
        Self {
            name: "Rudder angle".to_string(),
            abbreviation: "RUD".to_string(),
            display_unit: AngularUnit::Degrees,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.wind.directionTrue")]
pub struct SpeedOfWindRelativeGround {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl Default for SpeedOfWindRelativeGround {
    fn default() -> Self {
        Self {
            name: "Speed Of Wind Relative Ground".to_string(),
            abbreviation: "GWS".to_string(),
            display_unit: SpeedUnit::MeterPerSecond,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.speedThroughWater")]
pub struct SpeedThroughWater {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl Default for SpeedThroughWater {
    fn default() -> Self {
        Self {
            name: "Water Speed".to_string(),
            abbreviation: "STW".to_string(),
            display_unit: SpeedUnit::Knot,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.speedThroughWater")]
pub struct TrueWindSpeed {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl Default for TrueWindSpeed {
    fn default() -> Self {
        Self {
            name: "True wind speed relative vessel".to_string(),
            abbreviation: "TWS".to_string(),
            display_unit: SpeedUnit::Knot,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.speedOverGround")]
pub struct SpeedOverGround {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
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

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.courseOverGroundTrue")]
pub struct CourseOverGround {
    name: String,
    abbreviation: String,
    display_unit: AngularUnit,
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

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.environment.water.temperature")]
pub struct WaterTemperature {
    name: String,
    abbreviation: String,
    display_unit: TemperatureUnit,
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

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.course.nextPoint.velocityMadeGood")]
pub struct VelocityMadeGood {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl Default for VelocityMadeGood {
    fn default() -> Self {
        Self {
            name: "Velocity made good".to_string(),
            abbreviation: "VMG".to_string(),
            display_unit: SpeedUnit::Knot,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.course.nextPoint.velocityMadeGood")]
pub struct VelocityMadeGoodUpwind {
    name: String,
    abbreviation: String,
    display_unit: SpeedUnit,
}

impl Default for VelocityMadeGoodUpwind {
    fn default() -> Self {
        Self {
            name: "Velocity made good Upwind".to_string(),
            abbreviation: "WND".to_string(),
            display_unit: SpeedUnit::Knot,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "navigation.course.calcValues.crossTrackError")]
pub struct CrossTrackError {
    name: String,
    abbreviation: String,
    display_unit: DistanceUnit,
}

impl Default for CrossTrackError {
    fn default() -> Self {
        Self {
            name: "Cross track error".to_string(),
            abbreviation: "XTE".to_string(),
            display_unit: DistanceUnit::Meters,
        }
    }
}

#[derive(Debug, PartialEq, DataValue)]
#[data_value(data_path = "self.navigation.datetime")]
pub struct UniversalTimeCoordinated {
    name: String,
    abbreviation: String,
    display_unit: DateTimeUnit,
}

impl Default for UniversalTimeCoordinated {
    fn default() -> Self {
        Self {
            name: "Universal time coordinated".to_string(),
            abbreviation: "UTC".to_string(),
            display_unit: DateTimeUnit::Default,
        }
    }
}

impl crate::datavalues::UniversalTimeCoordinated {
    pub fn fmt_time(&self, _communicator: &SignalKCommunicator) -> String {
        "hh:mm:ss".to_string()
    }
}
