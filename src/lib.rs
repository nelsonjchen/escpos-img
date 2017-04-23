#[macro_use]
extern crate nom;
use nom::le_u16;

extern crate image;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let escpos_img = include_bytes!("../assets/indian_head.escpos");
        let parsed = print_raster_bit_image(escpos_img);
        let (_, parsed_raster) = parsed.unwrap();
        assert_eq!(parsed_raster.mode, RasterBitImageMode::Normal);
        assert_eq!(parsed_raster.data.len(), 24576);
    }
}
