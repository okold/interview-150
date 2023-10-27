fn main() {
    // ok less cheesy than using built in reverse_bits
    pub fn reverse_bits(x: u32) -> u32 {
        let mut worker = x;
        let mut rev = 0u32;

        for _ in 0..32 {
            // move rev to left and insert 1 if worker's least significant digit is a 1
            rev =  rev << 1 | worker & 1;
            // move worker right, so lsd is next lsd
            worker >>= 1;
        }

        rev
    }

    assert_eq!(reverse_bits(0b00000010100101000001111010011100), 964176192);
}
