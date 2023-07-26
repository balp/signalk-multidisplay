#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::Vec2;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(800., 480.));
    // native_options.max_window_size = Some(Vec2::new(800., 480.));
    // native_options.min_window_size = Some(Vec2::new(800., 480.));
    native_options.maximized = true;
    native_options.fullscreen = true;

    eframe::run_native(
        "SingalK Multidisplay",
        native_options,
        Box::new(|cc| Box::new(signalk_multidisplay::TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(signalk_multidisplay::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
