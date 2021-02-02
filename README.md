# module_generator

Cli-tool written in Rust. It is used to create rust modules and to eventually populate these with submodules in a folder. There is also the option to write the dependency in the main rust file. To use it you will have to be in a cargo directory before using the program.

## Warning

This tool should only be used to create modules and generate these uses in you're main or lib file, if you want to modify them later by removing or editing the names do it manually. Using this tool to add submodules to a existing modules is okay but not to remove or edit these, it will give bugs and it's not my intent.

## Example

This is a complete example where I make a module named `models` and it has the submodules `page` and `person` in it. I also set the flag to write the usages in the main rust file, the flag is **optional**.

```cd **cargo_directory**
cargo run "models:page,person" -W```