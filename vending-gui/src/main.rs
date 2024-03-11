use eframe::egui;
use egui::Key;

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
            ui.heading(format!("Selected: {}{}", self.current_selection.letter, self.current_selection.number));
            listen_for_letters(self, ctx);
            listen_for_numbers(self, ctx);
        });
    }
}

fn listen_for_letters(state: &mut AppState, ctx: &egui::Context) {
    if ctx.input(|i| i.key_pressed(Key::A)) {
        state.current_selection.letter = 'A';
        state.current_selection.number = 0;
    }
    if ctx.input(|i| i.key_pressed(Key::B)) {
        state.current_selection.letter = 'B';
        state.current_selection.number = 0;
    }
    if ctx.input(|i| i.key_pressed(Key::C)) {
        state.current_selection.letter = 'C';
        state.current_selection.number = 0;
    }
    if ctx.input(|i| i.key_pressed(Key::D)) {
        state.current_selection.letter = 'D';
        state.current_selection.number = 0;
    }
}

fn listen_for_numbers(state: &mut AppState, ctx: &egui::Context) {
    if ctx.input(|i| i.key_pressed(Key::Num1)) {
        state.current_selection.number = 1;
    }
    if ctx.input(|i| i.key_pressed(Key::Num2)) {
        state.current_selection.number = 2;
    }
    if ctx.input(|i| i.key_pressed(Key::Num3)) {
        state.current_selection.number = 3;
    }
    if ctx.input(|i| i.key_pressed(Key::Num4)) {
        state.current_selection.number = 4;
    }
    if ctx.input(|i| i.key_pressed(Key::Num5)) {
        state.current_selection.number = 5;
    }
    if ctx.input(|i| i.key_pressed(Key::Num6)) {
        state.current_selection.number = 6;
    }
    if ctx.input(|i| i.key_pressed(Key::Num7)) {
        state.current_selection.number = 7;
    }
    if ctx.input(|i| i.key_pressed(Key::Num8)) {
        state.current_selection.number = 8;
    }
    if ctx.input(|i| i.key_pressed(Key::Num9)) {
        state.current_selection.number = 9;
    }
    if ctx.input(|i| i.key_pressed(Key::Num0)) {
        state.current_selection.number = 0;
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
                number: 0,
            },            
        }
    }
}

