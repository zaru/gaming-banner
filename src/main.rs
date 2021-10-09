use colored::*;
use font8x8::{BASIC_FONTS, UnicodeFonts};
use std::{thread, time};

struct Cli {
    message: String,
    deco: String,
}



fn main() {
    let c1: Vec<u8> = vec![57, 0, 93];
    let c2: Vec<u8> = vec![19, 155, 221];
    let c3: Vec<u8> = vec![181, 211, 12];
    let c4: Vec<u8> = vec![254, 229, 9];
    let c5: Vec<u8> = vec![245, 111, 17];
    let c6: Vec<u8> = vec![200, 0, 5];
    let c7: Vec<u8> = vec![234, 0, 97];
    let colors: Vec<Vec<u8>> = vec![
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
    ];
    let mut colors_iter = colors.iter().cycle();

    let message = std::env::args().nth(1).expect("no message");
    let deco = std::env::args().nth(2).expect("no deco");
    let args = Cli {
        message,
        deco,
    };
    let cs = args.message.to_string();
    let mut chars: Vec<[u8; 8]> = Vec::new();
    for c in cs.as_str().chars() {
        if let Some(glyph) = BASIC_FONTS.get(c) {
            chars.push(glyph);
        }
    }

    let sleep = time::Duration::from_millis(100);

    for _ in 0..100 {
        for y in 0..9 {
            if y > 0 && y < 9 {
                let row = y - 1;
                for char in chars.iter() {
                    if let Some(color) = colors_iter.next() {
                        print!("{}", args.deco.truecolor(color[0], color[1], color[2]));
                    }
                    for x in 0..8 {
                        match char[row] & 1 << x {
                            0 => if let Some(color) = colors_iter.next() {
                                print!("{}", args.deco.truecolor(color[0], color[1], color[2]));
                            },
                            _ => print!("{}", "█".white())
                        }
                    }
                    if let Some(color) = colors_iter.next() {
                        print!("{}", args.deco.truecolor(color[0], color[1], color[2]));
                    }
                }
            } else {
                for _ in 0..(10 * chars.len()) {
                    if let Some(color) = colors_iter.next() {
                        print!("{}", args.deco.truecolor(color[0], color[1], color[2]));
                    }
                }
            }
            println!();
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        thread::sleep(sleep);
    }
}
