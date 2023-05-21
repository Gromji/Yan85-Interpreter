use crate::instruction::{describe_reg, Instruction};
use crate::vm::VMState;

pub fn interpret_instruction(vm_state: &mut VMState, instruction: Instruction) {
    vm_state.print_regs();
    print!("{}", instruction);

    match instruction.op_code {
        0x1 => interpret_imm(vm_state, &instruction),
        0x2 => interpret_add(vm_state, &instruction),
        0x4 => interpret_stk(vm_state, &instruction),
        0x8 => interpret_stm(vm_state, &instruction),
        0x10 => interpret_ldm(vm_state, &instruction),
        0x20 => interpret_cmp(vm_state, &instruction),
        0x40 => interpret_jmp(vm_state, &instruction),
        /** TODO: 0x80 => interpret_sys(vm_state, &instruction), */
        _ => panic!("Unknown op code!")
    }
}

fn read_reg(vm_state: &mut VMState, reg: u8) -> u8 {
    match reg {
        0x1 => return vm_state.reg_a,
        0x2 => return vm_state.reg_b,
        0x4 => return vm_state.reg_c,
        0x8 => return vm_state.reg_d,
        0x10 => return vm_state.reg_s,
        0x20 => return vm_state.reg_i,
        0x40 => return vm_state.reg_f,
        _ => panic!("Unknown register")
    }
}

fn write_reg(vm_state: &mut VMState, reg: u8, val: u8) {
    match reg {
        0x1 => vm_state.reg_a = val,
        0x2 => vm_state.reg_b = val,
        0x4 => vm_state.reg_c = val,
        0x8 => vm_state.reg_d = val,
        0x10 => vm_state.reg_s = val,
        0x20 => vm_state.reg_i = val,
        0x40 => vm_state.reg_f = val,
        _ => panic!("Unknown register")
    }
}

fn read_memory(vm_state: &mut VMState, address: u8) -> u8 {
    vm_state.mem[address as usize]
}

fn write_memory(vm_state: &mut VMState, address: u8, val: u8) {
    vm_state.mem[address as usize] = val
}

fn interpret_imm(vm_state: &mut VMState, instruction: &Instruction) {
    write_reg(vm_state, instruction.first, instruction.second)
}

fn interpret_add(vm_state: &mut VMState, instruction: &Instruction) {
    let reg_value = read_reg(vm_state, instruction.first) + read_reg(vm_state, instruction.second);
    write_reg(vm_state, instruction.first, reg_value)
}

fn interpret_stk(vm_state: &mut VMState, instruction: &Instruction) {
    if instruction.first != 0 {
        println!("[s] ... pushing {}", describe_reg(instruction.first));
        vm_state.reg_s += 1;
        let reg_value = read_reg(vm_state, instruction.first);
        write_memory(vm_state, vm_state.reg_s, reg_value)
    }

    if instruction.second != 0 {
        println!("[s] ... popping {}", describe_reg(instruction.second));
        let reg_value = read_memory(vm_state, vm_state.reg_s);
        write_reg(vm_state, instruction.second, reg_value);
        vm_state.reg_s -= 1
    }
}

fn interpret_stm(vm_state: &mut VMState, instruction: &Instruction) {
    let reg_value = read_reg(vm_state, instruction.second);
    let address = read_reg(vm_state, instruction.first);
    write_memory(vm_state, address, reg_value)
}

fn interpret_ldm(vm_state: &mut VMState, instruction: &Instruction) {
    let reg_value = read_reg(vm_state, instruction.second);
    let mem_value = read_memory(vm_state, reg_value);
    write_reg(vm_state, instruction.first, mem_value);
}

fn interpret_cmp(vm_state: &mut VMState, instruction: &Instruction) {
    let first_reg = read_reg(vm_state, instruction.first);
    let second_reg = read_reg(vm_state, instruction.second);
    vm_state.reg_f = 0;
    if first_reg < second_reg {
        vm_state.reg_f |= 0x1
    }
    if first_reg > second_reg {
        vm_state.reg_f |= 0x2
    }
    if first_reg == second_reg {
        vm_state.reg_f |= 0x4
    }
    if first_reg != second_reg {
        vm_state.reg_f |= 0x8
    }
    if first_reg != 0 && second_reg != 0 {
        vm_state.reg_f |= 0x10
    }
}

fn interpret_jmp(vm_state: &mut VMState, instruction: &Instruction) {
    if instruction.first != 0 && (instruction.first & vm_state.reg_f) == 0 {
        println!("[j] ... NOT TAKEN")
    } else {
        println!("[j] ... TAKEN");
        vm_state.reg_i = read_reg(vm_state, instruction.second)
    }
}