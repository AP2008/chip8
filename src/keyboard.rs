use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode::*;
use sdl2::EventPump;

pub struct Keyboard {
    events: EventPump,
    keys: Vec<u8>,
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Keyboard {
            events: sdl_context.event_pump().unwrap(),
            keys: Vec::new(),
        }
    }
    pub fn check(&mut self) -> &Vec<u8> {
        self.keys.append(
            &mut self
                .events
                .poll_iter()
                .map(Self::encode_event)
                .flatten()
                .collect(),
        );
        &self.keys
    }
    pub fn clear(&mut self) {
        self.keys = Vec::new();
    }
    pub fn rem_val(&mut self, v: u8) {
        self.keys.retain(|&x| x != v);
    }
    fn encode_event(event: sdl2::event::Event) -> Option<u8> {
        match event {
            Event::KeyDown {
                keycode: Some(A), ..
            } => Some(0xA),
            Event::KeyDown {
                keycode: Some(B), ..
            } => Some(0xB),
            Event::KeyDown {
                keycode: Some(C), ..
            } => Some(0xC),
            Event::KeyDown {
                keycode: Some(D), ..
            } => Some(0xD),
            Event::KeyDown {
                keycode: Some(E), ..
            } => Some(0xE),
            Event::KeyDown {
                keycode: Some(F), ..
            } => Some(0xF),
            Event::KeyDown {
                keycode: Some(Num0),
                ..
            } => Some(0x0),
            Event::KeyDown {
                keycode: Some(Num1),
                ..
            } => Some(0x1),
            Event::KeyDown {
                keycode: Some(Num2),
                ..
            } => Some(0x2),
            Event::KeyDown {
                keycode: Some(Num3),
                ..
            } => Some(0x3),
            Event::KeyDown {
                keycode: Some(Num4),
                ..
            } => Some(0x4),
            Event::KeyDown {
                keycode: Some(Num5),
                ..
            } => Some(0x5),
            Event::KeyDown {
                keycode: Some(Num6),
                ..
            } => Some(0x6),
            Event::KeyDown {
                keycode: Some(Num7),
                ..
            } => Some(0x7),
            Event::KeyDown {
                keycode: Some(Num8),
                ..
            } => Some(0x8),
            Event::KeyDown {
                keycode: Some(Num9),
                ..
            } => Some(0x9),
            Event::Window {
                win_event:
                    WindowEvent::FocusGained
                    | WindowEvent::Resized(..)
                    | WindowEvent::Moved(..)
                    | WindowEvent::Enter
                    | WindowEvent::Exposed,
                ..
            } => Some(17),
            Event::Quit { .. } => Some(16),
            _ => None,
        }
    }
    pub fn get_last_key(&mut self) -> Option<u8> {
        let key = self.check().last().copied();
        key
    }
}
