mod vm;
use vm::{VMState, CODE_SIZE, MEM_SIZE};

fn main() {
    println!("[+] This is an custom emulator. It emulates a completely custom");
    println!("[+] architecture that we call \"Yan85\"!");

    let mut vm_state = VMState {
        code: [0i8; CODE_SIZE],
        mem: [0i8; MEM_SIZE],
        reg_a: Default::default(),
        reg_b: Default::default(),
        reg_c: Default::default(),
        reg_d: Default::default(),
        reg_s: Default::default(),
        reg_i: Default::default(),
        reg_f: Default::default(),
    };
    
    

    println!("{}", vm_state);
}
