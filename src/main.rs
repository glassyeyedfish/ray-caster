mod buffer;
mod game;
mod tetra;

use game::MyGameState;
use tetra::{Tetra, TetraAPI, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, KEY_Z};

#[rustfmt::skip]
const MAP: [[i8; 16]; 12]= [
    [1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1],
    [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [2, 0, 0, 0, 4, 0, 0, 3, 4, 3, 4, 3, 0, 0, 1, 1],
    [1, 0, 4, 0, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 2],
    [2, 0, 0, 3, 0, 0, 0, 3, 0, 4, 0, 3, 0, 0, 0, 1],
    [1, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 3, 4, 2],
    [2, 0, 0, 3, 0, 0, 0, 3, 0, 3, 0, 4, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 4, 3, 4, 0, 0, 0, 0, 0, 4, 0, 2],
    [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 3, 0, 0, 1],
    [1, 0, 0, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 1],
];

fn init(_gs: &mut MyGameState) {}

fn update(gs: &mut MyGameState, api: &TetraAPI) {
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
    for x in 0..16 {
        for y in 0..12 {
            if MAP[y as usize][x as usize] == 1 {
                gs.buf.draw_rect(x * 20, y * 20, 20, 20, 0x3F7FBF);
            } else if MAP[y as usize][x as usize] == 2 {
                gs.buf.draw_rect(x * 20, y * 20, 20, 20, 0x7F3FBF);
            } else if MAP[y as usize][x as usize] == 3 {
                gs.buf.draw_rect(x * 20, y * 20, 20, 20, 0x7FBF3F);
            } else if MAP[y as usize][x as usize] == 4 {
                gs.buf.draw_rect(x * 20, y * 20, 20, 20, 0x3FBF7F);
            }
        }
    }

    // Cast a ray
    let mut c = 0.0;
    while c < 200.0 {
        let x = gs.player.x + c * gs.player.a.cos();
        let y = gs.player.y + c * gs.player.a.sin();

        gs.buf.set_pixel(x as u32, y as u32, 0xBFBFBF);

        if MAP[(y / 20.0) as usize][(x / 20.0) as usize] != 0 {
            break;
        }

        c += 0.5;
    }

    // Draw the player
    gs.player.draw(&mut gs.buf);

    gs.buf.get_buf()
}

fn main() {
    let gs = MyGameState::new();
    let backend: Tetra<MyGameState> = Tetra::new(320, 240, "Ray Caster", 30.0, gs);
    backend.run(init, update, render);
}
