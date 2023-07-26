use crate::communication::SignalKCommunicator;
use crate::layouts::Layout;
use eframe::egui;
use log::debug;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    server: String,
    view_config: bool,
    #[serde(skip)]
    communicator: Option<SignalKCommunicator>,
    #[serde(skip)]
    layout: crate::layouts::SingleValueLayout,
    #[serde(skip)]
    server_changed_tx: Option<Sender<String>>,
    #[serde(skip)]
    server_changed_rx: Option<Receiver<String>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            server: "https://demo.signalk.org/signalk".to_owned(),
            view_config: false,
            communicator: None,
            layout: crate::layouts::SingleValueLayout::default(),
            server_changed_tx: None,
            server_changed_rx: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            let restored_app: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            debug!("Restore object with server {}", restored_app.server);
            restored_app
        } else {
            debug!("Creating new instance.");
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

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        if let Some(ref mut sk_com) = self.communicator {
            sk_com.handle_data(ctx);
        }
        if let Some(ref mut server_changed_rx) = self.server_changed_rx {
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

        let Self {
            server,
            view_config,
            layout,
            server_changed_tx,
            ..
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Config").clicked() {
                        *view_config = !*view_config;
                    }
                    if !frame.is_web() {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
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
                layout.add_config(ui);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        egui::warn_if_debug_build(ui);
                    });
                });
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref comm) = self.communicator {
                layout.draw_ui(ui, comm);
            }
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
