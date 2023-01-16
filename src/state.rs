use crate::symbol::*;
use crate::pre::*;
use crate::inp::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
pub struct Response {
    pub read : Symbol,
    pub write : Symbol,
    pub go : usize,
}
pub type State = Vec<Response>;
pub type Key = Vec<State>;
pub fn random_state(range : u8, max_inp : usize, max_pre : usize, num_responses : usize ) -> State {
    let mut state = vec!();
    let prefix_code = random_prefix_code(range,max_pre,num_responses,10000 );
    let input_code = random_input_code(range, max_inp, num_responses);
    for i in 0..num_responses {
        state.push(Response { read : input_code[i].clone(), write : prefix_code[i].clone(), go : 0 });
    }
    state
}
pub fn random_key(range : u8, max_inp : usize, max_pre : usize, num_responses: usize, num_states : usize) -> Key {
    let mut key = vec!();
    for _ in 0..num_states {
        key.push(random_state(range,max_inp,max_pre, num_responses));
    }
    for response_index in 0..num_responses {
        let next_state = shuffled_count(num_states);
        for i in 0..key.len() {
            key[i][response_index].go = next_state[i];
        }
    }
    key
}
pub fn shuffled_count(length : usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..length).collect();
    v.shuffle(&mut thread_rng());
    v
}