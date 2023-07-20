use crate::communication::SignalKCommunicator;
use crate::layouts::Layout;
use eframe::egui;
use log::debug;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    server: String,
    view_config: bool,
    #[serde(skip)]
    communicator: Option<SignalKCommunicator>,
    #[serde(skip)]
    layout: crate::layouts::SingleValueLayout,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            server: "http://192.168.1.22:3000/signalk".to_owned(),
            view_config: false,
            communicator: None,
            layout: crate::layouts::SingleValueLayout::default(),
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
        app.communicator = Some(communicator);
        app
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        if let Some(ref mut sk_com) = self.communicator {
            sk_com.handle_data(ctx);
        }

        let Self {
            server,
            view_config,
            layout,
            ..
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Config").clicked() {
                        *view_config = !*view_config;
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        // Side panel for config? Maybe a different view?
        if *view_config {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Configuration");

                ui.vertical(|ui| {
                    ui.label("Server Address: ");
                    ui.text_edit_singleline(server);
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
            ui.heading("SignalK Multidisplay");
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
