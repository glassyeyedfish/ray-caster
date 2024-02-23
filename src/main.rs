#![allow(dead_code)]

mod buffer;
mod engine;
mod game;

use engine::{Engine, EngineAPI, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use game::MyGameState;

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
        gs.player.a += 0.1;
    }
    if api.is_key_down(KEY_LEFT) {
        gs.player.a -= 0.1;
    }
}

fn render(gs: &mut MyGameState) -> &[u32] {
    // Clear
    gs.buf.clear(0x000000);

    // Draw the map
    gs.map.draw(&mut gs.buf);

    // Cast a ray

    // Draw the player
    gs.player.draw(&mut gs.buf);

    gs.buf.get_buf()
}

fn main() {
    let gs = MyGameState::new();
    let ng: Engine<MyGameState> = Engine::new(320, 240, "Ray Caster", 30.0, gs);
    ng.run(init, update, render);
}
