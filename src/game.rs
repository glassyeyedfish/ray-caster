use std::f32::consts::PI;

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

        Self{ hor_walls, ver_walls }
    }

    pub fn draw(&self, buf: &mut MyBuffer) {
        // Draw horizontal wallk on the top 
        for x in 0..16 {
            if self.hor_walls[0][x as usize] != 0 {
                buf.draw_rect(20 * x, 0, 20, 1, Map::get_wall_color(self.hor_walls[0][x as usize]));
            }
        }

        // Draw horizontal wallk on the bottom
        for x in 0..16 {
            if self.hor_walls[12][x as usize] != 0 {
                buf.draw_rect(20 * x, 239, 20, 1, Map::get_wall_color(self.hor_walls[12][x as usize]));
            }
        }

        // Draw the rest of the horizontal walls
        for x in 0..16 {
            for y in 1..12 {
                if self.hor_walls[y as usize][x as usize] != 0 {
                    buf.draw_rect(20 * x, 20 * y - 1, 20, 2, Map::get_wall_color(self.hor_walls[y as usize][x as usize]));
                }
            }
        }

        // Draw vertical walls on the left
        for y in 0..12 {
            if self.ver_walls[y as usize][0] != 0 {
                buf.draw_rect(0, 20 * y, 1, 20, Map::get_wall_color(self.ver_walls[y as usize][0]));
            }
        }
        
        // Draw vertical walls on the right
        for y in 0..12 {
            if self.ver_walls[y as usize][16] != 0 {
                buf.draw_rect(319, 20 * y, 1, 20, Map::get_wall_color(self.ver_walls[y as usize][16]));
            }
        }
        
        // Draw the rest of the vertical walls
        for y in 0..12 {
            for x in 1..16 {
                if self.ver_walls[y as usize][x as usize] != 0 {
                    buf.draw_rect(20 * x - 1, 20 * y, 2, 20, Map::get_wall_color(self.ver_walls[y as usize][x as usize]));
                }
            }
        }
    }

    fn get_wall_color(code: u8) -> u32 {
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
            x: 30.0,
            y: 30.0,
            a: PI / 4.0,
        }
    }

    pub fn draw(&self, buf: &mut MyBuffer, color: u32) {
        buf.draw_rect(self.x as i32 - 1, self.y as i32 - 1, 3, 3, color);
    }
}

pub struct MyGameState {
    pub buf: MyBuffer,
    pub player: Player,
    pub map: Map,
}

impl MyGameState {
    pub fn new() -> Self {
        let buf = MyBuffer::new();
        let player = Player::new();
        let map = Map::new();

        MyGameState { buf, player, map }
    }
}
