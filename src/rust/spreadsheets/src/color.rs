use super::*;

pub fn color_from_code(code: &str) -> Color {
    match code.len() {
        3 => Color {
            red: Some(color_component_from_code(&code[0..1])),
            green: Some(color_component_from_code(&code[1..2])),
            blue: Some(color_component_from_code(&code[2..3])),
            ..Color::default()
        },
        6 => Color {
            red: Some(color_component_from_code(&code[0..2])),
            green: Some(color_component_from_code(&code[2..4])),
            blue: Some(color_component_from_code(&code[4..6])),
            ..Color::default()
        },
        _ => unreachable!(),
    }
}

fn color_component_from_code(code: &str) -> f32 {
    match code.len() {
        2 => u8::from_str_radix(&code[0..2], 16).unwrap() as f32 / 255f32,
        1 => {
            let i = u8::from_str_radix(&code[0..1], 16).unwrap();
            let i = i * 16 + i;
            i as f32 / 255f32
        }
        len => unreachable!("{len}"),
    }
}
