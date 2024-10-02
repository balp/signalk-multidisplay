#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    use egui::ViewportBuilder;

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let full_screen = cfg!(target_os = "linux");
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size([800., 480.])
            .with_fullscreen(full_screen)
            .with_maximized(full_screen),
        ..Default::default()
    };

    eframe::run_native(
        "SingalK Multidisplay",
        native_options,
        Box::new(|cc| Ok(Box::new(signalk_multidisplay::DisplayApplication::new(cc)))),
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
                Box::new(|cc| Box::new(signalk_multidisplay::DisplayApplication::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
