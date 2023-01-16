mod symbol;
mod pre;
mod inp;
mod print; use print::*;
mod state;use state::*;
mod file;use file::*;
use std::env;
use std::path::Path;
fn main() {
    if Path::new("key.txt").exists() {
        print!("***\nA file named key.txt already exists in this directory.\n");
        print!("Please move this file to proceed.\n***\n");
        std::process::exit(0);
    }
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        print!("invoke with arguments as \nkey [range] [max_inp] [max_pre] [num_responses] [num_states] [alphabet]\n");
        std::process::exit(0);
    }
    let range = match args[1].parse::<u8>() {
        Ok(u) => u,
        Err(_) => 2,
    };
    let max_inp = match args[2].parse::<usize>() {
        Ok(u) => u,
        Err(_) => 2,
    };
    let max_pre = match args[3].parse::<usize>() {
        Ok(u) => u,
        Err(_) => 2,
    };
    let num_responses = match args[4].parse::<usize>() {
        Ok(u) => u,
        Err(_) => 2,
    };
    let num_states = match args[5].parse::<usize>() {
        Ok(u) => u,
        Err(_) => 2,
    };
    let alphabet = &args[6];
    let k = random_key(range, max_inp, max_pre, num_responses,num_states);
    print_key(&k, alphabet);
    print!("SUCCESS\n{}\n",key_as_string(&k,alphabet));
    write_key(&k, alphabet);
}