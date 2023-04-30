mod chip;

use hex;
use substring::Substring;
use num::Num;
use std::{ fs::{File, self}, io::Read };

use crate::sdl::{canvas::CanvasUtils, events::EventHandler};

fn decode_hex<T>(string: &str) -> T
where 
    T: Num,
{
    let Ok(num) = T::from_str_radix(string, 16) else {
        panic!("Error parsing hex to decimal");
    };
    num
}
pub struct Emulation<'a> {
    instructions: Vec<u8>,
    pub chip8_data: chip::Chip8Components,
    pub canvas: &'a mut CanvasUtils,
    events_handler: &'a mut EventHandler,
}

impl<'a> Emulation<'a> {

    pub fn new(
        path: &str,
        canvas: &'a mut CanvasUtils,
        events_handler: &'a mut EventHandler,
    ) -> Self {
        let mut f = File::open(path).expect("File not found");
        let metadata = fs::metadata(path).expect("Could not read metadata");
        let mut instructions = vec![0; metadata.len() as usize];
        
        f.read(&mut instructions).expect("Buffer overflow");

        let mut chip8_data =chip::Chip8Components::new();
        for i in 0..instructions.len() {
            chip8_data.memory[0x200 + i] = instructions[i];
        }
        chip8_data.pc = 0x200;

        Self {
            instructions,
            chip8_data,
            canvas,
            events_handler,
        }
    }
    
    #[allow(dead_code)]
    pub fn debug_file(&self) {
        for i in 0..self.instructions.len() {
            println!("{}", hex::encode(&self.instructions[i..i+1]));
        }
    }

    pub fn execute_next_instruction(&mut self) {
        self.events_handler.update_events();

        let current_pc = self.chip8_data.pc as usize;
        let instruction_hex: String = hex::encode(&self.chip8_data.memory[current_pc..current_pc+2]);
        let instruction_dec: u16 = decode_hex(&instruction_hex);

        let mut jumped = false;

        //println!("Current Instuction: {}", instruction_hex);

        match instruction_hex.chars().nth(0).expect("Error in instruction deconstruction") {
            '0' => {
                if instruction_dec == 0x00E0 {
                    self.canvas.clear_screen();
                } else {
                    self.chip8_data.pc = self.chip8_data.stack.pop().expect("No item in stack");
                }
            },
            '1' => {
                //println!("Jumped to {}", decode_hex::<u16>(instruction_hex.substring(1, 4)));
                self.chip8_data.pc = decode_hex(instruction_hex.substring(1, 4));
                jumped = true;
            },
            '2' => {
                self.chip8_data.stack.push(self.chip8_data.pc);
                self.chip8_data.pc = decode_hex(instruction_hex.substring(1, 4));
            },
            '6' => {
                self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ] = decode_hex(instruction_hex.substring(2, 4));
            },
            '7' => {
                self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ] += decode_hex::<u8>(instruction_hex.substring(2, 4));
            },
            'a' => {
                self.chip8_data.index = decode_hex(instruction_hex.substring(1, 4));
            },
            'd' => {
                let x = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ] as usize;
                //println!("x: {}", x);
                let y = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(2, 3))
                ] as usize;
                //println!("y: {}", y);
                let n = decode_hex::<usize>(instruction_hex.substring(3, 4));
                //println!("n: {}", n);
                let i = self.chip8_data.index as usize;
                
                'inner: for index in 0..n {
                    if y > 31 { break 'inner; }

                    let line = format!("{:08b}", self.chip8_data.memory[i+index]);

                    'inner_inner: for (delta_x, pixel) in line.chars().enumerate() {
                        if delta_x > 63 { break 'inner_inner; }

                        if pixel == '1' {
                            if self.canvas.invert_pixel(x+delta_x, y + index) {
                                self.chip8_data.var_registers[0xF] = 1;
                            }
                        }
                    }
                }

                self.canvas.update();
            }
            _ => {}
        }

        if !jumped {
            self.chip8_data.pc += 2;
        }
    }
}
