use std::sync::Arc;

use wasm_bindgen::prelude::*;

use egui::{app, Context, Id};

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
    num_columns: usize,
    current_value: u32,
}

impl app::App for WebApp {
    fn ui(
        &mut self,
        ctx: &Arc<Context>,
        integration_context: &mut app::IntegrationContext<'_>,
    ) {
        egui::SidePanel::left(Id::new("side_panel"),200.0).show(ctx, |ui| {
            ui.heading("Egui Demo");
            ui.label("Egui is an immediate mode GUI library written in Rust.");
            ui.add(egui::Hyperlink::new("https://github.com/emilk/egui").text("Egui home page"));

            ui.separator();
            ui.label(
                "This is an example of a panel. Windows are constrained to the area that remain.",
            );
            if ui.button("Organize windows").clicked {
                ui.ctx().memory().reset_areas();
            }
            ui.separator();

            ui.heading("Windows:");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Random BaRo31");

            let mut player_input = 0;

            ui.horizontal(|ui| {
                ui.label("Your turn: ");
                if ui.button("1").clicked {
                    player_input = 1;
                }
                if ui.button("2").clicked {
                    player_input = 2;
                }
                if ui.button("3").clicked {
                    player_input = 3;
                }
            });
            self.num_columns += player_input;
            ui.columns(self.num_columns, |cols| {
                for (i, col) in cols.iter_mut().enumerate() {
                    col.label(format!("{}", i));
                }
            });
        });
    }
}
