// Garmins data displays:
//  Abbreviation    Name
//  AIR           | Air Temperature                         | + self.environment.outside.temperature
//  AWA           | Apparent Wind Angle                     | + self.environment.wind.angleApparent
//  AWS           | Apparent Wind Speed                     | + self.environment.wind.speedApparent
//  BAR           | Barometer                               | + self.environment.outside.pressure
//  BAT           | Battery voltage                         | self.electrical.batteries.house.voltage
//  BTW           | Direction from location to designation  | self.navigation.course.nextPoint.bearingTrue
//  COG           | Course over ground                      | + self.navigation.courseOverGroundTrue
//  CTS           | Course to steer                         | ??
//  DIS           | Distance traveled                       | self.navigation.log ??
//  DPT           | Depth of water                          | self.environment.depth.belowSurface
//  DRF           | Speed of current                        | self.environment.current.drift
//  DTW           | Distance to waypoint                    | self.navigation.course.nextPoint.distance
//  ELV           | Altitude                                | self.navigation.position.altitude
//  ERR           | Error of current position               | ??
//  GWD           | Direction of wind relative ground       | self.environment.wind.directionTrue
//  GWS           | Speed of wind relative ground           | self.environment.wind.speedOverGround
//  HDG           | The direction the boat points           | self.navigation.headingTrue
//  ODO           | Running tally of distance               | self.navigation.log ??
//  OTH           | Opposite track direction                | ??
//  POS           | Current position                        | self.navigation.position
//  RACE          | Race-timer                              | ??
//  REF           | A steer pilot reference                 | ??
//  RUD           | Rudder angle                            | self.steering.rudderAngle
//  SEA           | Temperature of sea water                | + self.environment.outside.temperature
//  SOG           | Speed over ground                       | + self.navigation.speedOverGround
//  STW           | Boat Speed aka Speed Through Water      | + self.navigation.speedThroughWater
//  STR           | The steep pilot                         | ??
//  TRP           | A running tally of distance travel since last reset | self.navigation.trip.log
//  TWA           | True wind angle from bow                | self.environment.wind.angleTrueGround
//  TWD           | True wind direction rel north           | self.environment.wind.directionTrue
//  TWS           | True wind speed relative vessel         | self.environment.wind.speedTrue
//  UTC           | Universal time coordinated              | self.navigation.datetime
//  VMG           | Speed towards designation               | self.navigation.course.nextPoint.velocityMadeGood
//  WND           | Velocity made good upwind               | ??
//  XTE           | Cross track error                       | self.navigation.course.crossTrackError

use crate::communication::SignalKCommunicator;
use crate::datavalues::{
    AirTemperature, ApparentWindAngle, ApparentWindSpeed, Barometer, Battery, BearingTrue,
    CourseOverGround, DataValue, DistanceTraveled, SpeedOverGround, SpeedThroughWater,
    WaterTemperature,
};
use egui::Ui;

#[derive(Debug, PartialEq)]
pub enum DataValues {
    AirTemperature(AirTemperature),
    ApparentWindAngle(ApparentWindAngle),
    ApparentWindSpeed(ApparentWindSpeed),
    Barometer(Barometer),
    Battery(Battery),
    BearingTrue(BearingTrue),
    DistanceTraveled(DistanceTraveled),
    CourseOverGround(CourseOverGround),
    SpeedThroughWater(SpeedThroughWater),
    SpeedOverGround(SpeedOverGround),
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
            DataValues::Battery(value) => value.abbreviation(),
            DataValues::BearingTrue(value) => value.abbreviation(),
            DataValues::DistanceTraveled(value) => value.abbreviation(),
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
            DataValues::Battery(value) => value.add_config(index, ui),
            DataValues::BearingTrue(value) => value.add_config(index, ui),
            DataValues::DistanceTraveled(value) => value.add_config(index, ui),
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
            DataValues::Battery(value) => value.fmt_value(communicator),
            DataValues::BearingTrue(value) => value.fmt_value(communicator),
            DataValues::DistanceTraveled(value) => value.fmt_value(communicator),
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
            DataValues::Battery(value) => value.name(),
            DataValues::BearingTrue(value) => value.name(),
            DataValues::DistanceTraveled(value) => value.name(),
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
            DataValues::Battery(value) => value.unit_name(),
            DataValues::BearingTrue(value) => value.unit_name(),
            DataValues::DistanceTraveled(value) => value.unit_name(),
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
        ui.selectable_value(self, DataValues::Battery(Battery::default()), "BAT");
        ui.selectable_value(self, DataValues::BearingTrue(BearingTrue::default()), "BTW");
        ui.selectable_value(
            self,
            DataValues::DistanceTraveled(DistanceTraveled::default()),
            "DIS",
        );
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
