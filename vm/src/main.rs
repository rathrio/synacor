use std::process;
use std::collections::HashMap;

struct VM {
    registers: HashMap<u16, u16>,
    stack: Vec<u16>,
}

impl VM {
    fn is_literal(n: &u16) -> bool {
        n >= &0 && n <= &32767
    }

    fn value(&self, n: &u16) -> u16 {
        if VM::is_literal(n) {
            return *n;
        }

        *self.registers.get(n).unwrap()
    }

    fn interpret(&mut self, binary: &[u8]) {
        let program: Vec<u16> = binary
            .chunks(2)
            .map(|le_pair| {
                let low_byte = le_pair[0];
                let high_byte = le_pair[1];

                ((low_byte as u16) << 0) | ((high_byte as u16) << 8)
            }).collect();

        let mut pc = 0;

        loop {
            let op = program[pc];

            match op {
                // HALT
                0 => process::exit(0),
                // // SET
                // 1 => {
                //     let a = program[pc + 1];
                //     let b = program[pc + 2];
                //     let value = self.value(&b);
                //     self.registers.insert(a, value);
                //     pc += 3;
                // }
                // // PUSH
                // 2 => {
                //     let a = program[pc + 1];
                //     let value = self.value(&a);
                //     self.stack.push(value);
                //     pc += 2
                // }
                // // POP
                // 3 => {
                //     let a = program[pc + 1];
                //     let value = self.stack.pop().unwrap();
                //     self.registers.insert(a, value);
                //     pc += 2
                // }
                // // EQ
                // 4 => {
                //     let a = program[pc + 1];
                //     let b = program[pc + 2];
                //     let c = program[pc + 3];

                //     let value = if self.value(&b) == self.value(&c) {
                //         1
                //     } else {
                //         0
                //     };

                //     self.registers.insert(a, value);
                //     pc += 4
                // }
                // // GT
                // 5 => {
                //     let a = program[pc + 1];
                //     let b = program[pc + 2];
                //     let c = program[pc + 3];

                //     let value = if self.value(&b) > self.value(&c) {
                //         1
                //     } else {
                //         0
                //     };

                //     self.registers.insert(a, value);
                //     pc += 4
                // }
                // JMP
                6 => {
                    let a = program[pc + 1];
                    pc = self.value(&a) as usize;
                }
                // JT
                7 => {
                    let a = program[pc + 1];
                    let b = program[pc + 2];

                    if self.value(&a) == 0 {
                        pc += 3;
                    } else {
                        pc = self.value(&b) as usize;
                    }
                }
                // JF
                8 => {
                    let a = program[pc + 1];
                    let b = program[pc + 2];

                    if self.value(&a) == 0 {
                        pc = self.value(&b) as usize;
                    } else {
                        pc += 3;
                    }
                }
                // OUT
                19 => {
                    let a = program[pc + 1];
                    print!("{}", self.value(&a) as u8 as char);
                    pc += 2;
                },
                // NOOP
                21 => (pc += 1),
                _ => {
                    panic!("Don't know how to interpret op code {}", op);
                },
            }
        }
    }
}

fn main() {
    let binary = include_bytes!("../../challenge.bin");

    let mut vm = VM {
        registers: HashMap::new(),
        stack: Vec::new(),
    };

    vm.interpret(binary);
}