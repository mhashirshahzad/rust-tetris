use raylib::prelude::*;

pub static LIGHT_BLUE: Color = Color {
    r: 59,
    g: 85,
    b: 162,
    a: 255,
};
pub static DARK_BLUE: Color = Color {
    r: 44,
    g: 44,
    b: 127,
    a: 255,
};

pub fn get_cell_colors() -> Vec<Color> {
    let dark_grey: Color = Color {
        r: 26,
        g: 31,
        b: 30,
        a: 255,
    };
    let green: Color = Color {
        r: 47,
        g: 230,
        b: 23,
        a: 255,
    };
    let red: Color = Color {
        r: 232,
        g: 18,
        b: 18,
        a: 255,
    };
    let orange: Color = Color {
        r: 226,
        g: 116,
        b: 17,
        a: 255,
    };
    let yellow: Color = Color {
        r: 237,
        g: 234,
        b: 4,
        a: 255,
    };
    let purple: Color = Color {
        r: 166,
        g: 0,
        b: 247,
        a: 255,
    };
    let cyan: Color = Color {
        r: 21,
        g: 204,
        b: 209,
        a: 255,
    };
    let blue: Color = Color {
        r: 13,
        g: 64,
        b: 216,
        a: 255,
    };

    vec![dark_grey, green, red, orange, yellow, purple, cyan, blue]
}
