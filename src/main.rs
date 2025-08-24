pub mod arg_parser;
pub mod cfg_parser;
pub mod display;
pub mod fetch;
pub mod format;

fn main() {
    arg_parser::parse();
}
