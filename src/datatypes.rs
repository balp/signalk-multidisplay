// Garmins data displays:
//  Abbreviation    Name
//  AIR           | Air Temperature                         | *+ self.environment.outside.temperature
//  AWA           | Apparent Wind Angle                     | *+ self.environment.wind.angleApparent
//  AWS           | Apparent Wind Speed                     | *+ self.environment.wind.speedApparent
//  BAR           | Barometer                               | *+ self.environment.outside.pressure
//  BAT           | Battery voltage                         | *+ self.electrical.batteries.house.voltage
//  BTW           | Direction from location to designation  | !!+ self.navigation.course.nextPoint.bearingTrue
//  COG           | Course over ground                      | !!+ self.navigation.courseOverGroundTrue
//  CTS           | Course to steer                         | ??
//  DIS           | Distance traveled                       | *+ self.navigation.log ??
//  DPT           | Depth of water                          | *+ self.environment.depth.belowSurface
//  DRF           | Speed of current                        | *+ self.environment.current.drift
//  DTW           | Distance to waypoint                    | !!+ self.navigation.course.nextPoint.distance
//  ELV           | Altitude                                | *+ self.navigation.position.altitude
//  ERR           | Error of current position               | ??
//  GWD           | Direction of wind relative ground       | *+ self.environment.wind.directionTrue
//  GWS           | Speed of wind relative ground           | *+ self.environment.wind.speedOverGround
//  HDG           | The direction the boat points           | *+ self.navigation.headingTrue
//  ODO           | Running tally of distance               | *+ self.navigation.log ??
//  OTH           | Opposite track direction                | ??
//  POS           | Current position                        | *+ self.navigation.position
//  RACE          | Race-timer                              | ??
//  REF           | A steer pilot reference                 | ??
//  RUD           | Rudder angle                            | !!+ self.steering.rudderAngle
//  SEA           | Temperature of sea water                | *+ self.environment.water.temperature
//  SOG           | Speed over ground                       | *+ self.navigation.speedOverGround
//  STW           | Boat Speed aka Speed Through Water      | *+ self.navigation.speedThroughWater
//  STR           | The steep pilot                         | ??
//  TRP           | A running tally of distance travel since last reset | +self.navigation.trip.log
//  TWA           | True wind angle from bow                | *+self.environment.wind.angleTrueGround
//  TWD           | True wind direction rel north           | *+self.environment.wind.directionTrue
//  TWS           | True wind speed relative vessel         | *+self.environment.wind.speedTrue
//  UTC           | Universal time coordinated              | !!+self.navigation.datetime
//  VMG           | Speed towards designation               | !!self.navigation.course.nextPoint.velocityMadeGood
//  WND           | Velocity made good upwind               |
//  XTE           | Cross track error                       | !!self.navigation.course.crossTrackError, navigation.course.calcValues.crossTrackError

use crate::communication::SignalKCommunicator;
use crate::datavalues::{
    AirTemperature, Altitude, ApparentWindAngle, ApparentWindSpeed, Barometer, Battery,
    BearingTrue, CourseOverGround, CrossTrackError, DataValue, DepthOfWater,
    DirectionOfWindRelativeGround, DistanceToWaypoint, DistanceTraveled, HeadingTrue, Odometer,
    Position, RudderAngle, SpeedOfCurrent, SpeedOfWindRelativeGround, SpeedOverGround,
    SpeedThroughWater, Trip, TrueWindAngleFromBow, TrueWindDirectionRelNorth, TrueWindSpeed,
    UniversalTimeCoordinated, VelocityMadeGood, VelocityMadeGoodUpwind, WaterTemperature,
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
    DepthOfWater(DepthOfWater),
    DistanceToWaypoint(DistanceToWaypoint),
    Altitude(Altitude),
    SpeedOfCurrent(SpeedOfCurrent),
    CourseOverGround(CourseOverGround),
    SpeedThroughWater(SpeedThroughWater),
    SpeedOverGround(SpeedOverGround),
    WaterTemperature(WaterTemperature),
    DirectionOfWindRelativeGround(DirectionOfWindRelativeGround),
    SpeedOfWindRelativeGround(SpeedOfWindRelativeGround),
    HeadingTrue(HeadingTrue),
    Odometer(Odometer),
    Position(Position),
    RudderAngle(RudderAngle),
    Trip(Trip),
    TrueWindAngleFromBow(TrueWindAngleFromBow),
    TrueWindDirectionRelNorth(TrueWindDirectionRelNorth),
    TrueWindSpeed(TrueWindSpeed),
    UniversalTimeCoordinated(UniversalTimeCoordinated),
    VelocityMadeGood(VelocityMadeGood),
    VelocityMadeGoodUpwind(VelocityMadeGoodUpwind),
    CrossTrackError(CrossTrackError),
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
            DataValues::DepthOfWater(value) => value.abbreviation(),
            DataValues::SpeedOfCurrent(value) => value.abbreviation(),
            DataValues::DistanceToWaypoint(value) => value.abbreviation(),
            DataValues::Altitude(value) => value.abbreviation(),
            DataValues::DirectionOfWindRelativeGround(value) => value.abbreviation(),
            DataValues::SpeedOfWindRelativeGround(value) => value.abbreviation(),
            DataValues::HeadingTrue(value) => value.abbreviation(),
            DataValues::Odometer(value) => value.abbreviation(),
            DataValues::Position(value) => value.abbreviation(),
            DataValues::RudderAngle(value) => value.abbreviation(),
            DataValues::Trip(value) => value.abbreviation(),
            DataValues::TrueWindAngleFromBow(value) => value.abbreviation(),
            DataValues::TrueWindDirectionRelNorth(value) => value.abbreviation(),
            DataValues::TrueWindSpeed(value) => value.abbreviation(),
            DataValues::UniversalTimeCoordinated(value) => value.abbreviation(),
            DataValues::VelocityMadeGood(value) => value.abbreviation(),
            DataValues::VelocityMadeGoodUpwind(value) => value.abbreviation(),
            DataValues::CrossTrackError(value) => value.abbreviation(),
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
            DataValues::DepthOfWater(value) => value.add_config(index, ui),
            DataValues::SpeedOfCurrent(value) => value.add_config(index, ui),
            DataValues::DistanceToWaypoint(value) => value.add_config(index, ui),
            DataValues::Altitude(value) => value.add_config(index, ui),
            DataValues::DirectionOfWindRelativeGround(value) => value.add_config(index, ui),
            DataValues::SpeedOfWindRelativeGround(value) => value.add_config(index, ui),
            DataValues::HeadingTrue(value) => value.add_config(index, ui),
            DataValues::Odometer(value) => value.add_config(index, ui),
            DataValues::Position(value) => value.add_config(index, ui),
            DataValues::RudderAngle(value) => value.add_config(index, ui),
            DataValues::Trip(value) => value.add_config(index, ui),
            DataValues::TrueWindAngleFromBow(value) => value.add_config(index, ui),
            DataValues::TrueWindDirectionRelNorth(value) => value.add_config(index, ui),
            DataValues::TrueWindSpeed(value) => value.add_config(index, ui),
            DataValues::UniversalTimeCoordinated(value) => value.add_config(index, ui),
            DataValues::VelocityMadeGood(value) => value.add_config(index, ui),
            DataValues::VelocityMadeGoodUpwind(value) => value.add_config(index, ui),
            DataValues::CrossTrackError(value) => value.add_config(index, ui),
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
            DataValues::DepthOfWater(value) => value.fmt_value(communicator),
            DataValues::SpeedOfCurrent(value) => value.fmt_value(communicator),
            DataValues::DistanceToWaypoint(value) => value.fmt_value(communicator),
            DataValues::Altitude(value) => value.fmt_value(communicator),
            DataValues::DirectionOfWindRelativeGround(value) => value.fmt_value(communicator),
            DataValues::SpeedOfWindRelativeGround(value) => value.fmt_value(communicator),
            DataValues::HeadingTrue(value) => value.fmt_value(communicator),
            DataValues::Odometer(value) => value.fmt_value(communicator),
            DataValues::Position(value) => value.fmt_position(communicator),
            DataValues::RudderAngle(value) => value.fmt_value(communicator),
            DataValues::Trip(value) => value.fmt_value(communicator),
            DataValues::TrueWindAngleFromBow(value) => value.fmt_value(communicator),
            DataValues::TrueWindDirectionRelNorth(value) => value.fmt_value(communicator),
            DataValues::TrueWindSpeed(value) => value.fmt_value(communicator),
            DataValues::UniversalTimeCoordinated(value) => value.fmt_time(communicator),
            DataValues::VelocityMadeGood(value) => value.fmt_value(communicator),
            DataValues::VelocityMadeGoodUpwind(value) => value.fmt_value(communicator),
            DataValues::CrossTrackError(value) => value.fmt_value(communicator),
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
            DataValues::DepthOfWater(value) => value.name(),
            DataValues::SpeedOfCurrent(value) => value.name(),
            DataValues::DistanceToWaypoint(value) => value.name(),
            DataValues::Altitude(value) => value.name(),
            DataValues::DirectionOfWindRelativeGround(value) => value.name(),
            DataValues::SpeedOfWindRelativeGround(value) => value.name(),
            DataValues::HeadingTrue(value) => value.name(),
            DataValues::Odometer(value) => value.name(),
            DataValues::Position(value) => value.name(),
            DataValues::RudderAngle(value) => value.name(),
            DataValues::Trip(value) => value.name(),
            DataValues::TrueWindAngleFromBow(value) => value.name(),
            DataValues::TrueWindDirectionRelNorth(value) => value.name(),
            DataValues::TrueWindSpeed(value) => value.name(),
            DataValues::UniversalTimeCoordinated(value) => value.name(),
            DataValues::VelocityMadeGood(value) => value.name(),
            DataValues::VelocityMadeGoodUpwind(value) => value.name(),
            DataValues::CrossTrackError(value) => value.name(),
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
            DataValues::DepthOfWater(value) => value.unit_name(),
            DataValues::SpeedOfCurrent(value) => value.unit_name(),
            DataValues::DistanceToWaypoint(value) => value.unit_name(),
            DataValues::Altitude(value) => value.unit_name(),
            DataValues::DirectionOfWindRelativeGround(value) => value.unit_name(),
            DataValues::SpeedOfWindRelativeGround(value) => value.unit_name(),
            DataValues::HeadingTrue(value) => value.unit_name(),
            DataValues::Odometer(value) => value.unit_name(),
            DataValues::Position(value) => value.unit_name(),
            DataValues::RudderAngle(value) => value.unit_name(),
            DataValues::Trip(value) => value.unit_name(),
            DataValues::TrueWindAngleFromBow(value) => value.unit_name(),
            DataValues::TrueWindDirectionRelNorth(value) => value.unit_name(),
            DataValues::TrueWindSpeed(value) => value.unit_name(),
            DataValues::UniversalTimeCoordinated(value) => value.unit_name(),
            DataValues::VelocityMadeGood(value) => value.unit_name(),
            DataValues::VelocityMadeGoodUpwind(value) => value.unit_name(),
            DataValues::CrossTrackError(value) => value.unit_name(),
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
            DataValues::DepthOfWater(DepthOfWater::default()),
            "DPT",
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
            DataValues::SpeedOfCurrent(SpeedOfCurrent::default()),
            "DRF",
        );
        ui.selectable_value(
            self,
            DataValues::DistanceToWaypoint(DistanceToWaypoint::default()),
            "DTW",
        );
        ui.selectable_value(self, DataValues::Altitude(Altitude::default()), "ELV");
        ui.selectable_value(
            self,
            DataValues::DirectionOfWindRelativeGround(DirectionOfWindRelativeGround::default()),
            "GWD",
        );
        ui.selectable_value(
            self,
            DataValues::SpeedOfWindRelativeGround(SpeedOfWindRelativeGround::default()),
            "GWS",
        );
        ui.selectable_value(self, DataValues::HeadingTrue(HeadingTrue::default()), "HDG");
        ui.selectable_value(self, DataValues::Odometer(Odometer::default()), "ODO");
        ui.selectable_value(self, DataValues::Position(Position::default()), "POS");
        ui.selectable_value(self, DataValues::RudderAngle(RudderAngle::default()), "RUD");
        ui.selectable_value(self, DataValues::Trip(Trip::default()), "TRP");
        ui.selectable_value(
            self,
            DataValues::TrueWindAngleFromBow(TrueWindAngleFromBow::default()),
            "TWA",
        );
        ui.selectable_value(
            self,
            DataValues::TrueWindDirectionRelNorth(TrueWindDirectionRelNorth::default()),
            "TWD",
        );
        ui.selectable_value(
            self,
            DataValues::TrueWindSpeed(TrueWindSpeed::default()),
            "TWS",
        );
        ui.selectable_value(
            self,
            DataValues::UniversalTimeCoordinated(UniversalTimeCoordinated::default()),
            "UTC",
        );
        ui.selectable_value(
            self,
            DataValues::VelocityMadeGood(VelocityMadeGood::default()),
            "VMG",
        );
        ui.selectable_value(
            self,
            DataValues::VelocityMadeGoodUpwind(VelocityMadeGoodUpwind::default()),
            "WND",
        );
        ui.selectable_value(
            self,
            DataValues::CrossTrackError(CrossTrackError::default()),
            "XTE",
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
