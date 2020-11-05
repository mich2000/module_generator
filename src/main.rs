mod file_man;
mod tree_config;

use file_man::{reader};
use std::env;
use tree_config::ModuleTreeConfig;

fn main() {
    let vector_module_tree = args_to_vec_module_tree(env::args().collect());
    print!("{}",exec_vec_module_tree(vector_module_tree));
}

/**
 * Parses all the arguments and returns a ModuleTreeConfig struct vector. It will make a 1st iteration over all the arguments except the 1st one, and will control in the first place if it is equal to --write-in-main. --write-in-main is a flag indicatin that the usage of the modules/submodules should be written in the main rust file. In the 2nd iteration all the strings that are not equalt to --write-in-main will be added and parsed to return the vector.
 */
fn args_to_vec_module_tree(args : Vec<String>) -> Vec<ModuleTreeConfig> {
    let mut vec_module_tree_config : Vec<ModuleTreeConfig> = Vec::new();
    let write_in_main = args.contains(&"-W".to_owned());
    for arg in args.iter().skip(1) {
        if arg != "-W" {
            println!("{}",&arg);
            vec_module_tree_config.push(ModuleTreeConfig::new(&arg, write_in_main));
        }
    }
    vec_module_tree_config
}

/**
 * Returns the output in a string, from when the execution of all the ModuleTreeConfig in the vector.
 */
fn exec_vec_module_tree(vec_module_tree_config : Vec<ModuleTreeConfig>) -> String {
    let mut output = String::new();
    for tree_config in vec_module_tree_config {
        output.push_str(&format!("{}\n", tree_config.execute()));
    }
    output
}