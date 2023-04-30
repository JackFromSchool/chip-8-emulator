extern crate sdl2;

use sdl2::{video::Window, pixels::Color, rect::{Point, Rect}};

use super::PIXEL_SIZE;

pub struct CanvasUtils {
    handle: sdl2::render::Canvas<Window>,
    pixel_data: [[bool; 64]; 32],
}

impl CanvasUtils {
    
    pub fn new(window: Window) -> Self {
        let handle = window.into_canvas()
            .build()
            .unwrap();

        let pixel_data = [[false; 64]; 32];

        Self {
            handle,
            pixel_data,
        }
    }

    pub fn clear_screen(&mut self) {
        self.handle.set_draw_color(Color::RGB(0, 0, 0));
        self.handle.clear();
    }

    pub fn invert_pixel(&mut self, x:usize, y:usize) -> bool {
        if self.pixel_data[y][x] == true {
            self.pixel_data[y][x] = false;
            true
        } else {
            self.pixel_data[y][x] = true;
            false
        }
    }

    pub fn update(&mut self) {
        for y in 0..self.pixel_data.len() {
            for x in 0..self.pixel_data[y].len() {
                    
                if self.pixel_data[y][x] {
                    self.handle.set_draw_color(Color::WHITE);
                } else {
                    self.handle.set_draw_color(Color::BLACK);
                }
                
                self.handle.fill_rect(Rect::new(
                        (x as u32*PIXEL_SIZE) as i32, 
                        (y as u32*PIXEL_SIZE) as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE
                    )).unwrap();

            }
        }
        self.handle.present();
    }
}
