use std::{env, fs, path::Path, process::Command};

pub struct Batt {
    pub level: String,
    pub status: String,
}

pub struct Ram {
    pub total: String,
    pub used: String,
    pub free: String,
    pub used_percentage: String,
    pub free_percentage: String,
    pub swap_total: String,
    pub swap_used: String,
    pub swap_free: String,
    pub swap_used_percentage: String,
    pub swap_free_percentage: String,
}

pub struct Swap {
    pub total: String,
    pub used: String,
    pub free: String,
    pub free_percentage: String,
    pub used_percentage: String,
}

pub struct Uptime {
    pub d: i32,
    pub h: i32,
    pub m: i32,
    pub s: i32,
}

pub struct Lifetime {
    pub y: i32,
    pub mo: i32,
    pub d: i32,
    pub h: i32,
    pub m: i32,
    pub s: i32,
}

pub struct De {
    pub de: String,
}

pub struct Kernel {
    pub kernel: String,
}

pub struct Distro {
    pub distro: String,
    pub arch: String,
    pub version: String,
}

pub struct Username {
    pub user: String,
    pub host: String,
}

pub struct Hostname {
    pub host: String,
}

pub struct Shell {
    pub shell: String,
}

pub struct Pkgs {
    pub native: String,
    pub flatpak: String,
    pub snap: String,
    pub manager: String,
}

pub struct Separator {
    pub separator: String,
}

fn read_os_release(key: &str, key_alt: Option<&str>) -> String {
    std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|release| {
            release.lines().find_map(|line| {
                line.strip_prefix(key)
                    .or_else(|| key_alt.and_then(|alt| line.strip_prefix(alt)))
                    .map(|val| val.trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| "Failed to read /etc/os-release!".to_string())
}

pub fn batt() -> Batt {
    let level = match fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
        Ok(l) => l.trim().to_string(),
        Err(_) => String::from("Failed to read battery level!"),
    };
    let status = match fs::read_to_string("/sys/class/power_supply/BAT0/status") {
        Ok(s) => s.trim().to_string(),
        Err(_) => String::from("Failed to read battery status!"),
    };

    Batt { level, status }
}

pub fn ram() -> Ram {
    use std::collections::HashMap;

    let memfile = fs::read_to_string("/proc/meminfo").unwrap();

    let mut values = HashMap::new();
    for line in memfile.lines() {
        if let Some((key, val)) = line.split_once(':') {
            let num = val
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            values.insert(key.to_string(), num);
        }
    }

    let total = values["MemTotal"] / 1024;
    let free = values["MemAvailable"] / 1024;
    let used = total - free;
    let used_percentage = (used as f32 / total as f32 * 100.0).floor();
    let free_percentage = (free as f32 / total as f32 * 100.0).floor();

    let total = total.to_string();
    let used = used.to_string();
    let free = free.to_string();
    let used_percentage = used_percentage.to_string();
    let free_percentage = free_percentage.to_string();

    let swap_total = values["SwapTotal"] / 1024;
    let swap_free = values["SwapFree"] / 1024;
    let swap_used = swap_total - swap_free;
    let swap_used_percentage = if swap_total > 0 {
        (swap_used as f32 / swap_total as f32 * 100.0).floor()
    } else {
        0.0
    };
    let swap_free_percentage = if swap_total > 0 {
        (swap_free as f32 / swap_total as f32 * 100.0).floor()
    } else {
        0.0
    };

    let swap_total = swap_total.to_string();
    let swap_used = swap_used.to_string();
    let swap_free = swap_free.to_string();
    let swap_used_percentage = swap_used_percentage.to_string();
    let swap_free_percentage = swap_free_percentage.to_string();

    Ram {
        total,
        used,
        free,
        used_percentage,
        free_percentage,
        swap_total,
        swap_used,
        swap_free,
        swap_used_percentage,
        swap_free_percentage,
    }
}

pub fn swap() -> Swap {
    let total = String::from("8");
    let used = String::from("4");
    let free = String::from("4");
    let free_percentage = String::from("50");
    let used_percentage = String::from("50");

    Swap {
        total,
        used,
        free,
        free_percentage,
        used_percentage,
    }
}

pub fn uptime() -> Uptime {
    let contents = fs::read_to_string("/proc/uptime").unwrap();

    let uptime: i32 = contents.split('.').next().unwrap().parse().unwrap();

    let d = uptime / 86400;
    let h = (uptime % 86400) / 3600;
    let m = (uptime % 3600) / 60;
    let s = uptime % 60;

    Uptime { d, h, m, s }
}

pub fn lifetime() -> Lifetime {
    use std::time::{Duration, SystemTime};

    let now = SystemTime::now();
    let mut oldest_time = now;

    let path = Path::new("/");
    if path.exists()
        && let Ok(metadata) = fs::metadata(path)
        && let Ok(created) = metadata.created()
    {
        oldest_time = created;
    }

    let duration = now
        .duration_since(oldest_time)
        .unwrap_or(Duration::from_secs(0));

    let y = duration.as_secs() as i32 / 31536000;
    let mo = (duration.as_secs() as i32 % 31536000) / 2628000;
    let d = (duration.as_secs() as i32 % 2628000) / 86400;
    let h = (duration.as_secs() as i32 % 86400) / 3600;
    let m = (duration.as_secs() as i32 % 3600) / 60;
    let s = duration.as_secs() as i32 % 60;

    Lifetime { y, mo, d, h, m, s }
}

pub fn de() -> De {
    let de = env::var("XDG_CURRENT_DESKTOP").unwrap_or(String::from(
        "Failed to get $XDG_CURRENT_DESKTOP env variable!",
    ));

    De { de }
}

pub fn kernel() -> Kernel {
    let command = Command::new("uname").arg("-r").output().unwrap();
    let kernel = String::from_utf8(command.stdout)
        .unwrap()
        .trim()
        .to_string();

    Kernel { kernel }
}

pub fn distro() -> Distro {
    let distro = read_os_release("PRETTY_NAME=", Some("NAME="));

    let version = read_os_release("BUILD_ID=", None);

    let command = Command::new("uname").arg("-m").output().unwrap();
    let arch = String::from_utf8(command.stdout)
        .unwrap()
        .trim()
        .to_string();

    Distro {
        distro,
        arch,
        version,
    }
}

pub fn username() -> Username {
    let user = env::var("USER").unwrap_or(String::from("Failed to get $USER env variable!"));
    let host = hostname::get().map_or(String::from("Failed to get hostname!"), |name| {
        name.to_string_lossy().to_string()
    });

    Username { user, host }
}

pub fn hostname() -> Hostname {
    let host = hostname::get().map_or(String::from("Failed to get hostname!"), |name| {
        name.to_string_lossy().to_string()
    });

    Hostname { host }
}

pub fn shell() -> Shell {
    let shell = env::var("SHELL")
        .map_or(String::from("Couldn't get SHELL env variable!"), |name| {
            name.split('/').next_back().unwrap().to_string()
        });

    Shell { shell }
}

pub fn pkgs() -> Pkgs {
    let distro_id = read_os_release("ID_LIKE=", Some("ID="));

    let manager = distro_id
    .as_str()
    .split(' ')
    .find_map(|distro| match distro {
        "arch" => Some(String::from("pacman")),
        "debian" => Some(String::from("dpkg")),
        "void" => Some(String::from("xbps")),
        "rhel" => Some(String::from("rpm")),
        "gentoo" => Some(String::from("portage")),
        _ => None,
    })
    .unwrap_or_else(|| {
        format!(
            "Failed to identify package manager: {distro_id}, either failed to read /etc/os-release or distro is not supported!"
        )
    });

    let pm_command = match manager.as_str() {
        "pacman" => Command::new("pacman")
            .arg("-Q")
            .output()
            .expect("Failed to run package manager command!"),
        "dpkg" => Command::new("dpkg")
            .arg("-l")
            .output()
            .expect("Failed to run package manager command!"),
        "xbps" => Command::new("xbps-query")
            .arg("-l")
            .output()
            .expect("Failed to run package manager command!"),
        "rpm" => Command::new("rpm")
            .arg("-qa")
            .output()
            .expect("Failed to run package manager command!"),
        "portage" => {
            use std::os::unix::process::ExitStatusExt;

            let mut count = 0;
            if let Ok(categories) = fs::read_dir("/var/db/pkg") {
                for cat in categories.flatten() {
                    if let Ok(pkgs) = fs::read_dir(cat.path()) {
                        count += pkgs.flatten().count();
                    }
                }
            }

            std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: count.to_string().into_bytes(),
                stderr: Vec::new(),
            }
        }
        _ => {
            eprintln!("Error in pkgs key:");
            eprintln!("Package manager/distro is not supported!");
            eprintln!("exiting...");
            std::process::exit(1);
        }
    };

    let native = String::from_utf8_lossy(&pm_command.stdout)
        .lines()
        .count()
        .saturating_sub(1)
        .to_string();

    let flatpak = {
        Command::new("flatpak")
            .arg("list")
            .arg("--app")
            .output()
            .map_or(
                "Failed to determine flatpak package count! (is flatpak available?)".to_string(),
                |out| {
                    String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .count()
                        .to_string()
                },
            )
    };

    let snap = {
        Command::new("snap").arg("list").output().map_or(
            "Failed to determine snap package count! (is snap available?)".to_string(),
            |out| {
                String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .count()
                    .to_string()
            },
        )
    };

    Pkgs {
        native,
        flatpak,
        snap,
        manager,
    }
}
