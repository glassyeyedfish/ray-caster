use std::f32::consts::PI;

use crate::buffer::MyBuffer;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub a: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 30.0,
            y: 30.0,
            a: PI / 4.0,
        }
    }

    pub fn draw(&self, buf: &mut MyBuffer) {
        buf.draw_rect(self.x as u32 - 1, self.y as u32 - 1, 3, 3, 0xFFFFFF);
    }
}

pub struct MyGameState {
    pub buf: MyBuffer,
    pub player: Player,
}

impl MyGameState {
    pub fn new() -> Self {
        let buf = MyBuffer::new();
        let player = Player::new();

        MyGameState { buf, player }
    }
}
