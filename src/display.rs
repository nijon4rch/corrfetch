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

fn is_unsupported_console() -> bool {
    if std::env::var("ZELLIJ").is_ok() {
        return true;
    }

    let term = std::env::var("TERM").unwrap_or_default();
    if term == "linux" || term == "dumb" {
        return true;
    }

    if let Ok(path) = std::fs::read_link("/proc/self/fd/1") {
        let path_str = path.to_string_lossy();

        if path_str.starts_with("/dev/tty") {
            return true;
        }
    }

    false
}
pub fn display(config: &Config, logo: Option<String>, width: Option<u32>, height: Option<u32>) {
    if is_unsupported_console() {
        return display_ascii(config, logo, height);
    }

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

    let raw_path = match logo {
        Some(path) => path,
        None => {
            eprintln!("Please provide a path to image file!");
            return;
        }
    };

    let path = shellexpand::full(&raw_path)
        .unwrap_or_else(|err| {
            eprintln!("Failed to expand path '{}': {}", raw_path, err);
            std::process::exit(1);
        })
        .into_owned();

    viuer::print_from_file(&path, &conf).expect("Image printing failed.");

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

    if let Some(raw_logo) = logo {
        let expanded_logo = shellexpand::full(&raw_logo)
            .unwrap_or_else(|err| {
                eprintln!("Failed to expand path '{}': {}", raw_logo, err);
                std::process::exit(1);
            })
            .into_owned();

        if !expanded_logo.ends_with(".txt") {
            let cfg = config;
            let charset: Vec<&str> = cfg
                .logo
                .as_ref()
                .and_then(|cfg_logo| cfg_logo.charset.as_ref())
                .map(|c| c.iter().map(|s| s.as_str()).collect())
                .unwrap_or_else(|| vec![".", ",", "-", "*", "£", "$", "#"]);

            if let Err(e) = render_to(
                &expanded_logo,
                &mut ascii,
                &RenderOptions::new()
                    .height(conf_height as u32)
                    .colored(true)
                    .charset(&charset),
            ) {
                eprintln!("Failed to render ASCII art from image: {}", e);
                std::process::exit(1);
            }
        } else if let Ok(lines) = read_lines(&expanded_logo) {
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
        .unwrap_or(0) as u16;

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
