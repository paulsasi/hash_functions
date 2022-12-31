use bit_vec::BitVec;
use hash_functions::md5;

fn main() {

    let data  = BitVec::from_elem(8, false);

    let result = md5::md5::hash(data);

    println!("Hello in English: {}", result.len());
}
