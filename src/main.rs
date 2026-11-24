use csdiy_eocs::computer::{SCREEN_HEIGHT, SCREEN_WIDTH, Screen, ScreenWindow};
use std::time::Duration;

fn main() -> Result<(), minifb::Error> {
    let mut screen = Screen::new();
    draw_text(&mut screen, "Welcome to Hack");

    let mut window = ScreenWindow::new()?;
    while window.is_open() {
        window.update(&screen)?;
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn draw_text(screen: &mut Screen, text: &str) {
    let scale = 5;
    let text_width = text
        .chars()
        .map(|character| if character == ' ' { 4 } else { 6 })
        .sum::<usize>()
        * scale
        - scale;
    let mut x = (SCREEN_WIDTH - text_width) / 2;
    let y = (SCREEN_HEIGHT - 7 * scale) / 2;

    for character in text.chars() {
        if character == ' ' {
            x += 4 * scale;
            continue;
        }

        for (row, glyph_row) in glyph(character).iter().enumerate() {
            for (column, pixel) in glyph_row.chars().enumerate() {
                if pixel == '1' {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            screen.set_pixel(x + column * scale + dx, y + row * scale + dy, true);
                        }
                    }
                }
            }
        }

        x += 6 * scale;
    }
}

fn glyph(character: char) -> [&'static str; 7] {
    match character {
        'W' => [
            "10001", "10001", "10001", "10101", "10101", "10101", "01010",
        ],
        'e' => [
            "00000", "00000", "01110", "10001", "11111", "10000", "01111",
        ],
        'l' => [
            "01000", "01000", "01000", "01000", "01000", "01000", "01110",
        ],
        'c' => [
            "00000", "00000", "01111", "10000", "10000", "10000", "01111",
        ],
        'o' => [
            "00000", "00000", "01110", "10001", "10001", "10001", "01110",
        ],
        'm' => [
            "00000", "00000", "11010", "10101", "10101", "10101", "10101",
        ],
        't' => [
            "00100", "00100", "11111", "00100", "00100", "00101", "00010",
        ],
        'H' => [
            "10001", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'a' => [
            "00000", "00000", "01110", "00001", "01111", "10001", "01111",
        ],
        'k' => [
            "10000", "10000", "10010", "10100", "11000", "10100", "10010",
        ],
        _ => ["00000"; 7],
    }
}
