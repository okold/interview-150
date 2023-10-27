fn main() {
    // also probably too cheesy
    pub fn reverse_bits(x: u32) -> u32 {
        return x.reverse_bits();
    }

    assert_eq!(reverse_bits(0b00000010100101000001111010011100), 964176192);
}
