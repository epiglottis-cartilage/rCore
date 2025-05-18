#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate libr;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;
const LINE_START: &str = "bish > ";

use alloc::string::String;
use alloc::vec::Vec;
use libr::console::getchar;
use libr::{OpenFlag, close, dup, exec, fork, open, pipe, waitpid};

#[derive(Debug)]
struct ProcessArguments<'a> {
    input: Option<&'a str>,
    output: Option<&'a str>,
    args: Vec<&'a str>,
}

impl<'a> ProcessArguments<'a> {
    pub fn new(command: &'a str) -> Self {
        let mut args: Vec<_> = command.split(' ').collect();

        // redirect input
        let mut input = None;
        if let Some(idx) = args.iter().position(|arg| *arg == "<") {
            input = Some(args[idx + 1]);
            args.drain(idx..=idx + 1);
        }

        // redirect output
        let mut output = None;
        if let Some(idx) = args.iter().position(|arg| *arg == ">") {
            output = Some(args[idx + 1]);
            args.drain(idx..=idx + 1);
        }

        let mut args_addr: Vec<*const u8> = args.iter().map(|arg| arg.as_ptr()).collect();
        args_addr.push(core::ptr::null::<u8>());

        Self {
            input,
            output,
            args,
        }
    }
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!("{}", LINE_START);
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    let splited: Vec<_> = line.as_str().split('|').collect();
                    let process_arguments_list: Vec<_> = splited
                        .iter()
                        .map(|&cmd| ProcessArguments::new(cmd))
                        .collect();
                    let mut valid = true;
                    for (i, process_args) in process_arguments_list.iter().enumerate() {
                        if i == 0 {
                            if process_args.output.is_some() {
                                valid = false;
                            }
                        } else if i == process_arguments_list.len() - 1 {
                            if process_args.input.is_some() {
                                valid = false;
                            }
                        } else if process_args.output.is_some() || process_args.input.is_some() {
                            valid = false;
                        }
                    }
                    if process_arguments_list.len() == 1 {
                        valid = true;
                    }
                    if !valid {
                        println!("Invalid command: Inputs/Outputs cannot be correctly binded!");
                    } else {
                        // create pipes
                        let mut pipes_fd: Vec<(usize, usize)> = Vec::new();
                        if !process_arguments_list.is_empty() {
                            for _ in 0..process_arguments_list.len() - 1 {
                                let pipe_fd = pipe().unwrap();
                                pipes_fd.push(pipe_fd);
                            }
                        }
                        let mut children: Vec<_> = Vec::new();
                        for (i, process_argument) in process_arguments_list.iter().enumerate() {
                            let pid = fork();
                            if pid == 0 {
                                let ProcessArguments {
                                    input,
                                    output,
                                    args,
                                } = &process_argument;

                                // redirect input
                                if let Some(input) = input {
                                    let input_fd = open(input, OpenFlag::RDONLY);
                                    if input_fd == -1 {
                                        println!("Error when opening file {}", input);
                                        return -4;
                                    }
                                    let input_fd = input_fd as usize;
                                    close(0);
                                    assert_eq!(dup(input_fd), 0);
                                    close(input_fd);
                                }
                                // redirect output
                                if let Some(output) = output {
                                    let output_fd =
                                        open(output, OpenFlag::CREATE | OpenFlag::WRONLY);
                                    if output_fd == -1 {
                                        println!("Error when opening file {}", output);
                                        return -4;
                                    }
                                    let output_fd = output_fd as usize;
                                    close(1);
                                    assert_eq!(dup(output_fd), 1);
                                    close(output_fd);
                                }
                                // receive input from the previous process
                                if i > 0 {
                                    close(0);
                                    let read_end = pipes_fd.get(i - 1).unwrap().0;
                                    assert_eq!(dup(read_end), 0);
                                }
                                // send output to the next process
                                if i < process_arguments_list.len() - 1 {
                                    close(1);
                                    let write_end = pipes_fd.get(i).unwrap().1;
                                    assert_eq!(dup(write_end), 1);
                                }
                                // close all pipe ends inherited from the parent process
                                for pipe_fd in pipes_fd.iter() {
                                    close(pipe_fd.0);
                                    close(pipe_fd.1);
                                }
                                // execute new application
                                if exec(args[0], args) == -1 {
                                    println!("Error when executing!");
                                    return -4;
                                }
                                unreachable!();
                            } else {
                                children.push(pid);
                            }
                        }
                        for pipe_fd in pipes_fd.iter() {
                            close(pipe_fd.0);
                            close(pipe_fd.1);
                        }
                        let mut exit_code: i32 = 0;
                        for pid in children.into_iter() {
                            let exit_pid = waitpid(pid as usize, &mut exit_code);
                            assert_eq!(pid, exit_pid);
                            //println!("Shell: Process {} exited with code {}", pid, exit_code);
                        }
                    }
                    line.clear();
                }
                print!("{}", LINE_START);
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
