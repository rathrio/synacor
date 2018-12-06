use std::process;

struct VM {
    registers: [u16; 8],
    memory: [u16; 32_768],
    stack: Vec<u16>,
}

impl VM {
    fn new() -> VM {
        VM {
            registers: [0; 8],
            memory: [0; 32_768],
            stack: Vec::new(),
        }
    }

    fn is_literal(n: &u16) -> bool {
        n >= &0 && n <= &32_767
    }

    fn is_register(n: &u16) -> bool {
        n >= &32_768 && n <= &32_775
    }

    fn read_register(&self, register: &u16) -> u16 {
        self.registers[(register - 32_768) as usize]
    }

    fn write_register(&mut self, register: &u16, value: u16) {
        self.registers[(register - 32_768) as usize] = value;
    }

    fn value(&self, n: &u16) -> u16 {
        if VM::is_literal(n) {
            return *n;
        }

        if VM::is_register(n) {
            return self.read_register(n);
        }

        panic!("Invalid number: {}", n);
    }

    fn read_memory(&self, address: &u16) -> u16 {
        self.memory[*address as usize]
    }

    fn write_memory(&mut self, address: &u16, value: u16) {
        self.memory[*address as usize] = value;
    }

    fn load_into_memory(&mut self, binary: &[u8]) {
        binary
            .chunks(2)
            .map(|le_pair| le_pair[0] as u16 | ((le_pair[1] as u16) << 8))
            .enumerate()
            .for_each(|(address, value)| {
                self.memory[address as usize] = value;
            });
    }

    fn interpret(&mut self, binary: &[u8]) {
        self.load_into_memory(binary);

        // Start at address 0
        let mut pc: u16 = 0;

        loop {
            let op = self.read_memory(&pc);

            match op {
                // HALT
                0 => process::exit(0),
                // SET a b
                1 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let value = self.value(&b);

                    self.write_register(&a, value);
                    pc += 3;
                }
                // PUSH a
                2 => {
                    let a = self.read_memory(&(pc + 1));
                    let value = self.value(&a);
                    self.stack.push(value);
                    pc += 2
                }
                // POP a
                3 => {
                    let a = self.read_memory(&(pc + 1));
                    let value = self.stack.pop().expect("Attempted to POP empty stack");

                    self.write_register(&a, value);
                    pc += 2
                }
                // EQ a b c
                4 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let c = self.read_memory(&(pc + 3));

                    let value = if self.value(&b) == self.value(&c) {
                        1
                    } else {
                        0
                    };

                    self.write_register(&a, value);
                    pc += 4
                }
                // GT a b c
                5 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let c = self.read_memory(&(pc + 3));

                    let value = if self.value(&b) > self.value(&c) {
                        1
                    } else {
                        0
                    };

                    self.write_register(&a, value);
                    pc += 4
                }
                // JMP a
                6 => {
                    let a = self.read_memory(&(pc + 1));
                    pc = self.value(&a);
                }
                // JT a b
                7 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));

                    if self.value(&a) == 0 {
                        pc += 3;
                    } else {
                        pc = self.value(&b);
                    }
                }
                // JF a b
                8 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));

                    if self.value(&a) == 0 {
                        pc = self.value(&b);
                    } else {
                        pc += 3;
                    }
                }
                // ADD a b c
                9 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let c = self.read_memory(&(pc + 3));

                    let b_value = self.value(&b);
                    let c_value = self.value(&c);
                    let result = (b_value.wrapping_add(c_value)) % 32768;

                    self.write_register(&a, result);
                    pc += 4;
                }
                // MULT a b c
                10 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let c = self.read_memory(&(pc + 3));

                    let b_value = self.value(&b);
                    let c_value = self.value(&c);
                    let result = (b_value.wrapping_mul(c_value)) % 32768;

                    self.write_register(&a, result);
                    pc += 4;
                }
                // MOD a b c
                11 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let c = self.read_memory(&(pc + 3));

                    let b_value = self.value(&b);
                    let c_value = self.value(&c);
                    let result = b_value % c_value;

                    self.write_register(&a, result);
                    pc += 4;
                }
                // AND a b c
                12 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let c = self.read_memory(&(pc + 3));

                    let b_value = self.value(&b);
                    let c_value = self.value(&c);
                    let result = b_value & c_value;

                    self.write_register(&a, result);
                    pc += 4;
                }
                // OR a b c
                13 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));
                    let c = self.read_memory(&(pc + 3));

                    let b_value = self.value(&b);
                    let c_value = self.value(&c);
                    let result = b_value | c_value;

                    self.write_register(&a, result);
                    pc += 4;
                }
                // NOT a b
                14 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));

                    // Only flip the last 15 bits. There's probably a prettier way to do this...
                    let result = b ^ 0b_0111_1111_1111_1111_u16;

                    self.write_register(&a, result);
                    pc += 3;
                }
                // RMEM a b
                15 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));

                    let address = self.value(&b);
                    let value = self.read_memory(&address);
                    self.write_register(&a, value);
                    pc += 3;
                }
                // WMEM a b
                16 => {
                    let a = self.read_memory(&(pc + 1));
                    let b = self.read_memory(&(pc + 2));

                    let address = self.value(&a);
                    let value = self.value(&b);
                    self.write_memory(&address, value);
                    pc += 3;
                }
                // CALL a
                17 => {
                    let a = self.read_memory(&(pc + 1));
                    self.stack.push(pc + 2);
                    pc = self.value(&a);
                }
                // RET
                18 => {
                    pc = self.stack.pop().unwrap();
                }
                // OUT a
                19 => {
                    let a = self.read_memory(&(pc + 1));
                    print!("{}", self.value(&a) as u8 as char);
                    pc += 2;
                }
                // NOOP
                21 => {
                    pc += 1;
                }
                _ => {
                    panic!("Don't know how to interpret op code {}", op);
                }
            }
        }
    }
}

fn main() {
    let binary = include_bytes!("../../challenge.bin");

    let mut vm = VM::new();
    vm.interpret(binary);
}
