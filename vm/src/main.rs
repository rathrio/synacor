fn main() {
    let buffer = include_bytes!("../../challenge.bin");

    for le_pair in buffer.chunks(2) {
        let low_byte = le_pair[0];
        let high_byte = le_pair[1];

        let number = ((low_byte as u16) << 0) | ((high_byte as u16) << 8);
        println!("{}", number);
    }
}
