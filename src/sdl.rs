extern crate sdl2;

use sdl2::{ Sdl, VideoSubsystem, video::Window };

use self::{canvas::CanvasUtils, events::EventHandler};

pub mod canvas;
pub mod events;

pub const PIXEL_SIZE: u32 = 10;

pub struct SdlHandles {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: CanvasUtils,
    pub events: EventHandler,
}

impl SdlHandles {

    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Chip-8 Emulator", 64*PIXEL_SIZE, 32*PIXEL_SIZE)
            .build()
            .unwrap();
        let canvas = CanvasUtils::new(window);
        let events = EventHandler::new(&sdl_context);

        Self {
            sdl_context,
            video_subsystem,
            canvas,
            events,
        }
    }

}
