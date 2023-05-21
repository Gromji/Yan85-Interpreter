use core::fmt;

pub const CODE_SIZE: usize = 0x300;
pub const MEM_SIZE: usize = 0x100;

pub struct VMState {
    pub code: Vec<u8>,
    pub mem: Vec<u8>,
    pub reg_a: u8,
    pub reg_b: u8,
    pub reg_c: u8,
    pub reg_d: u8,
    pub reg_s: u8,
    pub reg_i: u8,
    pub reg_f: u8,
}

impl VMState {
    pub fn print_regs(&self) {
        println!("[I] {}", reg_string_generator(self.reg_a, self.reg_b, self.reg_c, self.reg_d, self.reg_s, self.reg_i, self.reg_f))
    }
}

fn check(index: usize) -> bool {
    return if index != 0 && index % 0x10 == 0 {
        true
    } else {
        false
    };
}

fn print_helper(arg: &Vec<u8>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (index, element) in arg.iter().enumerate() {
        if check(index) {
            write!(f, "\n")?;
        }
        write!(f, "{:X} ", element)?;
    }
    write!(f, "\n")
}

fn reg_string_generator(
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_s: u8,
    reg_i: u8,
    reg_f: u8,
) -> String {
    format!("a: {0:x}, b: {1:x}, c: {2:x}, d: {3:x}, s: {4:x}, i: {5:x}, f: {6:x}", reg_a, reg_b, reg_c, reg_d, reg_s, reg_i, reg_f)
}

fn print_regs(
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_s: u8,
    reg_i: u8,
    reg_f: u8,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    writeln!(f, "INFO REGISTERS")?;
    writeln!(
        f,
        "{}",
        reg_string_generator(reg_a, reg_b, reg_c, reg_d, reg_s, reg_i, reg_f)
    )
}

impl Default for VMState {
    fn default() -> Self {
        let mut vm_state = VMState {
            code: Vec::new(),
            mem: Vec::new(),
            reg_a: Default::default(),
            reg_b: Default::default(),
            reg_c: Default::default(),
            reg_d: Default::default(),
            reg_s: Default::default(),
            reg_i: Default::default(),
            reg_f: Default::default(),
        };

        vm_state.code.resize(CODE_SIZE, 0u8);
        vm_state.mem.resize(MEM_SIZE, 0u8);

        vm_state
    }
}

impl fmt::Display for VMState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dashes = "-".repeat(0x10 * 3);

        writeln!(f, "{}", dashes)?;
        writeln!(f, "INFO CODE")?;
        print_helper(&self.code, f)?;

        writeln!(f, "{}", dashes)?;
        writeln!(f, "INFO MEMORY")?;
        print_helper(&self.mem, f)?;

        writeln!(f, "{}", dashes)?;
        print_regs(self.reg_a, self.reg_b, self.reg_c, self.reg_d, self.reg_s, self.reg_i, self.reg_f, f)?;

        write!(f, "{}", dashes)
    }
}
