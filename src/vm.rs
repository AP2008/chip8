use super::delay::Delay;
use super::display::Display;
use super::keyboard::Keyboard;
use super::sound::Sound;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use sdl2::TimerSubsystem;
use std::fs;
use std::include_bytes;
use std::time::Duration;

const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGS_SIZE: usize = 16;
const OFFSET: usize = 0x200;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    memory: [u8; MEM_SIZE],
    display: Display,
    pc: u16,
    index_reg: u16,
    stack: [u16; STACK_SIZE],
    delay_timer: Delay,
    sound_timer: Sound,
    registers: [u8; REGS_SIZE],
    sp: u8,
    rng: ThreadRng,
    keyboard: Keyboard,
    fps_timer: TimerSubsystem,
    file_name: String,
}

impl Chip8 {
    pub fn new(sdl_context: sdl2::Sdl) -> Self {
        let mut memory = [0; MEM_SIZE];
        for (i, v) in FONT.iter().enumerate() {
            memory[i] = *v;
        }
        Chip8 {
            memory,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: Delay::new(),
            sound_timer: Sound::new(&sdl_context),
            registers: [0; REGS_SIZE],
            pc: OFFSET as u16,
            display: Display::new(&sdl_context),
            index_reg: 0,
            rng: thread_rng(),
            keyboard: Keyboard::new(&sdl_context),
            fps_timer: sdl_context.timer().unwrap(),
            file_name: String::new(),
        }
    }
    pub fn load_rom(&mut self, file_name: Option<&String>) {
        let memory = if let Some(fname) = file_name {
            fs::read(fname).unwrap()
        } else {
            include_bytes!("../roms/Programs/Chip8 Picture.ch8").to_vec()
        };
        self.file_name = file_name
            .unwrap_or(&"Chip8 Picture.ch8".to_string())
            .clone();
        self.display.title(&self.file_name);
        for (i, m) in memory.iter().enumerate() {
            self.memory[OFFSET + i] = *m;
        }
    }
    fn add_with_carry(&mut self, a: u8, b: u8) -> u8 {
        let s = (a as u16) + (b as u16);
        if s >= 0x100 {
            self.registers[0xf] = 1;
            s as u8
        } else {
            self.registers[0xf] = 0;
            s as u8
        }
    }
    fn sub_with_carry(&mut self, a: u8, b: u8) -> u8 {
        self.registers[0xf] = if a > b { 1 } else { 0 };
        a.wrapping_sub(b)
    }
    pub fn execute(&mut self) -> Option<u16> {
        self.delay_timer.check();
        self.sound_timer.check();
        let instr: u16 = ((self.memory[self.pc as usize] as u16) << 8)
            + self.memory[self.pc as usize + 1] as u16;
        let nimbles = (
            (instr & 0xf000) >> 12,
            (instr & 0x0f00) >> 8,
            (instr & 0x00f0) >> 4,
            (instr & 0x000f),
        );
        let nnn = instr & 0x0fff;
        let kk = (instr & 0x00ff) as u8;
        let vx = *self.registers.get(nimbles.1 as usize).unwrap_or(&0);
        let vy = *self.registers.get(nimbles.2 as usize).unwrap_or(&0);

        match (nimbles.0, nimbles.1, nimbles.2, nimbles.3) {
            (0x0, 0x0, 0xe, 0x0) => self.display.clear(),
            (0x0, 0x0, 0xe, 0xe) => {
                self.sp -= 1;
                return Some(self.stack[self.sp as usize + 1]);
            }
            (0x1, _, _, _) => return Some(nnn),
            (0x2, _, _, _) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc + 2;
                return Some(nnn);
            }
            (0x3, _, _, _) => return Some(self.pc + if vx == kk { 4 } else { 2 }),
            (0x4, _, _, _) => return Some(self.pc + if vx != kk { 4 } else { 2 }),
            (0x5, _, _, 0x0) => return Some(self.pc + if vx == vy { 4 } else { 2 }),
            (0x6, _, _, _) => self.registers[nimbles.1 as usize] = kk as u8,
            (0x7, _, _, _) => self.registers[nimbles.1 as usize] += kk as u8,
            (0x8, _, _, 0x0) => self.registers[nimbles.1 as usize] = vy,
            (0x8, _, _, 0x1) => self.registers[nimbles.1 as usize] |= vy,
            (0x8, _, _, 0x2) => self.registers[nimbles.1 as usize] &= vy,
            (0x8, _, _, 0x3) => self.registers[nimbles.1 as usize] ^= vy,
            (0x8, _, _, 0x4) => self.registers[nimbles.1 as usize] = self.add_with_carry(vx, vy),
            (0x8, _, _, 0x5) => self.registers[nimbles.1 as usize] = self.sub_with_carry(vx, vy),
            (0x8, _, _, 0x7) => self.registers[nimbles.1 as usize] = self.sub_with_carry(vy, vx),
            (0x8, _, _, 0x6) => {
                self.registers[0xf] = vx & 1;
                self.registers[nimbles.1 as usize] >>= 1;
            }
            (0x8, _, _, 0xE) => {
                self.registers[0xf] = (vx & 0b10000000) >> 7;
                self.registers[nimbles.1 as usize] <<= 1;
            }
            (0x9, _, _, 0x0) => return Some(self.pc + if vx != vy { 4 } else { 2 }),
            (0xA, _, _, _) => self.index_reg = nnn,
            (0xB, _, _, _) => return Some(nnn + self.registers[0] as u16),
            (0xC, _, _, _) => self.registers[nimbles.1 as usize] = self.rng.gen::<u8>() & kk,
            (0xD, _, _, _) => {
                let x_coord = vx % (DISPLAY_WIDTH as u8);
                let y_coord = vy % (DISPLAY_HEIGHT as u8);
                let n = nimbles.3;
                self.registers[0xf] = 0;
                let mut conflict = false;
                for i in 0..n {
                    let sprite = self.memory[(self.index_reg + i) as usize];
                    for j in 1..=8 {
                        let status = (sprite & (1 << (8 - j))) != 0;
                        if x_coord + j - 1 < (DISPLAY_WIDTH as u8)
                            && y_coord + (i as u8) < (DISPLAY_HEIGHT as u8)
                        {
                            let v = self
                                .display
                                .draw_point((x_coord + j - 1, y_coord + (i as u8)), status);
                            conflict |= v;
                        }
                    }
                }
                self.registers[0xf] = if conflict { 1 } else { 0 };
                self.display.present();
            }
            (0xE, _, 0x9, 0xE) => {
                return Some(
                    self.pc
                        + if let Some(x) = self.keyboard.get_last_key() {
                            if x == vx {
                                self.keyboard.clear();
                                4
                            } else {
                                2
                            }
                        } else {
                            2
                        },
                )
            }
            (0xE, _, 0xA, 0x1) => {
                return Some(
                    self.pc
                        + if let Some(x) = self.keyboard.get_last_key() {
                            if x != vx {
                                4
                            } else {
                                self.keyboard.clear();
                                2
                            }
                        } else {
                            4
                        },
                )
            }
            (0xF, _, 0x1, 0xE) => self.index_reg += vx as u16,
            (0xF, _, 0x0, 0x7) => self.registers[nimbles.1 as usize] = self.delay_timer.get_count(),
            (0xF, _, 0x1, 0x5) => self.delay_timer.set_count(vx),
            (0xF, _, 0x0, 0xa) => {
                let v = self.keyboard.get_last_key();
                if let Some(n) = v {
                    self.keyboard.clear();
                    self.registers[nimbles.1 as usize] = n;
                } else {
                    return Some(self.pc);
                }
            }
            (0xF, _, 0x2, 0x9) => self.index_reg = (vx as u16) * 5,
            (0xF, _, 0x3, 0x3) => {
                (
                    self.memory[self.index_reg as usize],
                    self.memory[self.index_reg as usize + 1],
                    self.memory[self.index_reg as usize + 2],
                ) = ((vx / 100) as u8, ((vx / 10) as u8) % 10, vx % 10);
            }
            (0xF, _, 0x5, 0x5) => {
                for i in 0..=(nimbles.1 as usize) {
                    self.memory[self.index_reg as usize + i] = self.registers[i];
                }
            }
            (0xF, _, 0x6, 0x5) => {
                for i in 0..=(nimbles.1 as usize) {
                    self.registers[i] = self.memory[self.index_reg as usize + i];
                }
            }
            (0xF, _, 0x1, 0x8) => self.sound_timer.set_count(vx),
            (0x0, _, _, _) => return Some(u16::MAX),
            _ => panic!("Opcode {} not found", instr),
        }
        None
    }
    pub fn main_loop(mut self) {
        let mut t = vec![];
        let mut update = self.fps_timer.ticks();
        let fps = self.fps_timer.ticks();
        let mut frames = 0;
        while (self.pc as usize) < self.memory.len() && !t.contains(&16) {
            if t.contains(&17) {
                self.keyboard.rem_val(17);
                self.display.pres();
            }
            if let Some(p) = self.execute() {
                self.pc = p;
            } else {
                self.pc += 2;
            }
            std::thread::sleep(Duration::from_millis(2));
            if self.fps_timer.ticks() - update > 1000 {
                let l =
                    ((frames as f64) / ((self.fps_timer.ticks() - fps) as f64 / 1000.0)) as usize;
                self.display
                    .title(&format!("{}... - {} FPS", &self.file_name[..15], l));
                update = self.fps_timer.ticks();
            }
            frames += 1;
            t = self.keyboard.check().clone();
        }
    }
}
