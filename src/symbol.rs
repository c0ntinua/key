use rand::Rng;
pub type Symbol = Vec<u8>;
pub type Code = Vec<Symbol>;

pub fn random_symbol(range : u8, length : usize) -> Symbol {
    let mut s = vec![0u8;length];
    let mut rng = rand::thread_rng();
    for e in &mut s {
        *e = rng.gen_range(0..range);
    }
    s
}