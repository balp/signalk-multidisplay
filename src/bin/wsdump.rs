use env_logger;
use ewebsock::WsEvent;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    env_logger::init();
    log::info!("Starting WS dump");
    let options = ewebsock::Options::default();
    let result = ewebsock::connect(
        "ws://127.0.0.1:3000/signalk/v1/stream?subscribe=all",
        options,
    );
    if let Ok(ws_info) = result {
        log::debug!("connect ok");
        let (mut sender, receiver) = ws_info;
        log::info!("Connected");
        //sender.send(ewebsock::WsMessage::Text("{\"context\":\"*\",\"unsubscribe\":[{\"path\":\"*\"}]}".into()));
        log::info!("Sending message");
        loop {
            let res = receiver.try_recv();
            if let Some(event) = res {
                log::info!("Received event: {:?}", event);
                match event {
                    WsEvent::Opened => {
                        log::info!("Opened");
                    }
                    WsEvent::Message(msg) => {
                        log::info!("Message: {:?}", msg);
                    }
                    WsEvent::Error(error) => {
                        log::error!("Error: {:?}", error);
                        break;
                    }
                    WsEvent::Closed => {
                        log::info!("Closed");
                        break;
                    }
                }
            } else {
                sleep(Duration::from_millis(100));
            }
        }
    } else {
        log::error!("connect failed");
    }
    log::info!("Stopping WS dump");
}
