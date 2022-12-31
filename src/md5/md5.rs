use bit_vec::BitVec;
use super::super::bit_vec_extended::serializers;

pub fn hash(data: BitVec) -> BitVec {

    let message_size = 512;
    let block_size = 64; // max 128
    generate_message(data, message_size, block_size);

    BitVec::from_elem(300, false)

}

fn generate_message(mut data: BitVec,message_size: usize, block_size: usize) -> BitVec {
    let input_size = data.len();

    data.push(true); // pad a 1 bit

    let mut current_size: usize = input_size;
    loop {
        current_size += 1;
        if current_size % message_size == (message_size - block_size) {
            break
        }

        data.push(false); // pad a 0 bit
    }

    // data length in binary
    let mut input_size_bits = BitVec::from_bytes(&(input_size as u128).to_be_bytes());
    let mut input_size_bits_block = input_size_bits.split_off(128 - block_size);
    data.append(&mut input_size_bits_block);

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use bit_vec::BitVec;

    #[test]
    fn test_generate_message_1() {
        let input  = BitVec::from_elem(5, false); // 0000 0
        let message_size = 8;
        let block_size = 2;

        let expected = BitVec::from_bytes(&[0b0000_0101]);

        assert_eq!(generate_message(input, message_size, block_size), expected);
    }

    #[test]
    fn test_generate_message_2() {
        let input  = BitVec::from_elem(4, false); // 0000
        let message_size = 8;
        let block_size = 2;

        let expected = BitVec::from_bytes(&[0b0000_1000]);

        assert_eq!(generate_message(input, message_size, block_size), expected);
    }

    #[test]
    fn test_generate_message_3() {
        let input  = BitVec::from_elem(6, false); // 0000 00
        let message_size = 8;
        let block_size = 2;

        let expected = BitVec::from_bytes(&[0b0000_0010, 0b0000_0010]);

        assert_eq!(generate_message(input, message_size, block_size), expected);
    }

    #[test]
    fn test_generate_message_4() {
        let input = BitVec::from_bytes(&[0b0101_0100, 0b0110_1000, 0b0110_0101,
            0b0111_1001, 0b0010_0000, 0b0110_0001, 0b0111_0010, 0b0110_0101, 0b0010_0000,
            0b0110_0100, 0b0110_0101, 0b0111_0100, 0b0110_0101, 0b0111_0010, 0b0110_1101,
            0b0110_1001, 0b0110_1110, 0b0110_1001, 0b0111_0011, 0b0111_0100, 0b0110_1001,
            0b0110_0011]);

        let message_size = 512;
        let block_size = 64;

        let expected = BitVec::from_bytes(&[0b0101_0100, 0b0110_1000, 0b0110_0101,
            0b0111_1001, 0b0010_0000, 0b0110_0001, 0b0111_0010, 0b0110_0101, 0b0010_0000,
            0b0110_0100, 0b0110_0101, 0b0111_0100, 0b0110_0101, 0b0111_0010, 0b0110_1101,
            0b0110_1001, 0b0110_1110, 0b0110_1001, 0b0111_0011, 0b0111_0100, 0b0110_1001,
            0b0110_0011, 0b1000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            0b1011_0000]);

        assert_eq!(generate_message(input, message_size, block_size), expected);
    }



}