use std::fs;
use std::path::{Path,PathBuf};
use std::fs::ReadDir;
use std::fs::File;
use std::io::{self, BufRead};


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
 * - lock file
 **/
pub fn current_path_rust_dir() -> bool {
    let mut has_src_dir = false;
    let mut has_toml_file = false;
    let mut has_lock_file = false;
    for entry in get_files_of_dir() {
        let entry_path : PathBuf = entry.expect("Could not process this entry").path();
        if entry_path.is_dir() && entry_path.file_name().unwrap().to_str().unwrap() == "src" {
            has_src_dir = true;
        } else {
            let extension = entry_path.extension();
            if !extension.is_none(){
                if extension.unwrap() == "lock" {
                    has_lock_file = true;
                }
                if extension.unwrap() == "toml" {
                    has_toml_file = true;
                }
            }
        }
        if has_src_dir && has_toml_file && has_lock_file {
            return true;
        }
    }
    return false;
}   

/**
 * A path is given as parameter, this one is used to iterate through it and compare it with each element of a string vector.
 * It will compare these element in the string vector and if the line isn't equal to it.
 **/
pub fn control_file_lines(path : String, module_use_lines : Vec<String>) -> String {
    let mut non_present_in_file : String = String::new();
    for module in module_use_lines {
        for line in read_lines(&path).unwrap() {
            if let Ok(line_ok) = line {
                if line_ok.as_bytes().len() != 0 {
                    let sentence = String::from_utf8_lossy(line_ok.as_bytes());
                    if module != sentence {
                        non_present_in_file.push_str(&format!("{}\n", &module));
                        break;
                    }
                }
            }
        }
    }
    return non_present_in_file;
}

/**
 * Returns a result iterator which goes over all the lines in a particular file.
 */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}