#[allow(unused_imports)]
#[allow(unused_variables)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::path::{Path};
use std::process::Command as StdCommand;


fn main() {
    // Wait for user input
    const BUILT_IN: &[&str] = &["echo", "exit", "type", "pwd", "cd"];
    let stdin = io::stdin();
    shell_init();
    loop {
        let mut shell_input: String = String::new();
        let _  = stdin.read_line(&mut shell_input).unwrap();
        let mut full_command = Command::build(&shell_input);

        if full_command.command.is_empty(){
            shell_init();
            continue
        }
        
        match full_command.command.as_str() {
            "exit" => break,
            "echo" => {
                println!("{}", full_command.arguments);
                // shell_init();
            },
            "type" => {
                match full_command.find_type(BUILT_IN).as_str() {
                    "builtin" => println!("{} is a shell builtin",full_command.arguments),
                    "_" => println!("{}: not found", full_command.arguments),
                    path_cmd => println!("{} is {}", full_command.arguments, path_cmd)
                }
                // shell_init();
            },
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            },
            "cd" => {
                let path = if full_command.arguments == "~" || full_command.arguments == "" {
                    env::var("HOME").unwrap()
                }else{
                    full_command.arguments
                };

                match env::set_current_dir(Path::new(&path)){
                    Ok(_) => (),
                    Err(_) => println!("cd: {}: No such file or directory", path)
                }
            },
            other => {
                if let Ok(path_var) = env::var("PATH") {
                    let mut status = false;
                    for path in path_var.split(':') {
                        match find_file_in_path(Path::new(path), full_command.command.as_str()) {
                            Some(path) => {
                                let args: Vec<&str> = full_command.arguments
                                    .split_whitespace() // This handles multiple whitespace and trims them out
                                    .filter(|&arg| !arg.is_empty()) // Filter out empty arguments
                                    .collect();
                                StdCommand::new(path).args(args).status().expect("failed to execute process");
                                status = true
                            },
                            None => continue
                        }
                    }
                    if !status {
                        println!("{}: command not found", other);
                    }
                }
                // shell_init();
            }
        }
        shell_init();
    }
}

fn shell_init(){
    print!("$ ");
    io::stdout().flush().unwrap();
}

#[derive(Debug)]
struct Command{
    command: String,
    arguments: String,
}

impl Command {
    fn build(shell_input : &str) -> Command {
        let Some((command, arguments)) = shell_input.trim().split_once(" ") else { 
            return Command{command: shell_input.trim().to_string(), arguments: String::new() };
         };
        Command{command: command.to_string(), arguments: arguments.to_string()}
    }

    fn find_type(&mut self, built_in: &[&str]) -> String {
        
        if built_in.contains(&self.arguments.as_str()){
            return "builtin".to_string();
        }

        if let Ok(path_var) = env::var("PATH") {
            for path in path_var.split(':') {
                match find_file_in_path(Path::new(path), &self.arguments) {
                    Some(path) => {
                        return path;
                    },
                    None => continue
                }
            }
        }
        "_".to_string()
    }
}

fn find_file_in_path(dir: &Path, file_to_find: &str) -> Option<String> {
    if dir.is_dir(){
        if let Ok(files) = fs::read_dir(dir) {
            for file in files.filter_map(Result::ok){
                let path = file.path();

                if path.is_file() && path.file_name().map_or(false, |name| name == file_to_find){
                    return path.to_str().map(String::from);
                }
            }
        } 
    }

    None
}
