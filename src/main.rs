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
        print!("invoke with arguments as \nkey [range] [max_in_len] [max_out_len] [responses] [states] [alphabet]\n");
        std::process::exit(0);
    }
    let range = args[1].parse::<u8>().unwrap();
    let max_in_len = args[2].parse::<usize>().unwrap(); 
    let max_out_len = args[3].parse::<usize>().unwrap(); 
    let responses = args[4].parse::<usize>().unwrap(); 
    let states = args[5].parse::<usize>().unwrap();  
    let alphabet = &args[6];
    let k = random_key(range, max_in_len, max_out_len, responses,states);
    print!("SUCCESS\n");
    print_key(&k, alphabet); 
    //print!("SUCCESS\n{}\n",key_as_string(&k,alphabet));
    write_key(&k, alphabet);
}