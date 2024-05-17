#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use eframe::Theme;
    use egui::ViewportBuilder;

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let viewport=ViewportBuilder{
        ..Default::default()
    };
    
    let native_options=eframe::NativeOptions {
   
        viewport: viewport.with_decorations(false).with_fullscreen(true).with_maximized(true),
        default_theme:Theme::Dark,
        ..Default::default()
    };
    
    
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(order::TemplateApp::new(cc))),
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
                Box::new(|cc| Box::new(order::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
