use eframe::egui;
use egui::{Key, Style, Visuals};
use std::process::Command;
use std::sync::{Arc, Mutex};
use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let prices_file = std::fs::read_to_string("prices.txt").unwrap();

    let mut prices = std::collections::HashMap::new();
    
    for price_line in prices_file.lines() {
        let mut split = price_line.split(" ");
        let letter = split.next().unwrap().chars().next().unwrap();
        let number = split.next().unwrap().parse::<u8>().unwrap();
        let price = split.next().unwrap().parse::<i32>().unwrap();
        prices.insert(Selection{letter, number}, price);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([480.0, 800.0])
        .with_maximized(true)
        .with_always_on_top()
        .with_active(true),
        ..Default::default()
    };

    eframe::run_native("Vending Machine", options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(App::new(&cc, prices))
        }
    ))
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_zoom_factor(2.75);
            ui.heading("Select Item");
            let current_processing_state = self.state.lock().unwrap().processing_state.clone();
            match current_processing_state {
                ProcessingState::Idle => {
                    display_selection(self, ui);
                    listen_for_enter(self, ctx);
                    listen_for_letters(self, ctx);
                    listen_for_numbers(self, ctx);
                },
                ProcessingState::GetPayment => {
                    ui.heading("Please scan the QR code to pay");
                    if !self.state.lock().unwrap().qr_code_finished {
                        ui.heading("Processing...");
                    } else {
                        ui.add(egui::Image::new("file://./qr.png"));
                    }
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
                state.lock().unwrap().current_selection.number = 255;
                request_repaint(state.clone());
            },
            ProcessingState::GetPayment => {

                // generate qr code here

                state.lock().unwrap().qr_code_finished = false;

                let prices = state.lock().unwrap().prices.clone();
                let current_selection = state.lock().unwrap().current_selection.clone();

                println!("Generating qr code for item {}{}", current_selection.letter, current_selection.number);
                println!("Price: Rs {}", prices.get(&current_selection).unwrap());

                println!("Sending payment request to server...");

                std::thread::sleep(std::time::Duration::from_secs(2));

                let qrcode_data = format!("https://google.com/{}", prices.get(&current_selection).unwrap());

                let qrcode = QRBuilder::new(qrcode_data)
                    .build()
                    .unwrap();
                let _image = SvgBuilder::default()
                    .shape(Shape::Square)
                    .to_file(&qrcode, "qr.svg");

                let _ = Command::new("convert")
                    .arg("-size")
                    .arg("200x200")
                    .arg("qr.svg")
                    .arg("qr.png")
                    .output()
                    .expect("failed to execute process");

//                println!("{:?}", convert_output);

                state.clone().lock().unwrap().ctx.clone().unwrap().forget_all_images();

                std::thread::sleep(std::time::Duration::from_millis(500));
                state.lock().unwrap().qr_code_finished = true;
                request_repaint(state.clone());

                // wait for payment

                println!("Waiting for payment");
                std::thread::sleep(std::time::Duration::from_secs(5));

                
                state.lock().unwrap().processing_state = ProcessingState::Dispensing;
                state.lock().unwrap().qr_code_finished = false;
            }
            _ => (),
        }
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
        if state.current_selection.number == 255 { ' ' } else { char::from_digit(state.current_selection.number.try_into().unwrap(), 10).unwrap() }));
    if state.current_selection.letter != 'Z' && state.current_selection.number != 255 {
        ui.heading(format!("Price : Rs {}", state.prices.get(&state.current_selection).unwrap()));
    }
}

fn listen_for_enter(app: &mut App, ctx: &egui::Context) {
    let mut state = app.state.lock().unwrap();
    if state.current_selection.letter == 'Z' || state.current_selection.number == 255 {
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
        state.current_selection.number = 255;
    }
    if ctx.input(|i| i.key_pressed(Key::B)) {
        state.current_selection.letter = 'B';
        state.current_selection.number = 255;
    }
    if ctx.input(|i| i.key_pressed(Key::C)) {
        state.current_selection.letter = 'C';
        state.current_selection.number = 255;
    }
    if ctx.input(|i| i.key_pressed(Key::D)) {
        state.current_selection.letter = 'D';
        state.current_selection.number = 255;
    }
    if ctx.input(|i| i.key_pressed(Key::E)) {
        state.current_selection.letter = '*';
        state.current_selection.number = 255;
    }
}

fn listen_for_numbers(app: &mut App, ctx: &egui::Context) {
    let mut state = app.state.lock().unwrap();
    if ctx.input(|i| i.key_pressed(Key::Num0)) {
        state.current_selection.number = 0;
    }
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

fn request_repaint(state: Arc<Mutex<State>>) {
    let ctx = &state.lock().unwrap().ctx;
    match ctx {
        Some (x) => x.request_repaint(),
        None => (),
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Selection {
    letter: char,
    number: u8,
}

struct State {
    current_selection: Selection,
    processing_state: ProcessingState,
    ctx: Option<egui::Context>,
    qr_code_finished: bool,
    prices: std::collections::HashMap<Selection, i32>,
}

#[derive(PartialEq, Clone, Copy)]
enum ProcessingState {
    Idle,
    GetPayment,
    Dispensing,
}

impl State {
    fn new(prices: std::collections::HashMap<Selection, i32>) -> Self {
        Self {
            current_selection: Selection {
                letter: 'Z',
                number: 255,
            },
            processing_state: ProcessingState::Idle,
            qr_code_finished: false,
            ctx: None,
            prices: prices,
        }
    }
}

struct App {
    state: Arc<Mutex<State>>,
}

impl App {
    pub fn new (cc: &eframe::CreationContext, prices: std::collections::HashMap<Selection, i32>) -> Self {
        let state = Arc::new(Mutex::new(State::new(prices)));
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

