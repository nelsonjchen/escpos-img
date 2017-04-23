#[macro_use()]
extern crate nom;

pub enum RasterBitImageMode {
    Normal,
    DoubleWidth,
    DoubleHeight,
    Quadruple,
}

pub struct PrintRasterBitImage<'a> {
    pub mode: RasterBitImageMode,
    pub x: u16,
    pub y: u16,
    pub data: &'a u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
