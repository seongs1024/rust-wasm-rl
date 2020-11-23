use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    let app = egui::DemoApp::default();
    let backend = egui_web::WebBackend::new(canvas_id)?;
    let runner = egui_web::AppRunner::new(backend, Box::new(app))?;
    egui_web::start(runner)?;
    Ok(())
}
