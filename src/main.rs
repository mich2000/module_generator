mod file_man;
mod tree_config;

use file_man::reader;
use std::env;
use tree_config::ModuleTreeConfig;

fn main() {
    print!(
        "{}",
        exec_vec_module_tree(args_to_vec_module_tree(env::args().collect()))
    );
}

/**
 * Parses all the arguments and returns a ModuleTreeConfig struct vector. It will first try to see if the modules should be included in the main or lib file. It is done by seeing if the last argument equals -W flag. The rest of the argument is pushed towards the ModuleTreeConfig vector.
 */
fn args_to_vec_module_tree(args: Vec<String>) -> Vec<ModuleTreeConfig> {
    let mut vec_module_tree_config: Vec<ModuleTreeConfig> = Vec::new();
    let write_in_main = *args.last().unwrap_or(&String::new()) == *"-W";
    for arg in args.iter().skip(1) {
        if arg != "-W" {
            vec_module_tree_config.push(ModuleTreeConfig::new(&arg, write_in_main));
        }
    }
    vec_module_tree_config
}

/**
 * Returns the output in a string, from when the execution of all the ModuleTreeConfig in the vector.
 */
fn exec_vec_module_tree(vec_module_tree_config: Vec<ModuleTreeConfig>) -> String {
    let mut output = String::new();
    for tree_config in vec_module_tree_config {
        output.push_str(&format!("{}\n", tree_config.execute()));
    }
    output
}