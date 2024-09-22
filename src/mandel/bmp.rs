use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem;

//#[allow(dead_code)]
//#[repr(packed)]
struct BMPHeader {
    ftype: u16,
    size: u32,
    reserved: u32,
    offbits: u32,
}

//#[allow(dead_code)]
//#[repr(packed)]
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

//#[repr(packed)]
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

    //unsafe {
    //    writer.write_all(struct_to_bytes(&bmp_header)).expect("header writing failed");
    //    writer.write_all(struct_to_bytes(&bmp_info_header)).expect("info header writing failed");

    //    let pixel_size = mem::size_of::<Pixel>();
    //    let padding = ((4 - (width as usize) * pixel_size) % 4) % 4;
    //    let mut i: i32 = 1;

    //    for pixel in &pixels {
    //        writer.write_all(struct_to_bytes(pixel)).expect("failed writing pixel");

    //        if i == width {
    //            i = 1;
    //            for _ in 0..padding {
    //                let zero: [u8; 1] = [0];
    //                writer.write_all(&zero).expect("failed writing padding");
    //            }
    //        } else {
    //            i += 1;
    //        }
    //    }

    //    writer.flush().expect("flushing failed, crap leaked?");
    //}
    
    // writing bitmap file header
    writer.write_all(&bmp_header.ftype.to_le_bytes()).expect("header type writing failed");
    writer.write_all(&bmp_header.size.to_le_bytes()).expect("header size writing failed");
    writer.write_all(&bmp_header.reserved.to_le_bytes()).expect("header reserved writing failed");
    writer.write_all(&bmp_header.offbits.to_le_bytes()).expect("header offbits writing failed");

    // writing bitmap info header
    writer.write_all(&bmp_info_header.size.to_le_bytes()).expect("info header size writing failed");
    writer.write_all(&bmp_info_header.width.to_le_bytes()).expect("info header width writing failed");
    writer.write_all(&bmp_info_header.height.to_le_bytes()).expect("info header height writing failed");
    writer.write_all(&bmp_info_header.planes.to_le_bytes()).expect("info header planes writing failed");
    writer.write_all(&bmp_info_header.bitcount.to_le_bytes()).expect("info header bitcount writing failed");
    writer.write_all(&bmp_info_header.compression.to_le_bytes()).expect("info header compression writing failed");
    writer.write_all(&bmp_info_header.image_size.to_le_bytes()).expect("info header image size writing failed");
    writer.write_all(&bmp_info_header.hor_res.to_le_bytes()).expect("info header hor res writing failed");
    writer.write_all(&bmp_info_header.vert_res.to_le_bytes()).expect("info header vert res writing failed");
    writer.write_all(&bmp_info_header.color_palette.to_le_bytes()).expect("info header clr palette writing failed");
    writer.write_all(&bmp_info_header.color_imp.to_le_bytes()).expect("info header clr imp writing failed");

    // writing pixels with padding
    let pixel_size = mem::size_of::<Pixel>();
    let padding = ((4 - (width as usize) * pixel_size) % 4) % 4;
    let mut i: i32 = 1;

    for pixel in &pixels {
        writer.write_all(&pixel.b.to_le_bytes()).expect("failed writing pixel b");
        writer.write_all(&pixel.g.to_le_bytes()).expect("failed writing pixel g");
        writer.write_all(&pixel.r.to_le_bytes()).expect("failed writing pixel r");

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

//unsafe fn struct_to_bytes<T>(p: &T) -> &[u8] {
//    let ptr = p as *const T as *const u8;
//    
//    return std::slice::from_raw_parts(ptr, mem::size_of::<T>());
//}
