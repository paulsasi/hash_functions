use bit_vec::BitVec;

pub fn to_str(data: &BitVec) -> String {
    let mut result = String::new();
    for bit in data {
        result.push(match bit {
            true => '1',
            false => '0'
        });
    }

    result
}