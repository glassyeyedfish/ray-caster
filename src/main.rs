#![allow(dead_code)]

mod buffer;
mod engine;
mod game;

use std::f32::consts::{PI, TAU};

use engine::{Engine, EngineAPI, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use game::MyGameState;

fn cast_ray(gs: &mut MyGameState) -> (i32, i32, u8) {
    // The basic idea is to step along each grid line to check for a 
    // horizontal or vertival wall. Then, return the location and map 
    // id of the closer wall.
    let mut hx = gs.player.x;
    let mut hy = gs.player.y;

    // TODO
    // New idea, just step along in the middle of cells, then do the 
    // ceiling thing after. Also, move type casting into the map 
    // function for cleaner code.

    // Horizontal: look down or up
    if 0.0 < gs.player.a && gs.player.a < PI {
        hy = (gs.player.y / 20.0).ceil() * 20.0;
        hx = (hy - gs.player.y) / gs.player.a.tan() + gs.player.x;

        if hx < 0.0 || hx > 320.0 {
            return (-1, -1, 0);
        }
        
        let mut ray_map = gs.map.hor_wall_at(hx as i32, hy as i32);
        if ray_map != 0 {
            return (hx as i32, hy as i32 - 1, ray_map);
        }

        let dy = 20.0;
        let dx = dy / gs.player.a.tan();

        if hx + dx < 0.0 || hx + dx > 320.0 {
            return (-1, -1, 0);
        }

        loop {
            hx += dx;
            hy += dy;

            ray_map = gs.map.hor_wall_at(hx as i32, hy as i32);
            if ray_map != 0 {
                return (hx as i32, hy as i32 - 1, ray_map);
            }

            if hx + dx < 0.0 || hx + dx > 320.0 || hy + dy > 240.0 {
                return (-1, -1, 0);
            }
        }
    } else if PI < gs.player.a && gs.player.a < TAU {
        hy = (gs.player.y / 20.0).floor() * 20.0;
        hx = (hy - gs.player.y) / gs.player.a.tan() + gs.player.x;

        if hx < 0.0 || hx > 320.0 {
            return (-1, -1, 0);
        }
        
        let mut ray_map = gs.map.hor_wall_at(hx as i32, hy as i32);
        if ray_map != 0 {
            return (hx as i32, hy as i32 + 1, ray_map);
        }

        let dy = 20.0;
        let dx = dy / gs.player.a.tan();

        if hx - dx < 0.0 || hx - dx > 320.0 {
            return (-1, -1, 0);
        }

        loop {
            hx -= dx;
            hy -= dy;

            ray_map = gs.map.hor_wall_at(hx as i32, hy as i32);
            if ray_map != 0 {
                return (hx as i32, hy as i32 + 1, ray_map);
            }

            if hx - dx < 0.0 || hx - dx > 320.0 || hy - dy < 0.0 {
                return (-1, -1, 0);
            }
        }
    }
    
    (-1, -1, 0)
}

fn init(_gs: &mut MyGameState) {}

fn update(gs: &mut MyGameState, api: &EngineAPI) {
    if api.is_key_down(KEY_UP) {
        gs.player.x += 1.5 * gs.player.a.cos();
        gs.player.y += 1.5 * gs.player.a.sin();
    }
    if api.is_key_down(KEY_DOWN) {
        gs.player.x -= gs.player.a.cos();
        gs.player.y -= gs.player.a.sin();
    }
    if api.is_key_down(KEY_RIGHT) {
        gs.player.a = (gs.player.a + 0.1) % TAU;
    }
    if api.is_key_down(KEY_LEFT) {
        gs.player.a = (gs.player.a - 0.1).rem_euclid(TAU);
    }
}

fn render(gs: &mut MyGameState) -> &[u32] {
    // Clear
    gs.buf.clear(0x000000);

    // Draw the map
    gs.map.draw(&mut gs.buf);

    // Cast a ray to the horizontal walls:
    let (ray_x, ray_y, ray_map) = cast_ray(gs);
    println!("{}, {}", ray_x, ray_y);
    if ray_x != -1 && ray_y != -1 {
        gs.buf.draw_line(gs.player.x as i32, gs.player.y as i32, ray_x, ray_y, 0x00FF00);
    }

    // Draw the player
    gs.player.draw(&mut gs.buf);

    gs.buf.get_buf()
}

fn main() {
    let gs = MyGameState::new();
    let ng: Engine<MyGameState> = Engine::new(320, 240, "Ray Caster", 30.0, gs);
    ng.run(init, update, render);
}
