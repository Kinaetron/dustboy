extern crate sdl2;

use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct PPU {
    canvas: Canvas<Window>,
}

impl PPU {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("dustboy gb emulator", 160, 144)
                                    .position_centered()
                                    .build()
                                    .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        PPU { canvas }
    }

    pub fn display(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}