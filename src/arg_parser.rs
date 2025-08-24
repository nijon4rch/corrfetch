use clap::Parser;
use std::path::PathBuf;

use crate::{cfg_parser, display};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    logo: Option<String>,

    #[arg(short, long)]
    method: Option<String>,

    #[arg(short = 'W', long)]
    width: Option<u32>,

    #[arg(short = 'H', long)]
    height: Option<u32>,

    #[arg(short, long)]
    config: Option<PathBuf>,
}

pub fn parse() {
    let args = Args::parse();

    let config_file = args.config.or_else(|| {
        let mut path = std::env::home_dir().unwrap_or_else(|| {
            eprintln!("Failed to get home directory!");
            std::process::exit(1);
        });
        path.push(".config");
        path.push("corrfetch");
        path.push("config.toml");
        Some(path)
    });
    let config = cfg_parser::read_config(config_file.unwrap_or_else(|| {
        eprintln!("Failed to unwrap config file path!");
        std::process::exit(1);
    }));

    let logo_cfg = config.logo.as_ref();

    let path = args
        .logo
        .or_else(|| logo_cfg.and_then(|logo| logo.path.clone()));

    let width = args.width.or_else(|| logo_cfg.and_then(|logo| logo.width));

    let height = args
        .height
        .or_else(|| logo_cfg.and_then(|logo| logo.height));

    let method = args
        .method
        .as_deref()
        .or_else(|| logo_cfg.and_then(|logo| logo.method.as_deref()));

    match method {
        Some("ascii") => display::display_ascii(&config, path, height),
        Some("img") => display::display(&config, path, width, height),
        Some("none") => display::display_nologo(&config),
        Some(_) => eprintln!("Invalid method!"),
        None => {
            display::display_nologo(&config);
        }
    }
}
