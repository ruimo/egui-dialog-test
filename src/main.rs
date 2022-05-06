#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::containers::Resize;
use egui::color::*;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn render_dialog(&mut self, ctx: &egui::Context) {
        let window = egui::Window::new("My Dialog")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(true)
            .hscroll(true);

        window.show(ctx, |ui| {
            Resize::default().show(ui, |ui| {
                ui.colored_label(
                    Color32::RED,
                    "Hello!",
                );
            });
        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_dialog(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            println!("events: {:?}", ui.input().events.clone());

            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
