use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_simple_native("hello world", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
        });
    })
}