use eframe::egui;
use eframe::egui::Widget;
use egui::vec2;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("Vending Machine", options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<AppState>::default()
        }
    ))
}

struct AppState {

}

impl Default for AppState {
    fn default() -> Self {
        Self {

        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select Item");
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                selection_button(ui, "1");
                selection_button(ui, "2");
                selection_button(ui, "3");
                selection_button(ui, "A");
                ui.end_row();
                selection_button(ui, "4");
                selection_button(ui, "5");
                selection_button(ui, "6");
                selection_button(ui, "B");
                ui.end_row();
                selection_button(ui, "7");
                selection_button(ui, "8");
                selection_button(ui, "9");
                selection_button(ui, "C");
                ui.end_row();
                selection_button(ui, "*");
                selection_button(ui, "0");
                selection_button(ui, "#");
                selection_button(ui, "D");
                ui.end_row();
            });
        });
    }
}

fn selection_button(ui: &mut egui::Ui, text: &str) {
    if ui.add(egui::Button::new(text).min_size(vec2(50.0, 50.0))).clicked() {
        println!("Button {} clicked", text);
    }
}