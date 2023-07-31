// Garmins data displays:
//  AIR - Air Temperature
//  AWA - Apparent Wind Angle
//  AWS - Apparent Wind Speed
//  BAR - Barometer
//  BAT - Battery voltage
//  BSP - Boat Speed aka Speed Through Water
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

use crate::communication::SignalKCommunicator;
use crate::datavalues::{
    AirTemperature, ApparentWindAngle, ApparentWindSpeed, Barometer, CourseOverGround, DataValue,
    SpeedOverGround, SpeedThroughWater, WaterTemperature,
};
use egui::Ui;

#[derive(Debug, PartialEq)]
pub enum DataValues {
    AirTemperature(AirTemperature),
    ApparentWindAngle(ApparentWindAngle),
    ApparentWindSpeed(ApparentWindSpeed),
    Barometer(Barometer),
    SpeedThroughWater(SpeedThroughWater),
    SpeedOverGround(SpeedOverGround),
    CourseOverGround(CourseOverGround),
    WaterTemperature(WaterTemperature),
}

impl DataValues {
    pub fn abbreviation(&self) -> String {
        match self {
            DataValues::SpeedThroughWater(value) => value.abbreviation(),
            DataValues::SpeedOverGround(value) => value.abbreviation(),
            DataValues::CourseOverGround(value) => value.abbreviation(),
            DataValues::WaterTemperature(value) => value.abbreviation(),
            DataValues::AirTemperature(value) => value.abbreviation(),
            DataValues::ApparentWindAngle(value) => value.abbreviation(),
            DataValues::ApparentWindSpeed(value) => value.abbreviation(),
            DataValues::Barometer(value) => value.abbreviation(),
        }
    }

    pub fn add_config(&mut self, index: usize, ui: &mut Ui) {
        match self {
            DataValues::SpeedThroughWater(value) => value.add_config(index, ui),
            DataValues::SpeedOverGround(value) => value.add_config(index, ui),
            DataValues::CourseOverGround(value) => value.add_config(index, ui),
            DataValues::WaterTemperature(value) => value.add_config(index, ui),
            DataValues::AirTemperature(value) => value.add_config(index, ui),
            DataValues::ApparentWindAngle(value) => value.add_config(index, ui),
            DataValues::ApparentWindSpeed(value) => value.add_config(index, ui),
            DataValues::Barometer(value) => value.add_config(index, ui),
        }
    }

    pub fn formatted_value(&self, communicator: &SignalKCommunicator) -> String {
        match &self {
            DataValues::SpeedThroughWater(value) => value.fmt_value(communicator),
            DataValues::SpeedOverGround(value) => value.fmt_value(communicator),
            DataValues::CourseOverGround(value) => value.fmt_value(communicator),
            DataValues::WaterTemperature(value) => value.fmt_value(communicator),
            DataValues::AirTemperature(value) => value.fmt_value(communicator),
            DataValues::ApparentWindAngle(value) => value.fmt_value(communicator),
            DataValues::ApparentWindSpeed(value) => value.fmt_value(communicator),
            DataValues::Barometer(value) => value.fmt_value(communicator),
        }
    }

    pub fn name(&self) -> String {
        match &self {
            DataValues::SpeedThroughWater(value) => value.name(),
            DataValues::SpeedOverGround(value) => value.name(),
            DataValues::CourseOverGround(value) => value.name(),
            DataValues::WaterTemperature(value) => value.name(),
            DataValues::AirTemperature(value) => value.name(),
            DataValues::ApparentWindAngle(value) => value.name(),
            DataValues::ApparentWindSpeed(value) => value.name(),
            DataValues::Barometer(value) => value.name(),
        }
    }

    pub fn unit_name(&self) -> String {
        match &self {
            DataValues::SpeedThroughWater(value) => value.unit_name(),
            DataValues::SpeedOverGround(value) => value.unit_name(),
            DataValues::CourseOverGround(value) => value.unit_name(),
            DataValues::WaterTemperature(value) => value.unit_name(),
            DataValues::AirTemperature(value) => value.unit_name(),
            DataValues::ApparentWindAngle(value) => value.unit_name(),
            DataValues::ApparentWindSpeed(value) => value.unit_name(),
            DataValues::Barometer(value) => value.unit_name(),
        }
    }

    pub fn add_config_values(self: &mut DataValues, ui: &mut Ui) {
        ui.selectable_value(
            self,
            DataValues::AirTemperature(AirTemperature::default()),
            "AIR",
        );
        ui.selectable_value(
            self,
            DataValues::ApparentWindAngle(ApparentWindAngle::default()),
            "AWA",
        );
        ui.selectable_value(
            self,
            DataValues::ApparentWindSpeed(ApparentWindSpeed::default()),
            "AWS",
        );
        ui.selectable_value(self, DataValues::Barometer(Barometer::default()), "BAR");
        ui.selectable_value(
            self,
            DataValues::CourseOverGround(CourseOverGround::default()),
            "COG",
        );
        ui.selectable_value(
            self,
            DataValues::SpeedOverGround(SpeedOverGround::default()),
            "SOG",
        );
        ui.selectable_value(
            self,
            DataValues::SpeedThroughWater(SpeedThroughWater::default()),
            "STW",
        );
        ui.selectable_value(
            self,
            DataValues::WaterTemperature(WaterTemperature::default()),
            "SEA",
        );
    }
}
