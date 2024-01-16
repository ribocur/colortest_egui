#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::usize;

use eframe::{
    egui::{self, Visuals},
    epaint::Color32,
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([9999.0, 9999.0])
            .with_fullscreen(true),
        ..Default::default()
    };
    eframe::run_native(
        "ColorTest",
        options,
        Box::new(|_cc| Box::<ColorTestEgui>::default()),
    )
}

struct ColorTestEgui {
    visuals: Visuals,
    colors: Vec<Color32>,
    current_color: usize,
}

impl Default for ColorTestEgui {
    fn default() -> Self {
        Self {
            visuals: Visuals {
                panel_fill: Color32::WHITE,
                ..Visuals::default()
            },
            colors: vec![
                Color32::WHITE,
                Color32::GREEN,
                Color32::RED,
                Color32::BLUE,
                Color32::BLACK,
            ],
            current_color: 1,
        }
    }
}

impl eframe::App for ColorTestEgui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(self.visuals.clone());
            if ui.input(|i| i.key_pressed(egui::Key::Space)) {
                self.visuals = Visuals {
                    panel_fill: self.colors[self.current_color],
                    ..Visuals::default()
                };
                if self.current_color.eq(&(self.colors.len() - 1)) {
                    self.current_color = 0;
                } else {
                    self.current_color += 1;
                }
            }
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
