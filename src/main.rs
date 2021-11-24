#![windows_subsystem = "windows"]
#![feature(destructuring_assignment)]
mod delay;
mod display;
mod keyboard;
mod sound;
mod vm;

use std::env;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let args = env::args().collect::<Vec<_>>();
    let mut vm = vm::Chip8::new(sdl_context);
    vm.load_rom(args.get(1));
    vm.main_loop();
}
