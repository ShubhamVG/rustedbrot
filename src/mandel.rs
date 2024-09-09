use std::num::Wrapping;

pub mod bmp;

pub fn generate_image(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    width: i32,
    height: i32,
) {
    let mut z_real: f64 = x1;
    let mut z_imag: f64 = y1;
    let dx: f64 = (x2 - x1) / (width as f64);
    let dy: f64 = (y2 - y1) / (height as f64);
    let mut pixels: Vec<bmp::Pixel> = Vec::with_capacity((height * width) as usize);

    for _ in 0..height {
        for _ in 0..width {
            let count: u8 = iter_count(z_real, z_imag);
            pixels.push(bmp::Pixel {
                r: count,
                g: Wrapping(count * 4).0,
                b: Wrapping(count * 9).0,
            });

            z_real += dx;
        }

        z_imag += dy;
        z_real = x1;
    }

    bmp::new_bmp(height, width, pixels);
}

fn iter_count(cr: f64, ci: f64) -> u8 {
    let mut zr: f64 = cr;
    let mut zi: f64 = ci;

    for i in 0..255 {
        let zr_sq: f64 = zr * zr;
        let zi_sq: f64 = zi * zi;

        if zr_sq + zi_sq >= 4.0 {
            return i as u8;
        }

        zi = 2.0 * zr * zi;
        zr = zr_sq - zi_sq;
        zr += cr;
        zi += ci;
    }

    return 0;
}
