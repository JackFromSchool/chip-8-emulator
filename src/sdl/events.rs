extern crate sdl2;

use sdl2::{event::Event, EventPump, Sdl, keyboard::Keycode};

pub struct EventHandler {
    event_pump: EventPump,
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
        }
    }

    pub fn is_key_pressed(&mut self, keycode: ChipKeyCode) -> bool {
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
                } => Some(ChipKeyCode::FOUR),
                Event::KeyDown {
                    keycode: Some(Keycode::Num5),
                    ..
                } => Some(ChipKeyCode::FIVE),
                Event::KeyDown {
                    keycode: Some(Keycode::Num6),
                    ..
                } => Some(ChipKeyCode::SIX),
                Event::KeyDown {
                    keycode: Some(Keycode::Num7),
                    ..
                } => Some(ChipKeyCode::SEVEN),
                Event::KeyDown {
                    keycode: Some(Keycode::Num8),
                    ..
                } => Some(ChipKeyCode::EIGHT),
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => Some(ChipKeyCode::NINE),
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => Some(ChipKeyCode::ZERO),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => Some(ChipKeyCode::A),
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => Some(ChipKeyCode::B),
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => Some(ChipKeyCode::C),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => Some(ChipKeyCode::D),
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => Some(ChipKeyCode::E),
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => Some(ChipKeyCode::F),

                _ => {None}
            }; 

            if found_code.is_some() {
                if found_code.unwrap() == keycode {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn should_close(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            if matches!(event, Event::Quit { .. }) {
                return true;
            }
        }
        false
    }
}
