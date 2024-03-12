use eframe::egui;
use egui::Key;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("Vending Machine", options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(App::new(&cc))
        }
    ))
}

struct App {
    state: Arc<Mutex<State>>,
}

impl App {
    pub fn new (cc: &eframe::CreationContext) -> Self {
        let state = Arc::new(Mutex::new(State::default()));
        state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        let state_clone = state.clone();
        std::thread::spawn(move || {
            handle_states(state_clone)
        });
        Self {
            state,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select Item");
            display_selection(self, ui);
            let current_processing_state = self.state.lock().unwrap().processing_state.clone();
            match current_processing_state {
                ProcessingState::Idle => {
                    listen_for_enter(self, ctx);
                    listen_for_letters(self, ctx);
                    listen_for_numbers(self, ctx);
                },
                ProcessingState::GetPayment => {
                    self.state.lock().unwrap().processing_state = ProcessingState::Dispensing;
                },
                ProcessingState::Dispensing => {
                    display_dispensing(self, ui);
                }
            }
        });
    }
}

fn handle_states(state: Arc<Mutex<State>>) {
    loop {
        let current_state = state.lock().unwrap().processing_state;
        match current_state {
            ProcessingState::Dispensing => {
                request_repaint(state.clone());
                std::thread::sleep(std::time::Duration::from_secs(2));
                state.lock().unwrap().processing_state = ProcessingState::Idle;
                state.lock().unwrap().current_selection.letter = 'Z';
                state.lock().unwrap().current_selection.number = 0;
                request_repaint(state.clone());
            },
            _ => (),
        }
    }
}

fn request_repaint(state: Arc<Mutex<State>>) {
    let ctx = &state.lock().unwrap().ctx;
    match ctx {
        Some (x) => x.request_repaint(),
        None => (),
    }
}

fn display_dispensing(app: &mut App, ui: &mut egui::Ui) {
    let state = app.state.lock().unwrap();
    ui.heading(format!("Dispensing {}{}...", 
        state.current_selection.letter, state.current_selection.number), );
}

fn display_selection(app: &mut App, ui: &mut egui::Ui) {
    let state = app.state.lock().unwrap();
    ui.heading(format!("Selected item : {}{}", 
        if state.current_selection.letter == 'Z' { ' ' } else { state.current_selection.letter }, 
        if state.current_selection.number == 0 { ' ' } else { char::from_digit(state.current_selection.number.try_into().unwrap(), 10).unwrap() }));
}

fn listen_for_enter(app: &mut App, ctx: &egui::Context) {
    let mut state = app.state.lock().unwrap();
    if state.current_selection.letter == 'Z' || state.current_selection.number == 0 {
        return;
    }
    if ctx.input(|i| i.key_pressed(Key::Enter)) {
        state.processing_state = ProcessingState::GetPayment;
    }
}

fn listen_for_letters(app: &mut App, ctx: &egui::Context) {
    let mut state = app.state.lock().unwrap();
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

fn listen_for_numbers(app: &mut App, ctx: &egui::Context) {
    let mut state = app.state.lock().unwrap();
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
}

struct Selection {
    letter: char,
    number: i32,
}

struct State {
    current_selection: Selection,
    processing_state: ProcessingState,
    ctx: Option<egui::Context>,
}

#[derive(PartialEq, Clone, Copy)]
enum ProcessingState {
    Idle,
    GetPayment,
    Dispensing,
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

impl State {
    fn new() -> Self {
        Self {
            current_selection: Selection {
                letter: 'Z',
                number: 0,
            },
            processing_state: ProcessingState::Idle,
            ctx: None,
        }
    }
}

