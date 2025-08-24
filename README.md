# `corrfetch` - the corroded fetch :)

Yet another system fetch for linux, written in rust.

- Supports image rendering in supported terminals (via [viuer](https://crates.io/crates/viuer)), and can automatically convert images to ascii.
- Also supports text coloring (see [this example](https://github.com/nijon4rch/corrfetch/blob/main/examples/nitch.toml)).
- Configurable

Check out the [examples](https://github.com/nijon4rch/corrfetch/tree/main/examples)!



# Installation:

Download the latest [release](https://github.com/nijon4rch/corrfetch/releases/latest), or

#### Build from source:

```bash
git clone https://github.com/nijon/corrfetch-git
cd corrfetch-git
cargo build --release
```

or build and install with:

```bash
git clone https://github.com/nijon/corrfetch-git
cd corrfetch-git
cargo install --path .
```



# Usage:

```bash
corrfetch [OPTIONS]
```

#### Options:
-  `-l`, `--logo` `[path]`
    - path to logo file (image for img, image or .txt for ascii)
-  `-m`, `--method` `[none|img|ascii]`
    - method to print logo
-  `-W`, `--width` `[width]`
    - logo width (integer)
-  `-H`, `--height` `[height]`
    - logo height (integer)
-  `-c`, `--config` `[path]`
    - path to config file
-  `-h`, `--help`
    - Print help
-  `-V`, `--version`
    - Print version

>[!NOTE]
> ascii mode only support specifying height, and only when converting from image. Width is ignored, as well as height if using .txt as logo.




# Configuration:

`corrfetch` is configured through a .toml file, located at `~/.config/corrfetch/config.toml` or provided by the `-c`/`--config` flag.

Example configurations can be found in the `examples` directory.

Check out the [full config](https://github.com/nijon4rch/corrfetch/blob/main/examples/full.toml) to see all possible options and keys.



# Roadmap:

- [ ] Fetch more things: cpu, gpu, network, etc.
