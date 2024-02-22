use std::{num::NonZeroU32, rc::Rc, time::Instant};

use softbuffer::{Context, Surface};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

pub const KEY_X: u8 = 1;
pub const KEY_Z: u8 = 1 << 1;
pub const KEY_DOWN: u8 = 1 << 2;
pub const KEY_LEFT: u8 = 1 << 3;
pub const KEY_RIGHT: u8 = 1 << 4;
pub const KEY_UP: u8 = 1 << 5;

#[derive(Debug)]
pub struct TetraAPI {
    curr_key_mask: u8,
    prev_key_mask: u8,
}

impl TetraAPI {
    fn new() -> Self {
        let curr_key_mask = 0u8;
        let prev_key_mask = 0u8;

        Self { curr_key_mask, prev_key_mask }
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        self.curr_key_mask & key != 0
    }
}

pub struct Tetra<S> {
    event_loop: EventLoop<()>,
    window: Rc<Window>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    buf_width: u32,
    buf_height: u32,
    delta_time: f32,
    instant: Instant,
    game_state: S,
    api: TetraAPI,
}

impl<S> Tetra<S> {
    pub fn new(
        buf_width: u32,
        buf_height: u32,
        win_title: &str,
        target_fps: f32,
        game_state: S,
    ) -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let builder = WindowBuilder::new()
            // .with_resizable(false)
            .with_title(win_title)
            .with_inner_size(PhysicalSize::new(buf_width * 4, buf_height * 4));
        let window = Rc::new(builder.build(&event_loop).unwrap());

        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        let delta_time = 1.0 / target_fps;
        let instant = Instant::now();

        let api = TetraAPI::new();

        Tetra {
            event_loop,
            window,
            surface,
            buf_width,
            buf_height,
            delta_time,
            instant,
            game_state,
            api,
        }
    }

    pub fn run(mut self, init: fn(&mut S), update: fn(&mut S, &TetraAPI), render: fn(&mut S) -> &[u32]) {
        init(&mut self.game_state);

        self.event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => elwt.exit(),
                Event::AboutToWait => {
                    if self.instant.elapsed().as_secs_f32() >= self.delta_time {
                        self.instant = Instant::now();
                        update(&mut self.game_state, &self.api);
                        self.api.prev_key_mask = self.api.curr_key_mask;
                        self.window.request_redraw();
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let size = self.window.inner_size();
                    self.surface
                        .resize(
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        )
                        .unwrap();
                    let mut win_buf = self.surface.buffer_mut().unwrap();
                    let buf = render(&mut self.game_state);

                    for x in 0..size.width {
                        for y in 0..size.height {
                            let buf_x = (x * self.buf_width) / size.width;
                            let buf_y = (y * self.buf_height) / size.height;
                            win_buf[(x + y * size.width) as usize] =
                                buf[(buf_x + buf_y * self.buf_width) as usize];
                        }
                    }

                    win_buf.present().unwrap();
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Key(e),
                    ..
                } => {
                    let mask: u8;
                    if let PhysicalKey::Code(c) = e.physical_key {
                        mask = Tetra::<S>::keycode_to_mask(c);

                        match e.state {
                            winit::event::ElementState::Pressed => self.api.curr_key_mask |= mask,
                            winit::event::ElementState::Released => self.api.curr_key_mask &= !mask,
                        };
                    }
                }
                _ => (),
            })
            .unwrap();
    }

    fn keycode_to_mask(code: KeyCode) -> u8 {
        match code {
            KeyCode::KeyX => 1,
            KeyCode::KeyZ => 1 << 1,
            KeyCode::ArrowDown => 1 << 2,
            KeyCode::ArrowLeft => 1 << 3,
            KeyCode::ArrowRight => 1 << 4,
            KeyCode::ArrowUp => 1 << 5,
            _ => 0,
        }
    }
}