pub fn bytes_to_bits(bytes: Vec<u8>) -> Vec<bool> {
    let mut bits = Vec::new();
    for byte in bytes {
        // Iterate over each bit in the byte
        for i in 0..8 {
            // Shift the byte `i` places and check if the last bit is 1
            let bit = (byte >> i) & 1;
            bits.push(bit == 1);
        }
    }
    bits
}

pub fn bytes_to_bitstring(bytes: Vec<u8>) -> String {
    let mut bitstring = String::new();
    for byte in bytes {
        // Iterate over each bit in the byte
        for i in 0..8 {
            // Shift the byte `i` places and check if the last bit is 1
            let bit = (byte >> i) & 1;
            bitstring.push_str(&bit.to_string());
        }
    }
    bitstring
}
