use egui::Ui;

pub trait DataUnit {
    fn abbreviation(&self) -> String;
    fn add_config(&mut self, index: usize, ui: &mut Ui);
    fn format(&self, value: Result<f64, signalk::SignalKGetError>) -> String;
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
}

impl DataUnit for AngularUnit {
    fn abbreviation(&self) -> String {
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
                    format!("{:>5.3}", val)
                }
                AngularUnit::Degrees => {
                    let display_value = val * 180. / std::f64::consts::PI;
                    format!("{:>5.0}", display_value)
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
                ui.style_mut().wrap = Some(false);
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

    fn format(&self, value: Result<f64, signalk::SignalKGetError>) -> String {
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
