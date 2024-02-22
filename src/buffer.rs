pub struct MyBuffer {
    pixels: [u32; 320 * 240],
}

impl MyBuffer {
    pub fn new() -> Self {
        Self {
            pixels: [0u32; 320 * 240],
        }
    }

    pub fn clear(&mut self, color: u32) {
        for x in 0..320 {
            for y in 0..240 {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.pixels[(y * 320 + x) as usize] = color;
    }

    // pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
    //     self.pixels[(y * 320 + x) as usize]
    // }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for i in 0..w {
            for j in 0..h {
                self.set_pixel(x + i, y + j, color);
            }
        }
    }

    pub fn get_buf(&self) -> &[u32] {
        &(self.pixels)
    }
}
