#![allow(dead_code)]

mod buffer;
mod engine;
mod game;

use std::f32::consts::{PI, TAU};

use engine::{Engine, EngineAPI, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, KEY_Z};
use game::{Map, MyGameState};

fn cast_ray(gs: &mut MyGameState, angle: f32) -> (i32, i32, u8) {
    // The basic idea is to step along each grid line to check for a
    // horizontal or vertival wall. Then, return the location and map
    // id of the closer wall.
    let mut hx = gs.player.x;
    let mut hy = gs.player.y;
    let mut res_hx = -1;
    let mut res_hy = -1;
    let mut res_hcode = 0;
    let mut hsucc = false;

    let mut vx = gs.player.x;
    let mut vy = gs.player.y;
    let mut res_vx = -1;
    let mut res_vy = -1;
    let mut res_vcode = 0;
    let mut vsucc = false;

    // TODO
    // New idea, just step along in the middle of cells, then do the
    // ceiling thing after. Also, move type casting into the map
    // function for cleaner code.

    // Horizontal: look down or up
    let dy = 20.0;
    let dx = dy / angle.tan();
    let mut check_count = 0;
    if 0.0 < angle && angle < PI {
        while check_count < 20 {
            (res_hx, res_hy, res_hcode) = gs.map.check_down(hx, hy, angle);
            if res_hcode != 0 {
                hsucc = true;
                break;
            }

            hx += dx;
            hy += dy;
            check_count += 1;
        }
    } else if PI < angle && angle < TAU {
        while check_count < 20 {
            (res_hx, res_hy, res_hcode) = gs.map.check_up(hx, hy, angle);
            if res_hcode != 0 {
                hsucc = true;
                break;
            }

            hx -= dx;
            hy -= dy;
            check_count += 1;
        }
    }

    // Vertical: look right and left
    let dx = 20.0;
    let dy = dx * angle.tan();
    let mut check_count = 0;
    if PI / 2.0 < angle && angle < 3.0 * PI / 2.0 {
        while check_count < 20 {
            (res_vx, res_vy, res_vcode) = gs.map.check_left(vx, vy, angle);
            if res_vcode != 0 {
                vsucc = true;
                break;
            }

            vx -= dx;
            vy -= dy;
            check_count += 1;
        }
    } else if angle < PI / 2.0 || 3.0 * PI / 2.0 < angle {
        while check_count < 20 {
            (res_vx, res_vy, res_vcode) = gs.map.check_right(vx, vy, angle);
            if res_vcode != 0 {
                vsucc = true;
                break;
            }

            vx += dx;
            vy += dy;
            check_count += 1;
        }
    }

    // Return the results from the successful cast that was closer
    // Otherwise return the failure state
    if vsucc
        && hsucc
        && (res_vx - gs.player.x as i32).pow(2) + (res_vy - gs.player.y as i32).pow(2)
            < (res_hx - gs.player.x as i32).pow(2) + (res_hy - gs.player.y as i32).pow(2)
    {
        return (res_vx, res_vy, res_vcode);
    }

    if hsucc {
        return (res_hx, res_hy, res_hcode);
    }

    if vsucc {
        return (res_vx, res_vy, res_vcode);
    }

    (-1, -1, 0)
}

fn init(_gs: &mut MyGameState) {}

fn update(gs: &mut MyGameState, api: &EngineAPI) {
    // Move Player
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

    // Toggle map
    if api.is_key_pressed(KEY_Z) {
        gs.show_map = !gs.show_map;
    }
}

fn render(gs: &mut MyGameState) -> &[u32] {
    // Clear
    gs.buf.clear(0x000000);

    // The ray cast
    let mut rc_code = [0u8; 320];
    let mut wall_h = [0; 320];
    for i in 0..320 {
        let angle = gs.player.a - (PI / 6.0) + (i as f32 * PI / 3.0 / 320.0);

        let (ray_x, ray_y, ray_code) = cast_ray(gs, angle);
        println!("{}, {}", ray_x, ray_y);
        if ray_x != -1 && ray_y != -1 {
            rc_code[i] = ray_code;
            let d = ((gs.player.x - ray_x as f32).powi(2) + (gs.player.y - ray_y as f32).powi(2)).sqrt();
            wall_h[i] = (480.0 / (d * (angle - gs.player.a).cos())) as i32;
            if gs.show_map {
                gs.buf.draw_line(
                    gs.player.x as i32,
                    gs.player.y as i32,
                    ray_x,
                    ray_y,
                    0x3F3F3F,
                );
            }
        }
    }

    if gs.show_map {
        // Draw the map
        gs.map.draw(&mut gs.buf);

        // Draw the player
        gs.player.draw(&mut gs.buf);
    } else {
        for i in 0..320 {
            if rc_code[i] != 0 {
                gs.buf.draw_rect(i as i32, 120 - (wall_h[i] / 2), 1, wall_h[i], Map::get_wall_color(rc_code[i]));
            }
        }
    }

    gs.buf.get_buf()
}

fn main() {
    let gs = MyGameState::new();
    let ng: Engine<MyGameState> = Engine::new(320, 240, "Ray Caster", 30.0, gs);
    ng.run(init, update, render);
}
