extern crate sdl2;

use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::memory::*;

const CONTROL_REG: u16 = 0xFF40;
const STATUS_REG: u16 = 0xFF41;

const LY : u16 = 0xFF44;

enum ModeFlag {
    HBLANK,
    VBLANK,
    OAMRAM,
    DATATOLCD
}

fn get_bit(value: u8, offset: u8, bit_value: u8) -> bool {
    let ret_val = (value & bit_value) >> offset;
    ret_val != 0
}

pub struct Tile {
    value: [u16; 8],
}

struct ControlRegister {
    lcd_enable: bool,
    win_tile_map_display: bool,  // (0=0x9800-0x9BFF, 1=0x9C00-0x9FFF)
    window_display_enable: bool,
    bg_win_tile_data: bool,     // (false=0x8800-0x97FF, true=0x8000-0x8FFF)
    bg_tile_map_display: bool, // (false=0x9800-0x9BFF, true=0x9C00-0x9FFF)
    sprite_size: bool,        // (false=8x8, true=8x16)
    sprite_enable: bool,
    bg_enable:bool,
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister {
            lcd_enable: false,
            win_tile_map_display:false,
            window_display_enable: false,
            bg_win_tile_data: false,
            bg_tile_map_display: false,
            sprite_size: false,
            sprite_enable: false,
            bg_enable: false,
        }
    }
}

struct LCDStatusRegister {
    coin_interrupt: bool,
    oam_interrupt: bool,
    v_blank_interrupt: bool,
    h_blank_interrupt: bool,
    coin_flag: bool,
    mode_flag: ModeFlag
}

impl LCDStatusRegister {
    pub fn new() -> Self {
        LCDStatusRegister {
            coin_interrupt: false,
            oam_interrupt: false,
            v_blank_interrupt: false,
            h_blank_interrupt: false,
            coin_flag: false,
            mode_flag: ModeFlag::HBLANK
        }
    }
}

pub struct PPU {
    canvas: Canvas<Window>,
    control_reg: ControlRegister,
    lcd_stat_reg: LCDStatusRegister
}

impl PPU {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("dustboy", 160, 144)
                                    .position_centered()
                                    .build()
                                    .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        PPU { 
              canvas,
              control_reg: ControlRegister::new(),
              lcd_stat_reg: LCDStatusRegister::new()
            }
    }

    pub fn render(&mut self,  memory_bus: &mut Memory) {
        self.set_control_registers(memory_bus.read_memory(CONTROL_REG as usize));
        self.set_lcd_stat_registers(memory_bus.read_memory(STATUS_REG as usize));
    
        memory_bus.write_memory(LY as usize, 0x90);
    }

    fn set_control_registers(&mut self, reg_value: u8) {
        self.control_reg.lcd_enable = get_bit(reg_value, 7, 0x80);
        self.control_reg.win_tile_map_display = get_bit(reg_value, 6, 0x40);
        self.control_reg.window_display_enable = get_bit(reg_value, 5, 0x20);
        self.control_reg.bg_win_tile_data = get_bit(reg_value, 4, 0x10);
        self.control_reg.bg_tile_map_display = get_bit(reg_value, 3, 0x08);
        self.control_reg.sprite_size = get_bit(reg_value, 2, 0x04);
        self.control_reg.sprite_enable = get_bit(reg_value, 1, 0x02);
        self.control_reg.bg_enable = get_bit(reg_value, 0, 0x01);
    }

    fn set_lcd_stat_registers(&mut self, reg_value: u8) {
        self.lcd_stat_reg.coin_interrupt = get_bit(reg_value, 6, 0x40);
        self.lcd_stat_reg.oam_interrupt = get_bit(reg_value, 5, 0x20);
        self.lcd_stat_reg.v_blank_interrupt = get_bit(reg_value, 4, 0x10);
        self.lcd_stat_reg.h_blank_interrupt = get_bit(reg_value, 3, 0x08);
        self.lcd_stat_reg.coin_flag = get_bit(reg_value, 2, 0x04);

        let mode = reg_value & 0x03;

        match mode {
            0x00 => self.lcd_stat_reg.mode_flag = ModeFlag::HBLANK,
            0x01 => self.lcd_stat_reg.mode_flag = ModeFlag::VBLANK,
            0x02 => self.lcd_stat_reg.mode_flag = ModeFlag::OAMRAM,
            0x03 => self.lcd_stat_reg.mode_flag = ModeFlag::DATATOLCD,

            _ => panic!("This value {:X} has no match in LCD Mode Flag", mode)
        };
    }

    pub fn display(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}