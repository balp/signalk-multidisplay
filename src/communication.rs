use egui::Context;
use ewebsock::{WsEvent, WsMessage, WsReceiver};
use signalk::{Storage, V1DeltaFormat, V1Discovery, V1FullFormat};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct WebsocketHandler {
    ws_receiver: WsReceiver,
}

impl WebsocketHandler {
    fn recv_signalk_delta_messages(&mut self, storage: &mut Storage) {
        if let Some(ws_event) = self.ws_receiver.try_recv() {
            Self::handle_ws_event(storage, ws_event);
        }
    }

    fn handle_ws_event(storage: &mut Storage, ws_event: WsEvent) {
        match ws_event {
            WsEvent::Opened => {
                log::info!("WebSocket delta opened.");
            }
            WsEvent::Message(ws_message) => {
                log::info!("WebSocket message.");
                Self::handle_ws_message(storage, ws_message);
            }
            WsEvent::Error(ws_error) => {
                log::error!("Websocket error: {:?}", ws_error)
            }
            WsEvent::Closed => {
                log::info!("WebSocket delta closed.");
            }
        }
    }

    fn handle_ws_message(storage: &mut Storage, ws_message: WsMessage) {
        match ws_message {
            WsMessage::Binary(_) => {
                log::debug!("Binary ws message.");
            }
            WsMessage::Text(data) => {
                log::debug!("WS Text message: {:?}", data.as_str());
                let maybe_sk_delta: serde_json::Result<V1DeltaFormat> =
                    serde_json::from_str(data.as_str());
                if let Ok(sk_delta) = maybe_sk_delta {
                    log::debug!("New sk delta: {:?}", sk_delta);
                    storage.update(&sk_delta);
                }
            }
            WsMessage::Unknown(_) => {
                log::debug!("Unknown ws message.");
            }
            WsMessage::Ping(_) => {
                log::debug!("Ping ws message.");
            }
            WsMessage::Pong(_) => {
                log::debug!("Pong ws message.");
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
    pub(crate) fn disconnect_server(&mut self) {
        self.signalk_data = None;
        self.signalk_discovery = None;
        self.discovery_rx = None;
        self.full_rx = None;
        self.ws_handler = None;
    }
    pub(crate) fn set_up_server_connections(&mut self, server: String) {
        let request = ehttp::Request::get(server);
        let (signalk_tx, signalk_rx): (Sender<V1Discovery>, Receiver<V1Discovery>) = channel();
        self.discovery_rx = Some(signalk_rx);
        ehttp::fetch(
            request,
            move |result: ehttp::Result<ehttp::Response>| match result {
                Ok(response) => {
                    let discovery: serde_json::Result<V1Discovery> =
                        serde_json::from_slice(&response.bytes);
                    if let Ok(discovery_value) = discovery {
                        if let Err(e) = signalk_tx.send(discovery_value) {
                            log::error!("Can't send discovery back {:?}", e);
                        } else {
                            log::info!("Discovery message sent");
                        }
                    }
                }
                Err(err) => {
                    log::error!("Error: {:?}", err);
                }
            },
        );
    }

    pub(crate) fn handle_data(&mut self, ctx: &Context) {
        self.handle_discovery(ctx);
        self.handle_full_message(ctx);
        self.handle_signalk_data();
    }

    fn handle_signalk_data(&mut self) {
        if let Some(ref mut storage) = self.signalk_data {
            if let Some(ref mut ws_handler) = self.ws_handler {
                ws_handler.recv_signalk_delta_messages(storage);
            }
        }
    }

    fn handle_full_message(&mut self, ctx: &Context) {
        if let Some(ref mut full_rx_channel) = self.full_rx {
            if let Ok(full) = full_rx_channel.try_recv() {
                log::debug!("New sk full message");
                ctx.request_repaint();
                self.signalk_data = Some(Storage::new(full));
            }
        }
    }

    fn handle_discovery(&mut self, ctx: &Context) {
        if let Some(ref mut discovery_rx_channel) = self.discovery_rx {
            match discovery_rx_channel.try_recv() {
                Ok(discovery) => {
                    log::debug!("New discovery message");
                    self.set_discovery(ctx, discovery);
                }
                Err(_) => {
                    log::error!("Unable to recv discovery data");
                }
            }
        }
    }

    fn set_discovery(&mut self, ctx: &Context, discovery: V1Discovery) {
        ctx.request_repaint();
        self.signalk_discovery = Some(discovery);
        if let Some(ref endpoint) = self.get_http_endpoint() {
            self.request_full_status(ctx, endpoint);
        }
        if let Some(ref endpoint) = self.get_ws_endpoint() {
            self.setup_websocket_delta(ctx, endpoint);
        }
    }

    fn get_http_endpoint(&self) -> Option<String> {
        match &self.signalk_discovery {
            None => None,
            Some(discovery) => discovery.get_v1_http_endpoint(),
        }
    }
    fn get_ws_endpoint(&self) -> Option<String> {
        match &self.signalk_discovery {
            None => None,
            Some(discovery) => discovery.get_v1_ws_endpoint(),
        }
    }

    fn setup_websocket_delta(&mut self, ctx: &Context, endpoint: &String) {
        log::debug!("Connect websocket to {:?}", endpoint);
        let ws_url = endpoint.to_string();
        let ctx_clone = ctx.clone();
        log::info!("Connect to websocket url: {}", ws_url);
        let wakeup = move || ctx_clone.request_repaint();
        match ewebsock::connect_with_wakeup(&ws_url, wakeup) {
            Ok((_ws_sender, ws_receiver)) => {
                log::debug!("Websocket connected ok!");
                self.ws_handler = Some(WebsocketHandler { ws_receiver });
            }
            Err(error) => {
                log::error!("Failed to connect to {:?}: {}", &ws_url, error);
            }
        }
    }

    fn request_full_status(&mut self, ctx: &Context, endpoint: &String) {
        let request = ehttp::Request::get(endpoint);
        let (full_sk_tx, full_sk_rx): (Sender<V1FullFormat>, Receiver<V1FullFormat>) = channel();
        self.full_rx = Some(full_sk_rx);
        self.discovery_rx = None;
        let ctx_clone = ctx.clone();
        ehttp::fetch(
            request,
            move |result: ehttp::Result<ehttp::Response>| match result {
                Ok(response) => {
                    log::debug!("Full Got: {:?}", response);
                    let full: serde_json::Result<V1FullFormat> =
                        serde_json::from_slice(&response.bytes);
                    log::debug!("Full data: {:?}", full);
                    if let Ok(full_value) = full {
                        ctx_clone.request_repaint();
                        if let Err(err) = full_sk_tx.send(full_value) {
                            log::error!("Can't send full back {:?}", err)
                        }
                    }
                }
                Err(err) => {
                    log::debug!("Get full error: {:?}", err);
                }
            },
        );
    }

    pub(crate) fn get_f64_for_path(&self, path: String) -> Result<f64, signalk::SignalKGetError> {
        if let Some(ref storage) = self.signalk_data {
            storage.get_f64_for_path(path)
        } else {
            Err(signalk::SignalKGetError::ValueNotSet)
        }
    }
}
