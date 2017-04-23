#[macro_use]
extern crate nom;
use nom::le_u16;

extern crate image;
use image::{ImageLuma8, DynamicImage, GenericImage};

extern crate bit_vec;
use bit_vec::BitVec;

#[derive(PartialEq,Eq,Debug,Clone)]
pub enum RasterBitImageMode {
    Normal,
    DoubleWidth,
    DoubleHeight,
    Quadruple,
}

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct PrintRasterBitImage<'a> {
    pub mode: RasterBitImageMode,
    pub x: u16,
    pub y: u16,
    pub data: &'a [u8],
}

named!(pub print_raster_bit_image( &[u8] ) -> PrintRasterBitImage,
    do_parse!(
        tag!("\x1dv0") >>
        m: alt!(
            tag!("\x00") | tag!("\x30") |
            tag!("\x01") | tag!("\x31") |
            tag!("\x02") | tag!("\x32") |
            tag!("\x03") | tag!("\x33")
        ) >>
        x: le_u16 >>
        y: le_u16 >>
        data: take!(x as u32 * y as u32) >>
        (
            {
                let mode = match m {
                    b"\x00" | b"\x30" => RasterBitImageMode::Normal,
                    b"\x01" | b"\x31" => RasterBitImageMode::DoubleWidth,
                    b"\x02" | b"\x32" => RasterBitImageMode::DoubleHeight,
                    b"\x03" | b"\x33" => RasterBitImageMode::Quadruple,
                    _ => unreachable!()
                };
                PrintRasterBitImage{
                    mode: mode,
                    x: x,
                    y: y,
                    data: data,
                }
            }

        )
    )
);

impl<'a> PrintRasterBitImage<'a> {
    pub fn make_img(self) -> image::GrayImage {
        let mut imgbuf = image::GrayImage::from_fn(self.x as u32 * 8,
                                                   self.y as u32,
                                                   |_, _| image::Luma([255u8]));

        let width = self.x as u32 * 8;
        let mut y = 0;
        let bv = BitVec::from_bytes(self.data);
        for (i, bit) in bv.iter().enumerate() {
            let x = (i as u32) % width;
            let luma_data = if bit { 0 } else { 255 };
            imgbuf.put_pixel(x, y, image::Luma { data: [luma_data] });
            if x == width - 1 {
                y = y + 1;
            }
        }

        imgbuf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let escpos_img = include_bytes!("../assets/indian_head.escpos");
        let parsed = print_raster_bit_image(escpos_img);
        let (done, parsed_raster) = parsed.unwrap();
        // Leftovers
        assert_eq!(done, b"\n\n\n\n\x1dV\x00");
        assert_eq!(parsed_raster.mode, RasterBitImageMode::Normal);
        assert_eq!(parsed_raster.x, 64);
        assert_eq!(parsed_raster.y, 384);
        assert_eq!(parsed_raster.data.len(), 24576);
    }

    #[test]
    fn write_image() {
        let escpos_img = include_bytes!("../assets/indian_head.escpos");
        let parsed = print_raster_bit_image(escpos_img);
        let (_, parsed_raster) = parsed.unwrap();
        assert_eq!(parsed_raster.mode, RasterBitImageMode::Normal);
        assert_eq!(parsed_raster.data.len(), 24576);
        let imgbuf = parsed_raster.make_img();

        use std::fs::File;
        use std::path::Path;
        let ref mut fout = File::create(&Path::new("target/fractal.png")).unwrap();
        let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
    }
}
