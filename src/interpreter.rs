use crate::instruction_interpreter::interpret_instruction;
use crate::vm::VMState;


pub fn interpreter_loop(vm_state: &mut VMState) {
    println!("[+] Starting interpreter loop! Good luck!");

    loop {
        let current_ptr = vm_state.reg_i;
        vm_state.reg_i += 1;
        interpret_instruction()
    }
}
