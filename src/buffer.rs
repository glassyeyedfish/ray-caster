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
                self.pixels[(x + y * 320) as usize] = color;
            }
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.pixels[(x + y * 320) as usize] = color;
    }

    // pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
    //     self.pixels[(y * 320 + x) as usize]
    // }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for i in x..w+x {
            for j in y..y+h {
                self.pixels[(i + j * 320) as usize] = color;
            }
        }
    }

    // Bresenham's line drawing algorithm.
    // Implementation taken from:
    // https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm
    pub fn draw_line(&mut self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, color: u32) {
        let mut steep = false;

        if (x1 - x2).abs() < (y1 - y2).abs() {
            (x1, y1) = (y1, x1);
            (x2, y2) = (y2, x2);
            steep = true;
        }

        if x1 > x2 {
            (x1, x2) = (x2, x1);
            (y1, y2) = (y2, y1);
        }

        let dx = x2 - x1;
        let dy = y2 - y1;

        let derr = 2 * dy.abs();
        let mut err = 0;

        let mut y = y1;

        if steep {
            for x in x1..=x2 {
                self.set_pixel(y, x, color);
                err += derr;

                if err > dx {
                    y += if y2 > y1 { 1 } else { -1 };
                    err -= 2*dx;
                }
            }
        } else {
            for x in x1..=x2 {
                self.set_pixel(x, y, color);
                err += derr;

                if err > dx {
                    y += if y2 > y1 { 1 } else { -1 };
                    err -= 2*dx;
                }
            }
        }
    }

    pub fn get_buf(&self) -> &[u32] {
        &(self.pixels)
    }
}
