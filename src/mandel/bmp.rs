use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem;

#[allow(dead_code)]
#[repr(packed)]
struct BMPHeader {
    ftype: u16,
    size: u32,
    reserved: u32,
    offbits: u32,
}

#[allow(dead_code)]
#[repr(packed)]
struct BMPInfoHeader {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bitcount: u16,
    compression: u32,
    image_size: u32,
    hor_res: i32,
    vert_res: i32,
    color_palette: u32,
    color_imp: u32,
}

#[repr(packed)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn new_bmp(height: i32, width: i32, pixels: Vec<Pixel>) {
    let file = File::create("output.bmp").expect("file failed");
    let mut writer = BufWriter::new(file);

    let img_size: u32 = (height * width) as u32;
    let bmp_header = BMPHeader {
        ftype: 0x4d42,
        size: 14 + 40 + img_size,
        reserved: 0,
        offbits: 54,
    };
    let bmp_info_header = BMPInfoHeader {
        size: 40,
        width: width,
        height: height,
        planes: 1,
        bitcount: 24,
        compression: 0,
        image_size: img_size,
        hor_res: 11811,
        vert_res: 11811,
        color_palette: 0,
        color_imp: 0,
    };

    unsafe {
        writer.write_all(struct_to_bytes(&bmp_header)).expect("header writing failed");
        writer.write_all(struct_to_bytes(&bmp_info_header)).expect("info header writing failed");

        let pixel_size = mem::size_of::<Pixel>();
        let padding = ((4 - (width as usize) * pixel_size) % 4) % 4;
        let mut i: i32 = 1;

        for pixel in &pixels {
            writer.write_all(struct_to_bytes(pixel)).expect("failed writing pixel");

            if i == width {
                i = 1;
                for _ in 0..padding {
                    let zero: [u8; 1] = [0];
                    writer.write_all(&zero).expect("failed writing padding");
                }
            } else {
                i += 1;
            }
        }

        writer.flush().expect("flushing failed, crap leaked?");
    }
}

unsafe fn struct_to_bytes<T>(p: &T) -> &[u8] {
    let ptr = p as *const T as *const u8;
    
    return std::slice::from_raw_parts(ptr, mem::size_of::<T>());
}
