use std::fs::*;
use std::io::Write;
use crate::print::*;
use crate::state::*;
pub fn write_key(k : &Key, alphabet : &str) {
    let mut file = File::create( format!("key.txt")  ).unwrap();
    let s = key_as_string(k,alphabet);
    writeln!(file, "{}", &s).unwrap();
}
pub fn response_as_string(r : &Response, alphabet : &str) ->String {
    format!("({},{},{})", 
        symbol_as_string(&r.read,alphabet),
        symbol_as_string(&r.write,alphabet),
        r.go
    )
}
pub fn state_as_string(s : &State, alphabet : &str) -> String {
    let mut string = "".to_string();
    for i in 0..s.len() {
        string.push_str(&response_as_string(&s[i], alphabet));
    }
    string
}
pub fn key_as_string(k :&Key, alphabet : &str) -> String {
    let mut string = "".to_string();
    for i in 0..k.len() {
        string.push_str(&state_as_string(&k[i], alphabet));
        if i != k.len() - 1 {
            string.push('\n');
        }
    }
    string
}
