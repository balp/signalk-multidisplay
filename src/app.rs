use std::sync::mpsc::{channel, Sender, Receiver};
use eframe::egui;
use ehttp;
use eframe::egui::{RichText, Vec2};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use signalk::{Storage, V1DeltaFormat, V1Discovery, V1FullFormat, V1Navigation, V1NumberValue, V1Vessel};
use log::{debug, info, error};
use serde_json;

#[derive(Debug)]
pub enum SignalKError {
    Oops,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    server: String,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    signalk_data: Option<Storage>,
    #[serde(skip)]
    signalk_discovery: Option<V1Discovery>,
    #[serde(skip)]
    discovery_rx: Option<Receiver<V1Discovery>>,

    #[serde(skip)]
    delta_rx: Option<Receiver<V1DeltaFormat>>,
    #[serde(skip)]
    ws_sender: Option<WsSender>,
    #[serde(skip)]
    ws_receiver: Option<WsReceiver>,

    #[serde(skip)]
    full_rx: Option<Receiver<V1FullFormat>>,
}


impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            server: "http://192.168.1.22:3000/signalk".to_owned(),
            signalk_data: None,
            signalk_discovery: None,
            discovery_rx: None,
            delta_rx: None,
            ws_sender: None,
            ws_receiver: None,
            full_rx: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            let mut restored_app: TemplateApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            debug!("Restore object with server {}", restored_app.server);
            restored_app
        } else {
            debug!("Creating new instance.");
            Self::default()
        };
        app.set_up_server_connections();
        app
    }

    fn rx_data(&mut self, ctx: &egui::Context) {
        if let Some(ref discovery_rx_channel) = self.discovery_rx {
            match discovery_rx_channel.try_recv() {
                Ok(discovery) => {
                    self.signalk_discovery = Some(discovery);
                    let (full_sk_tx, full_sk_rx): (Sender<V1FullFormat>, Receiver<V1FullFormat>) = channel();
                    self.full_rx = Some(full_sk_rx);
                    if let Some(ref self_discovery) = self.signalk_discovery {
                        if let Some(ref endpoint) = self_discovery.get_v1_http_endpoint() {
                            let request = ehttp::Request::get(endpoint);
                            ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
                                match result {
                                    Ok(response) => {
                                        debug!("Full Got: {:?}", response);
                                        // debug!("full data: {:?}", response.bytes);
                                        let full: serde_json::Result<V1FullFormat> = serde_json::from_slice(&response.bytes);
                                        debug!("ful data: {:?}", full);
                                        if let Ok(full_value) = full {
                                            if let Err(err) = full_sk_tx.send(full_value) {
                                                error!("Can't send full back {:?}", err)
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        debug!("Get full error: {:?}", err);
                                    }
                                }
                            });
                            self.discovery_rx = None;
                        }
                        if let Some(ref endpoint) = self_discovery.get_v1_ws_endpoint() {
                            debug!("Connect websocket to {:?}", endpoint);
                            let ws_url = endpoint.to_string();
                            let ctx_clone = ctx.clone();
                            info!("Connect to websocket url: {}", ws_url);
                            let wakeup = move || ctx_clone.request_repaint();
                            match ewebsock::connect_with_wakeup(&ws_url, wakeup) {
                                Ok((ws_sender, ws_receiver)) => {
                                    debug!("Websocket connected ok!");
                                    self.ws_sender = Some(ws_sender);
                                    self.ws_receiver = Some(ws_receiver);
                                }
                                Err(error) => {
                                    error!("Failed to connect to {:?}: {}", &ws_url, error);
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    error!("Unable to recv discovery data");
                }
            }
        }
        if let Some(ref full_rx_channel) = self.full_rx {
            match full_rx_channel.try_recv() {
                Ok(full) => {
                    self.signalk_data = Some(Storage::new(full));
                }
                Err(_) => {}
            }
        }
        if let Some(ref delta_rx_channel) = self.delta_rx {
            match delta_rx_channel.try_recv() {
                Ok(delta) => {
                    if let Some(ref mut sk_data) = self.signalk_data {
                        sk_data.update(&delta);
                    }
                }
                Err(_) => {
                    error!("Unable to recv delta data");
                }
            }
        }
        if let Some(ref mut storage) = self.signalk_data {
            if let Some(ref ws_reiceiver_ref) = self.ws_receiver {
                if let Some(ws_event) = ws_reiceiver_ref.try_recv() {
                    debug!("Got ws event: {:?}", ws_event);
                    match ws_event {
                        WsEvent::Opened => {
                            info!("WebSocket delta opened.");
                        }
                        WsEvent::Message(ws_message) => {
                            match ws_message {
                                WsMessage::Binary(_) => {
                                    debug!("Binary ws message.");
                                }
                                WsMessage::Text(data) => {
                                    debug!("Text WS Message");
                                    let maybe_sk_delta : serde_json::Result<V1DeltaFormat> = serde_json::from_str(data.as_str());
                                    if let Ok(sk_delta) = maybe_sk_delta {
                                        storage.update(&sk_delta);
                                    }
                                }
                                WsMessage::Unknown(_) => { debug!("Unknown ws message."); }
                                WsMessage::Ping(_) => { debug!("Ping ws message."); }
                                WsMessage::Pong(_) => { debug!("Pong ws message."); }
                            }
                        }
                        WsEvent::Error(ws_error) => {
                            error!("Websocket error: {:?}", ws_error)
                        }
                        WsEvent::Closed => {
                            info!("WebSocket delta closed.");
                        }
                    }
                }
            }
        }
    }

    fn set_up_server_connections(&mut self) -> V1FullFormat {
        let request = ehttp::Request::get(self.server.as_str());
        let (signalk_tx, signalk_rx): (Sender<V1Discovery>, Receiver<V1Discovery>) = channel();
        self.discovery_rx = Some(signalk_rx);
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                Ok(response) => {
                    debug!("Got: {:?}", response);
                    debug!("data: {:?}", response.bytes);
                    let discovery: serde_json::Result<V1Discovery> = serde_json::from_slice(&response.bytes);
                    debug!("data: {:?}", discovery);
                    if let Ok(discovery_value) = discovery {
                        if let Err(e) = signalk_tx.send(discovery_value) {
                            error!("Can't send discovery back {:?}", e);
                        }
                    }
                }
                Err(err) => {
                    debug!("Error: {:?}", err);
                }
            }
        });

        debug!("Send request to discover signalk.");
        V1FullFormat::builder()
            .build()
    }

    fn get_stw_from_signalk(&self) -> Result<Option<f64>, SignalKError> {
        return if let Some(ref storage) = self.signalk_data {
            if let Some(_self_vessel) = storage.get().get_self() {
                if let Some(ref navigation) = _self_vessel.navigation {
                    if let Some(ref stw_number) = navigation.speed_through_water {
                        Ok(stw_number.value)
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            } else {
                Err(SignalKError::Oops)
            }
        } else {
            Err(SignalKError::Oops)
        };
    }
    fn fmt_stw(&self) -> String {
        let stw = self.get_stw_from_signalk();
        match stw {
            Ok(val) => {
                match val {
                    None => { "  -.-".to_owned() }
                    Some(value) => {
                        format!("{:5.1}", value)
                    }
                }
            }
            Err(_) => { " ERR ".to_owned() }
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.rx_data(ctx);

        let Self { server, signalk_data, signalk_discovery, discovery_rx, delta_rx, ws_sender, ws_receiver, full_rx } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Server Address: ");
                ui.text_edit_singleline(server);
            });


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            //
            // Layout:  STW
            //
            //  0.0  STW
            //       kt
            //  Water Speed
            ui.heading("SignalK Multidisplay");
            ui.group(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let current_stw = self.fmt_stw();
                        ui.label(RichText::new(current_stw).size(300.0).monospace());
                        ui.horizontal(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new("STW").size(50.0));
                                ui.label(RichText::new("kt").size(100.0));
                            });
                        });
                    });
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("Water Speed").size(150.0));
                    });
                });
                ui.set_min_size(Vec2::new(300.0, 150.0));
            });
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}