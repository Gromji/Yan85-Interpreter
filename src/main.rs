mod vm;
mod interpreter;
mod instruction_interpreter;
mod instruction;

use std::fs::File;
use std::io::Read;
use vm::{VMState, CODE_SIZE};
use crate::interpreter::interpreter_loop;

fn main() {
    println!("[+] This is an custom emulator. It emulates a completely custom");
    println!("[+] architecture that we call \"Yan85\"!");

    if let Some(file_path) = std::env::args().nth(1){
        let file = File::open(file_path);

        match file {
            Ok(mut file) =>{
                let mut vm_state = VMState::default();
                file.read(&mut vm_state.code).expect("Failed to read file contents");
                vm_state.code.resize(CODE_SIZE, 0u8);
                interpreter_loop(&mut vm_state);
            },
            Err(error) => panic!("Error opening the file {:?}", error)
        }
    }else {
        println!("Usage: [binary] [file_path]")
    }
}
