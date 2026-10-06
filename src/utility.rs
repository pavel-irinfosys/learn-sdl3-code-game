use sdl3::pixels::Color;

mod conversion;

pub trait ConvertColor {
    fn to_sdl_color(self) -> Color;
}
