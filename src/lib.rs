use std::sync::Arc;

use wasm_bindgen::prelude::*;

use egui::{app, Context};

#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    // let app = egui::DemoApp::default();
    let app = WebApp::default();
    let backend = egui_web::WebBackend::new(canvas_id)?;
    let runner = egui_web::AppRunner::new(backend, Box::new(app))?;
    egui_web::start(runner)?;
    Ok(())
}

#[derive(Default)]
struct WebApp {
    name: String,
    age: u32,
}

impl app::App for WebApp {
    fn ui(
        &mut self,
        ctx: &Arc<Context>,
        integration_context: &mut app::IntegrationContext<'_>,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit(&mut self.name);
            });
            ui.add(egui::Slider::u32(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
