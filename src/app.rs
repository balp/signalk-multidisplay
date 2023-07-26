use crate::communication::SignalKCommunicator;
use crate::layouts::LayoutComponent;
use eframe::egui;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DisplayApplication {
    server: String,
    view_config: bool,
    #[serde(skip)]
    communicator: Option<SignalKCommunicator>,
    #[serde(skip)]
    layouts: Vec<crate::layouts::Layout>,
    #[serde(skip)]
    current_layout: usize,
    #[serde(skip)]
    last_layout_change: Instant,
    #[serde(skip)]
    server_changed_tx: Option<Sender<String>>,
    #[serde(skip)]
    server_changed_rx: Option<Receiver<String>>,
}

impl Default for DisplayApplication {
    fn default() -> Self {
        Self {
            server: "https://demo.signalk.org/signalk".to_owned(),
            view_config: false,
            communicator: None,
            layouts: vec![
                crate::layouts::Layout::SingleValue(crate::layouts::SingleValueLayout::new(
                    crate::datatypes::DataValues::SpeedThroughWater(
                        crate::datatypes::SpeedThroughWater::default(),
                    ),
                )),
                crate::layouts::Layout::SingleValue(crate::layouts::SingleValueLayout::new(
                    crate::datatypes::DataValues::SpeedOverGround(
                        crate::datatypes::SpeedOverGround::default(),
                    ),
                )),
                crate::layouts::Layout::SingleValue(crate::layouts::SingleValueLayout::new(
                    crate::datatypes::DataValues::CourseOverGround(
                        crate::datatypes::CourseOverGround::default(),
                    ),
                )),
            ],
            current_layout: 0,
            last_layout_change: Instant::now(),
            server_changed_tx: None,
            server_changed_rx: None,
        }
    }
}

impl DisplayApplication {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            let restored_app: DisplayApplication =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            log::debug!("Restore object with server {}", restored_app.server);
            restored_app
        } else {
            log::debug!("Creating new instance.");
            Self::default()
        };
        let mut communicator = SignalKCommunicator::default();
        communicator.set_up_server_connections(app.server.to_string());
        let (server_changed_tx, server_changed_rx): (Sender<String>, Receiver<String>) = channel();
        app.server_changed_tx = Some(server_changed_tx);
        app.server_changed_rx = Some(server_changed_rx);

        app.communicator = Some(communicator);
        app
    }
    pub fn server_changed(&mut self) {
        log::warn!("Server changed to {} IGNORED!!!", self.server);
    }
}

impl eframe::App for DisplayApplication {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        log::debug!("TemplateApp::update() - Enter");
        ctx.request_repaint();
        if let Some(ref mut sk_com) = self.communicator {
            log::debug!("Handle sk_com.handle_data()");
            sk_com.handle_data(ctx);
        }
        if let Some(ref mut server_changed_rx) = self.server_changed_rx {
            log::debug!("Server changed..");
            if server_changed_rx.try_recv().is_ok() {
                if let Some(ref mut communicator) = self.communicator {
                    communicator.disconnect_server();
                    communicator.set_up_server_connections(self.server.to_string());
                } else {
                    let mut communicator = SignalKCommunicator::default();
                    communicator.set_up_server_connections(self.server.to_string());
                    self.communicator = Some(communicator);
                }
            }
        }
        log::debug!("Draw UI..");

        let Self {
            server,
            view_config,
            layouts,
            current_layout,
            server_changed_tx,
            last_layout_change,
            ..
        } = self;

        if last_layout_change.elapsed() > Duration::from_secs(3) {
            log::info!("Update current layout {}", *current_layout);
            *last_layout_change = Instant::now();
            *current_layout = (*current_layout + 1) % layouts.len();
            log::info!("New current layout {}", *current_layout);
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Config").clicked() {
                        *view_config = !*view_config;
                    }
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                if !frame.is_web() {
                    ui.menu_button("View", |ui| {
                        egui::gui_zoom::zoom_menu_buttons(ui, frame.info().native_pixels_per_point);
                    });
                }
            });
        });

        // Side panel for config? Maybe a different view?
        if *view_config {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Configuration");

                ui.vertical(|ui| {
                    ui.label("Server Address: ");
                    let response = ui.text_edit_singleline(server);
                    if response.lost_focus() {
                        if let Some(tx_channel) = server_changed_tx {
                            if let Err(err) = tx_channel.send(server.to_string()) {
                                log::error!("Can't send server changed message {:?}", err);
                            };
                        }
                    }
                });

                ui.add_space(6.);
                layouts[*current_layout].add_config(ui);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        egui::warn_if_debug_build(ui);
                    });
                });
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref comm) = self.communicator {
                layouts[*current_layout].draw_ui(ui, comm);
            }
        });
        log::debug!("TemplateApp::update() - Exit");
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
