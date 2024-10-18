use eframe::epaint::text::TextWrapMode;
use egui::Ui;
use signalk::{SignalKGetError, V1PositionType};

pub trait DataUnit {
    fn abbreviation(&self) -> String;
    fn add_config(&mut self, index: usize, ui: &mut Ui);
    fn format(&self, value: Result<f64, SignalKGetError>) -> String;
}

#[derive(Debug, PartialEq)]
pub enum SpeedUnit {
    MeterPerSecond,
    Knot,
    MilesPerHour,
    KilometerPerHour,
}

impl DataUnit for SpeedUnit {
    fn abbreviation(&self) -> String {
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
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
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
    fn format(&self, value: Result<f64, SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                SpeedUnit::MeterPerSecond => {
                    format!("{:>5.2}", val)
                }
                SpeedUnit::Knot => {
                    let display_value = val * 3600. / 1851.85;
                    format!("{:>5.1}", display_value)
                }
                SpeedUnit::MilesPerHour => {
                    let display_value = val * 3600. / 1609.344;
                    format!("{:>5.1}", display_value)
                }
                SpeedUnit::KilometerPerHour => {
                    let display_value = val * 3.600;
                    format!("{:>5.2}", display_value)
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
    Mil,
}

impl DataUnit for AngularUnit {
    fn abbreviation(&self) -> String {
        match self {
            AngularUnit::Radians => "rad".to_string(),
            AngularUnit::Degrees => "deg".to_string(),
            AngularUnit::Mil => "mil".to_string(),
        }
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        egui::ComboBox::new(format!("angular_{}", index), "Unit")
            .selected_text(self.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
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
                ui.selectable_value(self, AngularUnit::Mil, AngularUnit::Mil.abbreviation());
            });
    }

    fn format(&self, value: Result<f64, SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                AngularUnit::Radians => {
                    format!("{:>5.3}", val)
                }
                AngularUnit::Degrees => {
                    let display_value = val * 180. / std::f64::consts::PI;
                    format!("{:>5.0}", display_value)
                }
                AngularUnit::Mil => {
                    let display_value = val * 3200. / std::f64::consts::PI;
                    format!("{:>5.0}", display_value)
                }
            },
            Err(_) => "-----".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DistanceUnit {
    Meters,
    NauticalMile,
    CableLength,
    Fathom,
}

impl DataUnit for DistanceUnit {
    fn abbreviation(&self) -> String {
        match self {
            DistanceUnit::Meters => "m".to_string(),
            DistanceUnit::NauticalMile => "nm".to_string(),
            DistanceUnit::CableLength => "cl".to_string(),
            DistanceUnit::Fathom => "fm".to_string(),
        }
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        egui::ComboBox::new(format!("data_type_{}", index), "Unit")
            .selected_text(self.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    self,
                    DistanceUnit::Meters,
                    DistanceUnit::Meters.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    DistanceUnit::NauticalMile,
                    DistanceUnit::NauticalMile.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    DistanceUnit::CableLength,
                    DistanceUnit::CableLength.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    DistanceUnit::Fathom,
                    DistanceUnit::Fathom.abbreviation(),
                );
            });
    }

    fn format(&self, value: Result<f64, SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                DistanceUnit::Meters => {
                    format!("{:>5.1}", val)
                }
                DistanceUnit::NauticalMile => {
                    let display_value = val * 1852.;
                    format!("{:>5.1}", display_value)
                }
                DistanceUnit::CableLength => {
                    let display_value = val * 185.2;
                    format!("{:>5.1}", display_value)
                }
                DistanceUnit::Fathom => {
                    let display_value = val * 1.8288;
                    format!("{:>5.1}", display_value)
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

impl DataUnit for TemperatureUnit {
    fn abbreviation(&self) -> String {
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
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
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

    fn format(&self, value: Result<f64, SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                TemperatureUnit::Celsius => {
                    let display_value = val - 273.15;
                    format!("{:>5.1}", display_value)
                }
                TemperatureUnit::Fahrenheit => {
                    let display_value = 9.0 / 5.0 * (val - 273.15) + 32.0;
                    format!("{:>5.1}", display_value)
                }
                TemperatureUnit::Kelvin => {
                    format!("{:>5.1}", val)
                }
            },
            Err(_) => "-----".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PressureUnit {
    HectoPascal,
    Millibar,
    MilliMetresOfMercury,
}

impl DataUnit for PressureUnit {
    fn abbreviation(&self) -> String {
        match self {
            PressureUnit::HectoPascal => "hPa".to_string(),
            PressureUnit::Millibar => "mbar".to_string(),
            PressureUnit::MilliMetresOfMercury => "mmHg".to_string(),
        }
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        egui::ComboBox::new(format!("angular_{}", index), "Unit")
            .selected_text(self.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    self,
                    PressureUnit::HectoPascal,
                    PressureUnit::HectoPascal.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    PressureUnit::Millibar,
                    PressureUnit::Millibar.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    PressureUnit::MilliMetresOfMercury,
                    PressureUnit::MilliMetresOfMercury.abbreviation(),
                );
            });
    }

    fn format(&self, value: Result<f64, SignalKGetError>) -> String {
        match value {
            Ok(val) => match self {
                PressureUnit::HectoPascal => {
                    let display_value = val * 0.01;
                    format!("{:>5.0}", display_value)
                }
                PressureUnit::Millibar => {
                    let display_value = val * 0.01;
                    format!("{:>5.0}", display_value)
                }
                PressureUnit::MilliMetresOfMercury => {
                    let display_value = val * 0.00750062;
                    format!("{:>5.0}", display_value)
                }
            },
            Err(_) => "-----".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VoltageUnit {
    Volt,
}

impl DataUnit for VoltageUnit {
    fn abbreviation(&self) -> String {
        "V".to_string()
    }

    fn add_config(&mut self, _index: usize, _ui: &mut Ui) {}

    fn format(&self, value: Result<f64, SignalKGetError>) -> String {
        match value {
            Ok(val) => format!("{:>5.1}", val),
            Err(_) => "-----".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PositionUnit {
    DecimalDegrees,
    DegreesMinutesSeconds,
    DegreesDecimalMinutes,
}
impl PositionUnit {
    pub(crate) fn format_pos(&self, value: &Option<V1PositionType>) -> String {
        if let Some(position) = value {
            if let Some(ref position_value) = position.value {
                match self {
                    PositionUnit::DecimalDegrees => {
                        // Decimal degrees (DD): 41.40338, 2.17403
                        format!(
                            "{:<10.6}\n{:<10.6}",
                            position_value.latitude, position_value.longitude
                        )
                        .to_string()
                    }
                    PositionUnit::DegreesMinutesSeconds => {
                        // Degrees, minutes, and seconds (DMS): 41°24'12.2"N 2°10'26.5"E
                        let lat_deg = position_value.latitude.trunc();
                        let lat_minutes = position_value.latitude.fract() * 60.0;
                        let lat_min = lat_minutes.trunc();
                        let lat_sec = lat_minutes.fract() * 60.0;
                        let lat_axel = if position_value.latitude > 0.0 {
                            "N"
                        } else {
                            "S"
                        };

                        let lon_deg = position_value.longitude.trunc();
                        let lon_minutes = position_value.longitude.fract() * 60.0;
                        let lon_min = lon_minutes.trunc();
                        let lon_sec = lon_minutes.fract() * 60.0;
                        let lon_axel = if position_value.longitude > 0.0 {
                            "E"
                        } else {
                            "W"
                        };

                        format!(
                            "{:>3.}°{:>2.}'{:>4.1}\"{}\n{:>3.}°{:>2.}'{:>4.1}\"{}",
                            lat_deg,
                            lat_min,
                            lat_sec,
                            lat_axel,
                            lon_deg,
                            lon_min,
                            lon_sec,
                            lon_axel,
                        )
                        .to_string()
                    }
                    PositionUnit::DegreesDecimalMinutes => {
                        // Degrees and decimal minutes (DMM): 41 24.2028, 2 10.4418
                        let lat_deg = position_value.latitude.trunc();
                        let lat_minutes = position_value.latitude.fract() * 60.0;
                        let lat_axel = if position_value.latitude > 0.0 {
                            "N"
                        } else {
                            "S"
                        };

                        let lon_deg = position_value.longitude.trunc();
                        let lon_minutes = position_value.longitude.fract() * 60.0;
                        let lon_axel = if position_value.longitude > 0.0 {
                            "E"
                        } else {
                            "W"
                        };

                        format!(
                            "{:>3.} {:>5.3} {}\n{:>3.} {:>5.3} {}",
                            lat_deg, lat_minutes, lat_axel, lon_deg, lon_minutes, lon_axel,
                        )
                        .to_string()
                    }
                }
            } else {
                "-----".to_string()
            }
        } else {
            "-----".to_string()
        }
    }
}

impl DataUnit for PositionUnit {
    fn abbreviation(&self) -> String {
        match self {
            PositionUnit::DecimalDegrees => "DD".to_string(),
            PositionUnit::DegreesMinutesSeconds => "DMS".to_string(),
            PositionUnit::DegreesDecimalMinutes => "DMM".to_string(),
        }
    }

    fn add_config(&mut self, index: usize, ui: &mut Ui) {
        egui::ComboBox::new(format!("position_{}", index), "Unit")
            .selected_text(self.abbreviation())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    self,
                    PositionUnit::DecimalDegrees,
                    PositionUnit::DecimalDegrees.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    PositionUnit::DegreesMinutesSeconds,
                    PositionUnit::DegreesMinutesSeconds.abbreviation(),
                );
                ui.selectable_value(
                    self,
                    PositionUnit::DegreesDecimalMinutes,
                    PositionUnit::DegreesDecimalMinutes.abbreviation(),
                );
            });
    }

    fn format(&self, _value: Result<f64, SignalKGetError>) -> String {
        "-----".to_string()
    }
}

#[derive(Debug, PartialEq)]
pub enum DateTimeUnit {
    Default,
}

impl DataUnit for crate::dataunits::DateTimeUnit {
    fn abbreviation(&self) -> String {
        "".to_string()
    }

    fn add_config(&mut self, _index: usize, _ui: &mut Ui) {}

    fn format(&self, _value: Result<f64, SignalKGetError>) -> String {
        "hh:mm:ss".to_string()
    }
}
