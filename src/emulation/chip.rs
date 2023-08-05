pub struct Chip8Components {
    pub memory: [u8; 4096],
    pub pc: u16,
    pub index: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub var_registers: [u8; 16]
}

impl Chip8Components {

    pub fn new() -> Self {
        let mut memory = [0; 4096];
        let mut i = 0;
        for line in include_str!("font.txt").lines() {
            for section in line.split(" ") {
                memory[i] = u8::from_str_radix(section, 16).unwrap();
                i += 1;
            }
        }
        

        Self {
            memory,
            pc: 0,
            index: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            var_registers: [0; 16]
        }
    }

}
