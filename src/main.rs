#![allow(unused_variables)]
use eframe::egui;

fn main() {

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_position([100.0, 100.0])
            .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native("Word Counter", native_options, Box::new(|cc| Ok(Box::new(EguiApp::new(cc))))).unwrap();
}

#[derive(Default)]
struct EguiApp {
    text_box_contents: String,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {

        cc.egui_ctx.set_visuals(egui::Visuals::light()); // Set light theme
        Self::default()
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let symbol_count: usize = self.text_box_contents.len();
        let word_count:   usize = self.text_box_contents.split_whitespace().count();

        // If Esc is pressed, exit the app.
        if ctx.input(|inp| inp.key_pressed(egui::Key::Escape)){
            std::process::exit(0);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Words: {word_count}\t\tSymbols: {symbol_count}"));
            let text_box_response = ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text_box_contents));

            text_box_response.request_focus();
        });
    }
}
