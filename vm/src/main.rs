use std::process;
use std::collections::HashMap;

fn main() {
    let buffer = include_bytes!("../../challenge.bin");
    let mut numbers = buffer
        .chunks(2)
        .map(|le_pair| {
            let low_byte = le_pair[0];
            let high_byte = le_pair[1];

            ((low_byte as u16) << 0) | ((high_byte as u16) << 8)
        });

    let mut registers: HashMap<u16, u16> = HashMap::new();

    loop {
        let n = numbers.next().unwrap();

        match n {
            // HALT
            0 => process::exit(0),
            // OUT
            19 => {
                let a = numbers.next().unwrap();
                print!("{}", a as u8 as char);
            },
            // NOOP
            21 => (),
            _ => (),
        }
    }
}
