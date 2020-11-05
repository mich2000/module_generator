use std::fs;
use std::io;
use std::env;

use std::path::Path;

use crate::reader;

/**
 * Makes a module directory in the source folder of the rust project. Returns a result to determine if the directory has been made.
 **/
fn make_dir(folder : &str) -> io::Result<()> {
    let mut path = env::current_dir().unwrap();
    path.push(&format!("./src/{}",folder));
    println!("{:?}", path.display());
    fs::create_dir(path)
}


/**
 * Creates an rust file in the source folder, under the module directory.
 * Arguments:
 * - sup_module_name: module where the rust file will be made.
 * - module_name: The name the rust file will take.
 **/
fn create_rust_module(sup_module_name : &str,module_name : &str) -> io::Result<()> {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.push(&sup_module_name);
    path.push(&module_name);
    path.set_extension("rs");
    fs::write(path, "")
}

/**
 * Creates an rust file under the source folder, under the module folder and this file will serve the purpopse of module reference file.
 * Arguments:
 * - folder_name: is the module name where the rust module will be made
 * - list_modules: these are the submodules that wil be referenced in the rust module file
 **/
fn create_rust_module_holder(folder_name : &str , list_modules : Vec<String>) -> io::Result<()> {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.push(&folder_name);
    path.push("mod");
    path.set_extension("rs");
    let mut content_file = String::new();
    for module in list_modules {
        content_file.push_str(&format!("pub mod {};\n",module));
    }
    fs::write(path, content_file)
}

/**
 * Function responsible to making the folder where the module files will come. It will also create the module file in the module folder that references the submodules.
 * 
 * It will also add all the references to the module file and the uses for the submodules. If the user wants to write in the main file all the uses.
 * Arguments:
 * - module_name: This is the module name that will be created as a folder or file depending on the lenght of the list_modules vector. 
 * - list_modules: These are the submodules that will be added under the module
 * - _write_in_main: If true, the module declaration and usages declarations will be set in the main rust file.
 **/
pub fn create_mod_tree(module_name : &str, list_modules : Vec<&str>, _write_in_main : bool) -> String {
    let mut output = String::new();
    let mut line_to_be_controller : Vec<String> = vec![];
    let owned_list_modules : Vec<String> = list_modules.iter().map(|a| a.to_string()).collect::<Vec<String>>();
    output.push_str(&format!("Module {} generetion\n", module_name.to_owned()));
    if reader::current_path_rust_dir() {
        output.push_str("Current directory is a rust directory.\n");
    } else {
        panic!("This is not a rust cargo project directory.");
    }
    if !owned_list_modules.is_empty() {
        if !Path::new(&format!("./src/{}",&module_name)).exists() {
            match make_dir(module_name){
                Ok(_e) => output.push_str(&format!("Succeeded to make directory {}\n", &module_name)),
                Err(_e) => println!("{:?}",_e)
            };
        }
        output.push_str("Successfully made a directory for the modules.\n");
        line_to_be_controller.push(format!("mod {};", &module_name));
        for module in owned_list_modules.to_owned() {
            if !Path::new(&format!("./src/{}/{}.rs",&module_name, &module)).exists() {
                match create_rust_module(module_name, &module) {
                    Ok(_e) => {
                        output.push_str(&format!("Module {} has been added in the supmodule {}.\n",&module, module_name));
                    },
                    Err(_e) => println!("{:?}",_e)
                }
            }
        }
        match create_rust_module_holder(&module_name, owned_list_modules) {
            Ok(_e) => output.push_str(&format!("Module lister {} has successfully been made.\n",module_name)),
            Err(_e) => println!("{:?}",_e)
        }
    } else if !Path::new(&format!("./src/{}",&module_name)).exists() {
        match create_rust_module(module_name, "") {
            Ok(_e) => {
                output.push_str(&format!("Module {} has been added.\n", module_name));
                line_to_be_controller.push(format!("mod {};",&module_name));
            },
            Err(_e) => println!("{:?}",_e)
        }
    }
    if _write_in_main {
        output.push_str(&format!("{}\n",write_in_main(line_to_be_controller)))
    }
    output
}

/**
 * Function used to write all the usages of modules and submodules into the main file, it will only write the lines that are not present in the main rust file.
 * Arguments:
 * - line_to_be_controller: These lines will be controlled if they are present in the main rust file and be written in it if these are not in there.
 **/
fn write_in_main(line_to_be_controller : Vec<String>) -> &'static str {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    if path.join("main.rs").exists() {
        path.push("main.rs");
    } else if path.join("lib.rs").exists() {
        path.push("lib.rs");
    }
    let mut usages_for_main : String = reader::control_file_lines(path.to_str().unwrap().to_owned(),line_to_be_controller);
    let content_file = fs::read_to_string(&path).expect("Could not read the path");
    usages_for_main.push_str(&format!("\n{}",content_file));
    match fs::write(path, usages_for_main){
        Ok(_e) => "Managed to add the usages in the main.rs file.",
        Err(_e) => "Did not manage to add the usages in the main.rs file."
    }
}