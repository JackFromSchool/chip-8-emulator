mod chip;

use hex;
use substring::Substring;
use num::Num;
use rand::prelude::*;
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

fn least_significant_bit(num: u8) -> u8 {
    let binary_str = format!("{:08b}", num);
    binary_str.chars().nth(15).unwrap().to_digit(2).unwrap() as u8
}

fn most_significant_bit(num: u8) -> u8 {
    let binary_str = format!("{:08b}", num);
    binary_str.chars().nth(0).unwrap().to_digit(2).unwrap() as u8
}

fn add_overflow(num: u8, add: u8) -> (bool, u8) {
    let product = num as u16 + add as u16;
    if product > 255 {
        (true, (product-255) as u8)
    } else {
        (false, product as u8)
    }
}

fn sub_overflow(num: u8, sub: u8) -> (bool, u8) {
    let product = num as i16 - sub as i16;
    if product < 0 {
        (false, (num as i16 + product) as u8)
    } else {
        (true, product as u8)
    }
}

trait NumExt {
    fn to_binary_iterator(&self) -> Vec<u32>;
}

impl<T: Num + std::fmt::Binary> NumExt for T {
    fn to_binary_iterator(&self) -> Vec<u32> {
        let binary_str = format!("{:08b}", self);
        binary_str.chars().map(|x| x.to_digit(2).unwrap() ).collect()
    }
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
            '3' => {
                let x = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ];
                let n = decode_hex(instruction_hex.substring(2, 4));

                if x == n {
                    self.chip8_data.pc += 2;
                }
            },
            '4' => {
                let x = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ];
                let n = decode_hex(instruction_hex.substring(2, 4));

                if x != n {
                    self.chip8_data.pc += 2;
                }
            },
            '5' => {
                let x = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ];
                let y = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(2, 3))
                ];

                if x == y {
                    self.chip8_data.pc += 2;
                }
            },
            '6' => {
                self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ] = decode_hex(instruction_hex.substring(2, 4));
            },
            '7' => {
                self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ].wrapping_add(decode_hex::<u8>(instruction_hex.substring(2, 4)));
            },
            '8' => {

                let y = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(2, 3))
                ];

                let x = &mut self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ];

                match instruction_hex.chars().nth(3).unwrap() {
                    '0' => {
                        *x = y;
                    },
                    '1' => {
                        *x |= y;
                    },
                    '2' => {
                        *x &= y;
                    },
                    '3' => {
                        *x ^= y;
                    },
                    '4' => {
                        let (carry, val) = add_overflow(*x, y);
                        *x = val;
                        self.chip8_data.var_registers[0xF] = if carry { 1 } else { 0 };
                    },
                    '5' => {
                        let (carry, val) = sub_overflow(*x, y);
                        *x = val;
                        self.chip8_data.var_registers[0xF] = if carry { 1 } else { 0 };
                    },
                    '6' => {
                        let carry = least_significant_bit(*x);
                        *x >>= 1;
                        self.chip8_data.var_registers[0xF] = carry;
                    },
                    '7' => {
                        let (carry, val) = sub_overflow(y, *x);
                        *x = val;
                        self.chip8_data.var_registers[0xF] = if carry { 1 } else { 0 };
                    },
                    'E' => {
                        let carry = most_significant_bit(*x);
                        *x <<= 1;
                        self.chip8_data.var_registers[0xF] = carry;
                    }
                    _ => {}
                }
            }
            'a' => {
                self.chip8_data.index = decode_hex(instruction_hex.substring(1, 4));
            },
            'b' => {
                let x = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ];
                let n: u8 = decode_hex(instruction_hex.substring(1, 4));

                self.chip8_data.pc = (n + x) as u16;
            },
            'c' => {
                let mut num: u8 = rand::thread_rng().gen_range(0..=255);
                num &= decode_hex::<u8>(instruction_hex.substring(2, 4));
                self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ] = num;
            }
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
            },
            'E' => {
                let x = self.chip8_data.var_registers[
                    decode_hex::<usize>(instruction_hex.substring(1, 2))
                ];
                if decode_hex::<u8>(instruction_hex.substring(2, 4)) == 0x9E {
                    if self.events_handler.is_pressed(x) {
                        self.chip8_data.pc += 2;
                    }
                } else {
                    if !self.events_handler.is_pressed(x) {
                        self.chip8_data.pc += 2;
                    }
                }
            },
            'F' => {
                match decode_hex::<u8>(instruction_hex.substring(2, 4)) {
                    0x07 => {
                        self.chip8_data.var_registers[
                            decode_hex::<usize>(instruction_hex.substring(1, 2))
                        ] = self.chip8_data.delay_timer;
                    },
                    0x15 => {
                        self.chip8_data.delay_timer = self.chip8_data.var_registers[
                            decode_hex::<usize>(instruction_hex.substring(1, 2))
                        ];
                    },
                    0x18 => {
                        self.chip8_data.sound_timer = self.chip8_data.var_registers[
                            decode_hex::<usize>(instruction_hex.substring(1, 2))
                        ];
                    },
                    0x1E => {
                        self.chip8_data.index =
                            decode_hex(instruction_hex.substring(1,2));
                    }
                    0x0A => {
                        if self.events_handler.events.is_empty() {
                            jumped = true;
                        } else {

                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        if !jumped {
            self.chip8_data.pc += 2;
        }
    }
}
