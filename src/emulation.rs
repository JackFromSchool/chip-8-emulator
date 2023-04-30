mod chip;

use hex;
use substring::Substring;
use num::Num;
use std::{ fs::{File, self}, io::Read };

use crate::sdl::canvas::CanvasUtils;

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
}

impl<'a> Emulation<'a> {

    pub fn new(path: &str, canvas: &'a mut CanvasUtils) -> Self {
        let mut f = File::open(path).expect("File not found");
        let metadata = fs::metadata(path).expect("Could not read metadata");
        let mut instructions = vec![0; metadata.len() as usize];
        
        f.read(&mut instructions).expect("Buffer overflow");

        Self {
            instructions,
            chip8_data: chip::Chip8Components::new(),
            canvas,
        }
    }
    
    #[allow(dead_code)]
    pub fn debug_file(&self) {
        for i in 0..self.instructions.len() {
            println!("{}", hex::encode(&self.instructions[i..i+1]));
        }
    }

    pub fn execute_next_instruction(&mut self) {
        let current_pc = self.chip8_data.pc as usize;
        let instruction_hex: String = hex::encode(&self.instructions[current_pc..current_pc+2]);
        let instruction_dec: u16 = decode_hex(&instruction_hex);

        match instruction_hex.chars().nth(0).expect("Error in instruction deconstruction") {
            '0' => {
                if instruction_dec == 0x00E0 {
                    self.canvas.clear_screen();
                } else {
                    self.chip8_data.pc = self.chip8_data.stack.pop().unwrap();
                }
            },
            '1' => {
                self.chip8_data.pc = decode_hex(instruction_hex.substring(1, 4));
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
                ] = decode_hex(instruction_hex.substring(2, 4));
            },
            'A' => {
                self.chip8_data.index = decode_hex(instruction_hex.substring(1, 4));
            },
            'D' => {
                let x = decode_hex::<usize>(instruction_hex.substring(1, 2)) % 64;
                let y = decode_hex::<usize>(instruction_hex.substring(2, 3)) % 32;
                let n = decode_hex::<usize>(instruction_hex.substring(3, 4));

            }
            _ => {}
        }
    }
}
