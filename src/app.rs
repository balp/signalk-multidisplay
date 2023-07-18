use eframe::egui;
use eframe::egui::{RichText, Vec2};
use ehttp;
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use log::{debug, error, info};
use serde_json;
use signalk::{
    Storage, V1DeltaFormat, V1Discovery, V1FullFormat,
};
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub enum SignalKError {
    Oops,
}

pub struct WebsocketHandler {
    ws_sender: WsSender,
    ws_receiver: WsReceiver,
}

impl WebsocketHandler {
    fn recv_signalk_delta_messages(&mut self, storage: &mut Storage) {
        if let Some(ws_event) = self.ws_receiver.try_recv() {
            Self::handle_ws_event(storage, ws_event);
        }
    }

    fn handle_ws_event(storage: &mut Storage, ws_event: WsEvent) {
        debug!("Got ws event: {:?}", ws_event);
        match ws_event {
            WsEvent::Opened => {
                info!("WebSocket delta opened.");
            }
            WsEvent::Message(ws_message) => {
                Self::handle_ws_message(storage, ws_message);
            }
            WsEvent::Error(ws_error) => {
                error!("Websocket error: {:?}", ws_error)
            }
            WsEvent::Closed => {
                info!("WebSocket delta closed.");
            }
        }
    }

    fn handle_ws_message(storage: &mut Storage, ws_message: WsMessage) {
        match ws_message {
            WsMessage::Binary(_) => {
                debug!("Binary ws message.");
            }
            WsMessage::Text(data) => {
                debug!("Text WS Message");
                let maybe_sk_delta: serde_json::Result<V1DeltaFormat> =
                    serde_json::from_str(data.as_str());
                if let Ok(sk_delta) = maybe_sk_delta {
                    storage.update(&sk_delta);
                }
            }
            WsMessage::Unknown(_) => {
                debug!("Unknown ws message.");
            }
            WsMessage::Ping(_) => {
                debug!("Ping ws message.");
            }
            WsMessage::Pong(_) => {
                debug!("Pong ws message.");
            }
        }
    }
}

#[derive(Default)]
pub struct SignalKCommunicator {
    signalk_data: Option<Storage>,
    signalk_discovery: Option<V1Discovery>,
    discovery_rx: Option<Receiver<V1Discovery>>,
    full_rx: Option<Receiver<V1FullFormat>>,
    ws_handler: Option<WebsocketHandler>,
}

impl SignalKCommunicator {

    fn rx_data(&mut self, ctx: &egui::Context) {
        if let Some(ref mut discovery_rx_channel) = self.discovery_rx {
            match discovery_rx_channel.try_recv() {
                Ok(discovery) => {
                    ctx.request_repaint();
                    self.signalk_discovery = Some(discovery);
                    let (full_sk_tx, full_sk_rx): (Sender<V1FullFormat>, Receiver<V1FullFormat>) =
                        channel();
                    self.full_rx = Some(full_sk_rx);
                    if let Some(ref self_discovery) = self.signalk_discovery {
                        if let Some(ref endpoint) = self_discovery.get_v1_http_endpoint() {
                            let request = ehttp::Request::get(endpoint);
                            let ctx_clone = ctx.clone();
                            ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
                                match result {
                                    Ok(response) => {
                                        debug!("Full Got: {:?}", response);
                                        let full: serde_json::Result<V1FullFormat> =
                                            serde_json::from_slice(&response.bytes);
                                        debug!("ful data: {:?}", full);
                                        if let Ok(full_value) = full {
                                            ctx_clone.request_repaint();
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
                                    self.ws_handler = Some(WebsocketHandler {
                                        ws_sender,
                                        ws_receiver,
                                    });
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
        if let Some(ref mut full_rx_channel) = self.full_rx {
            match full_rx_channel.try_recv() {
                Ok(full) => {
                    ctx.request_repaint();
                    self.signalk_data = Some(Storage::new(full));
                }
                Err(_) => {}
            }
        }
        if let Some(ref mut storage) = self.signalk_data {
            if let Some(ref mut ws_handler) = self.ws_handler {
                ws_handler.recv_signalk_delta_messages(storage);
            }
        }
    }
    fn set_up_server_connections(&mut self, server: String) {
        let request = ehttp::Request::get(server);
        let (signalk_tx, signalk_rx): (Sender<V1Discovery>, Receiver<V1Discovery>) = channel();
        self.discovery_rx = Some(signalk_rx);
        ehttp::fetch(
            request,
            move |result: ehttp::Result<ehttp::Response>| match result {
                Ok(response) => {
                    debug!("Got: {:?}", response);
                    debug!("data: {:?}", response.bytes);
                    let discovery: serde_json::Result<V1Discovery> =
                        serde_json::from_slice(&response.bytes);
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
            },
        );

        debug!("Send request to discover signalk.");
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

}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    server: String,
    #[serde(skip)]
    communicator: Option<SignalKCommunicator>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            server: "http://192.168.1.22:3000/signalk".to_owned(),
            communicator: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            let mut restored_app: TemplateApp =
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


    fn fmt_stw(&self) -> String {
        if let Some(ref sk_com) = self.communicator {
            let stw = sk_com.get_stw_from_signalk();
            match stw {
                Ok(val) => match val {
                    None => "  -.-".to_owned(),
                    Some(value) => {
                        format!("{:5.1}", value)
                    }
                },
                Err(_) => "-----".to_owned(),
            }
        } else {
            "NOCOM".to_owned()
        }
    }
}

// Garmins data displays:
//  AIR - Air Temperature
//  AWA - Apparent Wind Angle
//  AWS - Apparent Wind Speed
//  BAR - Barmometer
//  BAT - Battery voltage
//  BSP - Boat Speed
//  BTW - Direction from location to desination
//  COG - Course over ground
//  CTS - Course to steer
//  DIS - Distance traveled
//  DPT - Depth of water
//  DRF - Speed of current
//  DTW - Distance to waypoint
//  ELV - Altitude
//  ERR - Error of current position
//  GWD - Direction of wind relative ground
//  GWS - Spped of wind relative ground
//  HDG - The direction boat ppoints
//  ODO - Running tally of distance
//  OTH - Opposite track direction
//  POS - Current position
//  RACE - Rracetimer
//  REF - A steer pilot reference
//  RUD - Rudder angle
//  SEA - Teamerature of sea water
//  SOG - Speed over ground
//  STR - The steep pilot
//  TRP - A running tally of distance travel since last reset
//  TWA - True wind angle from bow
//  TWD - True wind diection rel north
//  TWS - True wind speed relative vessel
//  UTC - Universal time coordinated
//  VMG - Speed towards desination
//  WND - Velocity made good upwind
//  XTE - Cross track error

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        if let Some(ref mut sk_com) = self.communicator {
            sk_com.rx_data(ctx);
        }

        let Self { server, .. } = self;

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

        // Side panel for config? Maybe a different view?
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
