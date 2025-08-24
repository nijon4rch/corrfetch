use crate::{cfg_parser::Config, format};
use crossterm::{
    cursor::{MoveRight, MoveToNextLine, MoveToPreviousLine},
    execute,
};
use rascii_art::{RenderOptions, render_to};
use std::fs::File;
use std::io::{self, BufRead, stdout};
use std::path::Path;
use unicode_width::UnicodeWidthStr;

pub fn display(config: &Config, logo: Option<String>, width: Option<u32>, height: Option<u32>) {
    let fetch_text = format::fetch(config);

    let (conf_height, conf_width) = match (width, height) {
        (Some(w), h) => (Some(h.unwrap_or(w)), Some(w * 2)),
        (None, Some(h)) => (Some(h), Some(h * 2)),
        (None, None) => {
            let base = if fetch_text.len() <= 10 {
                12
            } else {
                fetch_text.len() as u32 + 4
            };
            (Some(base), Some(base * 2))
        }
    };

    let conf = viuer::Config {
        width: conf_width,
        height: conf_height,
        absolute_offset: false,
        restore_cursor: false,
        ..Default::default()
    };
    viuer::print_from_file(
        match logo {
            Some(logo) => logo,
            None => {
                eprintln!("Please provide a path to image file!");
                return;
            }
        },
        &conf,
    )
    .expect("Image printing failed.");

    execute!(stdout(), MoveToPreviousLine(conf_height.unwrap() as u16)).unwrap();

    fetch_text.iter().for_each(|s| {
        execute!(stdout(), MoveRight(conf_width.unwrap() as u16 + 2)).unwrap();
        println!("{s}")
    });

    let move_by = {
        if fetch_text.len() < conf_height.unwrap() as usize {
            conf_height.unwrap() as u16 - fetch_text.len() as u16
        } else {
            fetch_text.len() as u16
        }
    };

    execute!(stdout(), MoveToNextLine(move_by)).unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn display_ascii(config: &Config, logo: Option<String>, height: Option<u32>) {
    let fetch_text = format::fetch(config);
    let mut ascii = String::new();

    let conf_height = if let Some(h) = height {
        h
    } else if fetch_text.len() <= 10 {
        12
    } else {
        fetch_text.len() as u32 + 4
    };

    if let Some(logo) = logo {
        if !logo.ends_with(".txt") {
            let cfg = config;
            let charset: Vec<&str> = cfg
                .logo
                .as_ref()
                .and_then(|cfg_logo| cfg_logo.charset.as_ref())
                .map(|c| c.iter().map(|s| s.as_str()).collect())
                .unwrap_or_else(|| vec![".", ",", "-", "*", "£", "$", "#"]);

            render_to(
                logo,
                &mut ascii,
                &RenderOptions::new()
                    .height(conf_height as u32)
                    .colored(true)
                    .charset(&charset),
            )
            .unwrap();
        } else if let Ok(lines) = read_lines(logo) {
            for line in lines.map_while(Result::ok) {
                ascii.push_str(&line);
                ascii.push('\n');
            }
        }
    } else {
        eprintln!("Please provide a path to image or .txt file!");
        return;
    }

    let width = ascii
        .lines()
        .map(|s| console::strip_ansi_codes(s).width())
        .max()
        .unwrap() as u16;
    let height = ascii.lines().count() as u16;

    ascii.lines().for_each(|s| println!("{s}"));

    execute!(stdout(), MoveToPreviousLine(height)).unwrap();

    fetch_text.iter().for_each(|s| {
        execute!(stdout(), MoveRight(width + 2)).unwrap();
        println!("{s}")
    });

    let move_by = {
        if fetch_text.len() < height as usize {
            height - fetch_text.len() as u16
        } else {
            fetch_text.len() as u16
        }
    };
    execute!(stdout(), MoveToNextLine(move_by)).unwrap();
}

pub fn display_nologo(config: &Config) {
    let fetch_text = format::fetch(config);

    fetch_text.iter().for_each(|s| println!("{s}"));
}
