use std::{
    f32::consts::PI,
    ops::{Div, Mul},
};

use crate::buffer::MyBuffer;

pub struct Map {
    hor_walls: [[u8; 16]; 13],
    ver_walls: [[u8; 17]; 12],
}

impl Map {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let hor_walls = [
            [1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,4,3,4,3,4,3,4,3,4,3,4,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,3,0,3,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,4,3,4,3,4,3,4,3,4,3,4],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],
        ];

        #[rustfmt::skip]
        let ver_walls = [
            [2,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,4,0,4,0,0,0,0,0,0,0,0,0,0,0,2],
            [2,0,3,0,3,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,4,0,4,0,0,0,0,0,0,0,0,0,0,0,2],
            [2,0,3,0,3,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,4,0,4,0,0,0,0,4,4,0,0,0,0,0,2],
            [2,0,3,0,3,0,0,0,0,0,0,4,4,0,0,0,1],
            [1,0,4,0,4,0,0,0,0,0,0,0,0,0,0,0,2],
            [2,0,3,0,3,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,4,0,4,0,0,0,0,0,0,0,0,0,0,0,2],
            [2,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,2],
        ];

        Self {
            hor_walls,
            ver_walls,
        }
    }

    pub fn draw(&self, buf: &mut MyBuffer) {
        // Draw the horizontal walls
        for x in 0..16 {
            for y in 0..13 {
                if self.hor_walls[y as usize][x as usize] != 0 {
                    buf.draw_rect(
                        20 * x,
                        20 * y - 1,
                        20,
                        2,
                        Map::get_wall_color(self.hor_walls[y as usize][x as usize]),
                    );
                }
            }
        }

        // Draw the vertical walls
        for y in 0..12 {
            for x in 0..17 {
                if self.ver_walls[y as usize][x as usize] != 0 {
                    buf.draw_rect(
                        20 * x - 1,
                        20 * y,
                        2,
                        20,
                        Map::get_wall_color(self.ver_walls[y as usize][x as usize]),
                    );
                }
            }
        }
    }

    pub fn check_down(&self, x: f32, y: f32, a: f32) -> (i32, i32, u8) {
        let cy = y.div(20.0).ceil().mul(20.0);
        let cx = (cy - y).div(a.tan()) + x;
        (
            cx as i32,
            cy as i32,
            self.get_hor_wall_code(cx as i32 / 20, cy as i32 / 20),
        )
    }

    pub fn check_up(&self, x: f32, y: f32, a: f32) -> (i32, i32, u8) {
        let cy = y.div(20.0).floor().mul(20.0);
        let cx = (cy - y).div(a.tan()) + x;
        (
            cx as i32,
            cy as i32,
            self.get_hor_wall_code(cx as i32 / 20, cy as i32 / 20),
        )
    }

    pub fn check_left(&self, x: f32, y: f32, a: f32) -> (i32, i32, u8) {
        let cx = x.div(20.0).floor().mul(20.0);
        let cy = (cx - x).mul(a.tan()) + y;
        (
            cx as i32,
            cy as i32,
            self.get_ver_wall_code(cx as i32 / 20, cy as i32 / 20),
        )
    }

    pub fn check_right(&self, x: f32, y: f32, a: f32) -> (i32, i32, u8) {
        let cx = x.div(20.0).ceil().mul(20.0);
        let cy = (cx - x).mul(a.tan()) + y;
        (
            cx as i32,
            cy as i32,
            self.get_ver_wall_code(cx as i32 / 20, cy as i32 / 20),
        )
    }

    pub fn get_hor_wall_code(&self, x: i32, y: i32) -> u8 {
        if (0..16).contains(&x) && (0..13).contains(&y) {
            return self.hor_walls[y as usize][x as usize];
        }
        0
    }

    pub fn get_ver_wall_code(&self, x: i32, y: i32) -> u8 {
        if (0..17).contains(&x) && (0..12).contains(&y) {
            return self.ver_walls[y as usize][x as usize];
        }
        0
    }

    pub fn get_wall_color(code: u8) -> u32 {
        match code {
            1 => 0x3F7FBF,
            2 => 0x7F3FBF,
            3 => 0x3FBF7F,
            4 => 0x7FBF3F,
            _ => 0xFF00FF,
        }
    }
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub a: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 110.0,
            y: 15.0,
            a: 3.0 * PI / 2.0,
        }
    }

    pub fn draw(&self, buf: &mut MyBuffer) {
        buf.draw_line(
            self.x as i32,
            self.y as i32,
            (self.x + 8.0 * self.a.cos()) as i32,
            (self.y + 8.0 * self.a.sin()) as i32,
            0xFF0000,
        );
        buf.draw_rect(self.x as i32 - 1, self.y as i32 - 1, 3, 3, 0xFFFFFF);
    }
}

pub struct MyGameState {
    pub buf: MyBuffer,
    pub player: Player,
    pub map: Map,
    pub show_map: bool,
}

impl MyGameState {
    pub fn new() -> Self {
        let buf = MyBuffer::new();
        let player = Player::new();
        let map = Map::new();
        let show_map = false;

        MyGameState { buf, player, map, show_map }
    }
}
