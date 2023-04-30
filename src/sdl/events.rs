extern crate sdl2;

use std::thread::Thread;

use sdl2::{event::Event, EventPump, Sdl, keyboard::Keycode};

pub struct EventHandler {
    event_pump: EventPump,
    pub events: Vec<ChipKeyCode>,
}

#[derive(PartialEq, Eq)]
pub enum ChipKeyCode {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    ZERO,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl EventHandler {
    
    pub fn new(sdl_context: &Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            event_pump,
            events: Vec::new(),
        }
    }

    pub fn update_events(&mut self) {
        self.events.clear();

        for event in self.event_pump.poll_iter() {

            let found_code: Option<ChipKeyCode> = match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => Some(ChipKeyCode::ONE),
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => Some(ChipKeyCode::TWO),
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => Some(ChipKeyCode::THREE),
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => Some(ChipKeyCode::C),
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => Some(ChipKeyCode::FOUR),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => Some(ChipKeyCode::FIVE),
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => Some(ChipKeyCode::SIX),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => Some(ChipKeyCode::D),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => Some(ChipKeyCode::SEVEN),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => Some(ChipKeyCode::EIGHT),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => Some(ChipKeyCode::NINE),
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => Some(ChipKeyCode::E),
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => Some(ChipKeyCode::A),
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => Some(ChipKeyCode::ZERO),
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => Some(ChipKeyCode::B),
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => Some(ChipKeyCode::F),

                Event::Quit { .. } => {
                    std::process::exit(0);
                }

                _ => {None}
            }; 

            if found_code.is_some() {
                self.events.push(found_code.unwrap());
            }
        }
    }

}
