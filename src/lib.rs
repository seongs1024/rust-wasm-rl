use std::{
    sync::Arc,
    collections::{LinkedList},
    vec::Vec
};

use wasm_bindgen::prelude::*;

use egui::{app, Context, Id, Label, color::srgba};

#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    // let app = egui::DemoApp::default();
    let app = WebApp::default();
    let backend = egui_web::WebBackend::new(canvas_id)?;
    let runner = egui_web::AppRunner::new(backend, Box::new(app))?;
    egui_web::start(runner)?;
    Ok(())
}

struct WebApp {
    num_players: usize,
    current_player: Player,
    turns: LinkedList<Turn>,
}

type Player = usize;

enum Count {
    Strike,
    Doubble,
    Turkey,
}

type Turn = (Player, Count);

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

            ui.horizontal(|ui| {
                ui.label(format!("{} Players are up!", self.num_players));
            });

            ui.horizontal(|ui| {
                ui.label(format!("Player {}'s turn: ", self.current_player));
                if ui.button("1").clicked {
                    self.stack_turn(self.current_player, Count::Strike);
                    self.end_turn();
                }
                if ui.button("2").clicked {
                    self.stack_turn(self.current_player, Count::Doubble);
                    self.end_turn();
                }
                if ui.button("3").clicked {
                    self.stack_turn(self.current_player, Count::Turkey);
                    self.end_turn();
                }
            });

            ui.columns(self.total_count(), |cols| {
                let mut acc_count = 0;
                let list: Vec<(&Player, usize)> = self.turns.iter()
                    .flat_map(|(player, count)| {
                        let mut v = Vec::new();
                        for c in 0..c_to_i(count) {
                            acc_count += 1;
                            v.push((player, acc_count));
                        }
                        v
                    })
                    .collect();
                cols.iter_mut().zip(list.iter())
                    .for_each(|(col, &(p, i))| {
                        if *p % 2 == 0 {
                            col.add(Label::new(format!("{}", i)).text_color(srgba(110, 255, 110, 255)));
                        }
                        else {
                            col.add(Label::new(format!("{}", i)).text_color(srgba(128, 140, 255, 255)));
                        }
                    });
            });

        });
    }
}

impl Default for WebApp {
    fn default() -> Self {
        Self {
            num_players: 2,
            current_player: 0 as Player,
            turns: LinkedList::new(),
        }
    }
}

fn c_to_i(count: &Count) -> usize {
    match count {
        Count::Strike => 1,
        Count::Doubble => 2,
        Count::Turkey => 3,
    }
}

impl WebApp {
    pub fn stack_turn(&mut self, player: Player, count: Count) {
        self.turns.push_back((player, count));
    }

    pub fn end_turn(&mut self) {
        self.current_player = (self.current_player + 1) % self.num_players;
    }

    pub fn total_count(&self) -> usize {
       self.turns.iter()
            .fold(0, |acc, (_, count)| acc + c_to_i(count) )
    }
}
