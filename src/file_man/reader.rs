use std::fs;
use std::fs::File;
use std::fs::ReadDir;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

/**
 * Return an iterator which goes through every file or folder on the current directory where the user is.
 **/
pub fn get_files_of_dir() -> ReadDir {
    fs::read_dir(std::env::current_dir().unwrap()).expect("Unable to list")
}

/**
 * Return a boolean on which the value is decided if the current directory is a cargo project.
 * It decides if a directory is a cargo directory if contains the following elements:
 * - src folder
 * - toml file
 **/
pub fn current_path_rust_dir() -> bool {
    let mut has_src_dir = false;
    let mut has_toml_file = false;
    for entry in get_files_of_dir() {
        let entry_path: PathBuf = entry.expect("Could not process this entry").path();
        if entry_path.is_dir() && entry_path.file_name().unwrap().to_str().unwrap() == "src" {
            has_src_dir = true;
        } else if !has_toml_file {
            has_toml_file = entry_path.ends_with("Cargo.toml");
        }
    }
    if has_src_dir && has_toml_file {
        return true;
    }
    false
}

/**
 * Returns a result iterator which goes over all the lines in a particular file.
 */
pub fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
