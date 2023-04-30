use std::fmt::format;

mod sdl;
mod emulation;

fn main() {
    let mut handles = sdl::SdlHandles::new();

    let mut emulation = emulation::Emulation::new("roms/IBM_Logo.ch8", &mut handles.canvas);
    
    
    loop {
        if handles.events.should_close() {
            break;
        }


        emulation.execute_next_instruction();
    }
}
