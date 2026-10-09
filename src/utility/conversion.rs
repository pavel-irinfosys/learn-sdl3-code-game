use sdl3::pixels::Color;
use crate::utility::ConvertColor;

impl ConvertColor for u32 {
    fn to_sdl_color(self) -> Color {
        from_u32_to_color(self)
    }
}

impl ConvertColor for &str {
    fn to_sdl_color(self) -> Color {
        from_str_to_color(self)
    }
}

const FALL_BACK_COLOR: Color = Color::RGB(0, 0, 0);
fn from_str_to_color(hex_code: &str) -> Color {
    if hex_code.is_empty() {
        return FALL_BACK_COLOR;
    }
    let hex = hex_code.trim().trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
            Color::RGB(r, g, b)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
            Color::RGBA(r, g, b, a)
        }
        _ => FALL_BACK_COLOR
    }
}

fn from_u32_to_color(hex_code: u32) -> Color {
    Color::RGB((hex_code >> 16) as u8, (hex_code >> 8) as u8, hex_code as u8)
}
