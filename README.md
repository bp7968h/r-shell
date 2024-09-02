# Rust POSIX Shell Project (r-shell)
Currently, I am learning rust, so there might me many places to improve, as I go along I will improve these code.

### `Any suggestions are welcomed`

This project, I built my own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, I learnt about shell command parsing,
REPLs, builtin commands, and more.

## Overview
The Rust POSIX Shell (r-shell) is a minimalistic shell built with Rust, demonstrating fundamental concepts of shell development including command parsing, execution, and interaction with the underlying operating system. This project is intended as a learning tool to explore the capabilities of Rust in creating command-line interfaces and system tools.

## Features
- **Command Parsing**: Simple parsing logic to interpret user inputs.
- **REPL Loop**: Read-Eval-Print Loop for continuous command execution.
- **Built-in Commands**: Includes basic built-in commands such as `cd`, `pwd`, `echo`, and `exit`.
- **External Command Execution**: Ability to run system commands that are available in the system's `PATH`.
- **POSIX Compliance**: Aims to adhere to POSIX standards for command execution.

## Prerequisites
To build and run the r-shell, you will need:
- Rust programming environment: Ensure you have `rustc` (Rust compiler) and `cargo` (Rust package manager) installed. [Install Rust](https://www.rust-lang.org/tools/install).

## Installation
1. **Clone the Repository**:
   ```
   git clone https://github.com/bpandit/r-shell.git
   cd r-shell
   ```

2. **Build the Project**:
   ```
   cargo build --release
   ```

3. **Run the Shell**:
   ```
   cargo run
   ```

## Usage
Start the shell and type commands. Examples include:
1. **Change Directory**:
   ```
   cd /path/to/directory
   cd ~
   cd
   ```
2. **Print Working Directory**
   ```
   pwd
   ```
3. **Echo a Message**
   ```
   echo hello
   echo hello, world
   ```
4. **Exit Shell**
   ```
   exit
   ```
5. **Command Type**
Gives the type of command, if this is inbuilt or command from PATH directory
   ```
   type command-name
   ```
6. **Command From PATH**
Can trigger any command found in the PATH environment variable of the user such as ls, cat etc.
   ```
   ls -l
   ```

## Future Improvements
- **Refactor**: Refactor Code
- **Command History**: Add command history to navigate through previously entered commands.
- **Script Execution**: Support executing shell scripts.
- **Enhanced Error Handling**: Improve error handling mechanisms.
- **Additional Built-in Commands**: Expand the list of built-in commands.

## License
This project is licensed under the MIT License.
