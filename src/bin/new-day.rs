use std::{env::args, fs::{File, copy}, path::PathBuf};

fn main() {
    let n = args().nth(1).expect("Missing day number");
    let bin_file = PathBuf::from(format!("src/bin/day{}.rs", n));
    if bin_file.exists() {
        panic!("Binary file already exists");
    }
    copy("template.rs", bin_file).expect("Failed to copy template");
    let input_file = PathBuf::from(format!("inputs/day{}.txt", n));
    if input_file.exists() {
        panic!("Input file already exists");
    }
    File::create(input_file).expect("Failed to create input file");
}
