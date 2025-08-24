use crate::cfg_parser::Config;
use regex::Regex;
use std::collections::HashMap;

use crate::fetch;
use strfmt::strfmt;

fn process_time_strings(
    format_str: &str,
    mut y: i32,
    mut mo: i32,
    mut d: i32,
    mut h: i32,
    mut m: i32,
    mut s: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r"\{(\w+)\}")?;

    let used_vars: std::collections::HashSet<String> = re
        .captures_iter(format_str)
        .map(|cap| cap[1].to_string())
        .collect();

    if !used_vars.contains("y") && used_vars.contains("mo") {
        mo += y * 12;
        y = 0;
    }

    if !used_vars.contains("mo") && used_vars.contains("d") {
        d += mo * 30;
        mo = 0;
    }

    if !used_vars.contains("d") && used_vars.contains("h") {
        h += d * 24;
        d = 0;
    }

    if !used_vars.contains("h") && used_vars.contains("m") {
        m += h * 60;
        h = 0;
    }

    if !used_vars.contains("m") && used_vars.contains("s") {
        s += m * 60;
        m = 0;
    }

    let mut vars = HashMap::new();
    vars.insert("y".to_string(), y.to_string());
    vars.insert("mo".to_string(), mo.to_string());
    vars.insert("d".to_string(), d.to_string());
    vars.insert("h".to_string(), h.to_string());
    vars.insert("m".to_string(), m.to_string());
    vars.insert("s".to_string(), s.to_string());

    Ok(strfmt(format_str, &vars)?)
}

fn format(config: &Config, key: &str) -> String {
    let parsed_cfg = config
        .format
        .get(key)
        .unwrap_or_else(|| panic!("'{}' key not found in config 'format' section!", key))
        .as_str();

    let mut vars = HashMap::new();

    match key {
        "batt" => {
            vars.insert("level".to_string(), fetch::batt().level);
            vars.insert("status".to_string(), fetch::batt().status);
        }
        "ram" => {
            vars.insert("total".to_string(), fetch::ram().total);
            vars.insert("used".to_string(), fetch::ram().used);
            vars.insert("free".to_string(), fetch::ram().free);
            vars.insert("used_percentage".to_string(), fetch::ram().used_percentage);
            vars.insert("free_percentage".to_string(), fetch::ram().free_percentage);
            vars.insert("swap_total".to_string(), fetch::ram().swap_total);
            vars.insert("swap_used".to_string(), fetch::ram().swap_used);
            vars.insert("swap_free".to_string(), fetch::ram().swap_free);
            vars.insert(
                "swap_used_percentage".to_string(),
                fetch::ram().swap_used_percentage,
            );
            vars.insert(
                "swap_free_percentage".to_string(),
                fetch::ram().swap_free_percentage,
            );
        }
        "swap" => {
            vars.insert("total".to_string(), fetch::swap().total);
            vars.insert("used".to_string(), fetch::swap().used);
            vars.insert("free".to_string(), fetch::swap().free);
            vars.insert("free_percentage".to_string(), fetch::swap().free_percentage);
            vars.insert("used_percentage".to_string(), fetch::swap().used_percentage);
        }
        "uptime" => {
            return process_time_strings(
                parsed_cfg,
                0,
                0,
                fetch::uptime().d,
                fetch::uptime().h,
                fetch::uptime().m,
                fetch::uptime().s,
            )
            .unwrap();
        }
        "lifetime" => {
            return process_time_strings(
                parsed_cfg,
                fetch::lifetime().y,
                fetch::lifetime().mo,
                fetch::lifetime().d,
                fetch::lifetime().h,
                fetch::lifetime().m,
                fetch::lifetime().s,
            )
            .unwrap();
        }
        "de" => {
            vars.insert("de".to_string(), fetch::de().de);
        }
        "kernel" => {
            vars.insert("kernel".to_string(), fetch::kernel().kernel);
        }
        "distro" => {
            vars.insert("distro".to_string(), fetch::distro().distro);
            vars.insert("arch".to_string(), fetch::distro().arch);
            vars.insert("version".to_string(), fetch::distro().version);
        }
        "username" => {
            vars.insert("user".to_string(), fetch::username().user);
            vars.insert("host".to_string(), fetch::username().host);
        }
        "hostname" => {
            vars.insert("host".to_string(), fetch::hostname().host);
        }
        "shell" => {
            vars.insert("shell".to_string(), fetch::shell().shell);
        }
        "pkgs" => {
            vars.insert("native".to_string(), fetch::pkgs().native);
            vars.insert("flatpak".to_string(), fetch::pkgs().flatpak);
            vars.insert("snap".to_string(), fetch::pkgs().snap);
            vars.insert("manager".to_string(), fetch::pkgs().manager);
        }
        _ => {}
    }

    strfmt(parsed_cfg, &vars).unwrap()
}

pub fn fetch(config: &Config) -> Vec<String> {
    let mut fetch_text: Vec<String> = Vec::new();
    let mut separator_indices: Vec<usize> = Vec::new();

    for (i, key) in config.keys.split(",").enumerate() {
        match key.trim() {
            "batt" => fetch_text.push(format(config, "batt")),
            "ram" => fetch_text.push(format(config, "ram")),
            "swap" => fetch_text.push(format(config, "swap")),
            "uptime" => fetch_text.push(format(config, "uptime")),
            "lifetime" => fetch_text.push(format(config, "lifetime")),
            "de" => fetch_text.push(format(config, "de")),
            "kernel" => fetch_text.push(format(config, "kernel")),
            "distro" => fetch_text.push(format(config, "distro")),
            "username" => fetch_text.push(format(config, "username")),
            "hostname" => fetch_text.push(format(config, "hostname")),
            "shell" => fetch_text.push(format(config, "shell")),
            "pkgs" => fetch_text.push(format(config, "pkgs")),
            "separator" => {
                fetch_text.push(String::new());
                separator_indices.push(i)
            }
            _ => fetch_text.push(key.to_string()),
        }
    }

    let max_length = fetch_text
        .iter()
        .enumerate()
        .filter(|(i, _)| !separator_indices.contains(i))
        .map(|(_, s)| s.len())
        .max()
        .unwrap_or(0);

    let separator = config
        .separator
        .unwrap_or('-')
        .to_string()
        .repeat(max_length);

    for &index in &separator_indices {
        fetch_text[index] = separator.clone();
    }

    fetch_text
}
