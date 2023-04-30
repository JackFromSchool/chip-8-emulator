mod sdl;
mod emulation;

fn main() {
    let mut handles = sdl::SdlHandles::new();
    
    let mut emulation = emulation::Emulation::new(
        "roms/IBM_Logo.ch8",
        &mut handles.canvas,
        &mut handles.events
    );
    
    
    loop {
        emulation.execute_next_instruction();
    }
}
