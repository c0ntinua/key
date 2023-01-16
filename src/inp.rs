use crate::symbol::*;
use rand::Rng;

pub fn random_input_code(range : u8, max_length : usize, num_symbols : usize) -> Code {
    debug_assert!(num_symbols > 1);
    let mut finished = false;
    let extra_symbols = num_symbols - range as usize;
    let mut rng = rand::thread_rng();
    let mut code = vec!();
    while extra_symbols > 0 && !finished {
        code = vec!();
        for _ in 0..extra_symbols {
            let length = rng.gen_range(range..=max_length as u8);
            code.push(random_symbol(range, length as usize) );
        }
        finished = if !symbol_appears_twice_in(&code){ true} else {false};
    }
    for i in 0u8..range {
        code.push(vec![i]);
    }
    code
}

pub fn symbol_appears_twice_in(code : &Code) -> bool {
    for i in 0..code.len() {
        for j in 0..code.len() {
            if i != j && code[i].starts_with(&code[j]) {
                return true;
            }
        }
    }
    false
}