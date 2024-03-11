use eframe::egui;
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

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select Item");
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                selection_button(self, ui, "1");
                selection_button(self, ui, "2");
                selection_button(self, ui, "3");
                selection_button(self, ui, "A");
                ui.end_row();
                selection_button(self, ui, "4");
                selection_button(self, ui, "5");
                selection_button(self, ui, "6");
                selection_button(self, ui, "B");
                ui.end_row();
                selection_button(self, ui, "7");
                selection_button(self, ui, "8");
                selection_button(self, ui, "9");
                selection_button(self, ui, "C");
                ui.end_row();
                selection_button(self, ui, "*");
                selection_button(self, ui, "0");
                selection_button(self, ui, "#");
                selection_button(self, ui, "D");
                ui.end_row();
            });
            ui.add(egui::Label::new(format!("Selected: {}{}", self.current_selection.letter, self.current_selection.number)));
        });
    }
}

fn selection_button(state: &mut AppState, ui: &mut egui::Ui, text: &str) {
    if ui.add(egui::Button::new(text).min_size(vec2(50.0, 50.0))).clicked() {
        println!("Button {} clicked", text);
        match text {
            "A" => state.current_selection.letter = 'A',
            "B" => state.current_selection.letter = 'B',
            "C" => state.current_selection.letter = 'C',
            "D" => state.current_selection.letter = 'D',
            "#" => println!("dispense"),
            "*" => println!("cancel"),
            _ => state.current_selection.number = text.parse().unwrap(),
        }
    }
}

struct Selection {
    letter: char,
    number: i32,
}

struct AppState {
    current_selection: Selection,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_selection: Selection {
                letter: 'A',
                number: 1,
            },            
        }
    }
}

