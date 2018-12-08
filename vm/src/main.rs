mod vm;
use vm::VM;

fn main() {
    let binary = include_bytes!("../../challenge.bin");
    let mut vm = VM::new();
    vm.interpret(binary);
}
