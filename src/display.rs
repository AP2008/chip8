use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const DISPLAY_WIDTH: u32 = 64;
const DISPLAY_HEIGHT: u32 = 32;
const SCALE: u32 = 16;

pub struct Display {
    canvas: Canvas<Window>,
    display: [[u8; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
    scale: u32,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "CHIP-8 Emulator",
                SCALE * DISPLAY_WIDTH,
                SCALE * DISPLAY_HEIGHT,
            )
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        Self {
            canvas,
            display: [[0; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
            scale: SCALE,
        }
    }
    pub fn title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(title).unwrap();
    }
    pub fn clear(&mut self) {
        self.display
            .iter_mut()
            .for_each(|x| x.iter_mut().for_each(|y| *y = 0));
        self.present();
    }
    pub fn draw_point(&mut self, point: (u8, u8), status: bool) -> bool {
        let t = self.display[point.1 as usize][point.0 as usize];
        if status {
            self.display[point.1 as usize][point.0 as usize] = 1;
        }
        if status && (t == 1) {
            self.display[point.1 as usize][point.0 as usize] = 0;
            return true;
        }
        false
    }
    pub fn present(&mut self) {
        self.canvas.clear();
        for (i, v) in self.display.iter().enumerate() {
            for (j, v2) in v.iter().enumerate() {
                if *v2 == 1 {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
                let i = self.scale * (i as u32);
                let j = self.scale * (j as u32);
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        j as i32,
                        i as i32,
                        (j + self.scale) as u32,
                        (i + self.scale) as u32,
                    ))
                    .unwrap();
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
        }
        self.canvas.present();
    }
    pub fn pres(&mut self) {
        self.canvas.present();
    }
}
