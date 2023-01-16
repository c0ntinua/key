use crate::symbol::*;
use rand::Rng;
pub fn random_prefix_code(range : u8, max_length : usize, num_symbols : usize, tries : usize) -> Code {
    let mut tries_so_far = 0;
    loop {
        let code = random_code(range, max_length, num_symbols);
        tries_so_far += 1;
        if is_prefix_code(&code) {
            return code;
        }
        if tries_so_far >= tries {
            print!("ERROR! Prefix code could not be generated. Please check parameters.\n");
            std::process::exit(0);
        }
    }
}
pub fn random_code(range : u8, max_length : usize, num_symbols : usize) -> Code {
    let mut rng = rand::thread_rng();
    let mut code = vec!();
    for _ in 0..num_symbols {
        let length = rng.gen_range(1..=max_length);
        code.push(random_symbol(range, length));
    }
    code
}
pub fn is_prefix_code(code : &Code) -> bool {
    for i in 0..code.len() {
        for j in 0..code.len() {
            if i != j && code[i].starts_with(&code[j]) {
                return false;
            }
        }
    }
    true
}