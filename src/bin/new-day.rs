use std::{
    env::args,
    fs::{read_to_string, write, File},
    path::PathBuf,
};

fn main() {
    let n = args()
        .nth(1)
        .expect("Missing day number")
        .parse::<usize>()
        .expect("Argument must be a number");

    let bin_file = PathBuf::from(format!("src/bin/day{}.rs", n));
    if bin_file.exists() {
        panic!("Binary file already exists");
    }
    let template = read_to_string("template.rs").unwrap();
    let template = template.replace("{number}", &n.to_string());
    write(bin_file, template).expect("Failed to copy template");

    let input_file = PathBuf::from(format!("inputs/day{}.txt", n));
    if input_file.exists() {
        panic!("Input file already exists");
    }
    File::create(input_file).expect("Failed to create input file");
}
