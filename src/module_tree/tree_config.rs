use crate::file_man::maker;

pub struct ModuleTreeConfig {
    module_name : String,
    sub_modules : Vec<String>,
    write_in_main : bool
}

impl ModuleTreeConfig {
    /**
     * Return a ModuleTreeConfig struct based on the given arguments.
     * Arguments:
     * -module_string: This is the raw string that will be parsed to give the module and the submodules of it. The string will have to have a particular format, this format is (module:{submodules?..}). The submodules in the string are optional.
     * - write_main: If true, this means that the usages of the module and submodules will be written in the main rust file.
     **/
    pub fn new(module_string : &str, write_main : bool) -> ModuleTreeConfig {
        let modules_and_lists :Vec<&str> = module_string.split(":").collect::<Vec<&str>>();
        if modules_and_lists.len() == 2 {
            let sup_module_name : String = modules_and_lists[0].to_string();
            let sub_modules_names : Vec<String> = modules_and_lists[1].to_string().split(",")
                .map(|md| md.trim().to_owned()).collect::<Vec<String>>();
            return ModuleTreeConfig {
                module_name : sup_module_name,
                sub_modules : sub_modules_names,
                write_in_main : write_main
            };
        } else {
            return ModuleTreeConfig {
                module_name : module_string.to_owned(),
                sub_modules : vec![],
                write_in_main : write_main
            };
        }
    }

    /**
     * Returns the output obtained through executing the function who will create all the module file/folder and all of its possible submodules. Will also return the output of the act of writting in the main rust file.
     **/
    pub fn execute(&self) -> String {
        maker::create_mod_tree(
            &self.module_name, 
            self.sub_modules.iter().map(|module| &**module).collect(),
            self.write_in_main)
    }
}