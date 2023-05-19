use core::fmt;

pub const CODE_SIZE: usize = 0x300;
pub const MEM_SIZE: usize = 0x100;

pub struct VMState {
    pub code: [i8; CODE_SIZE],
    pub mem: [i8; MEM_SIZE],
    pub reg_a: i8,
    pub reg_b: i8,
    pub reg_c: i8,
    pub reg_d: i8,
    pub reg_s: i8,
    pub reg_i: i8,
    pub reg_f: i8,
}

fn check(index: usize) -> bool {
    return if index != 0 && index % 0x10 == 0 {
        true
    } else {
        false
    };
}

fn print_code(code: [i8; CODE_SIZE], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "INFO CODE BYTES")?;
    for (index, element) in code.iter().enumerate() {
        if check(index) {
            write!(f, "\n")?;
        }
        write!(f, "{:X} ", element)?;
    }
    write!(f, "\n")
}

fn print_mem(mem: [i8; MEM_SIZE], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "INFO MEMORY")?;
    for (index, element) in mem.iter().enumerate() {
        if check(index) {
            write!(f, "\n")?;
        }
        write!(f, "{:X} ", element)?;
    }
    write!(f, "\n")
}

fn print_regs(
    reg_a: i8,
    reg_b: i8,
    reg_c: i8,
    reg_d: i8,
    reg_s: i8,
    reg_i: i8,
    reg_f: i8,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    writeln!(f, "INFO REGISTERS")?;
    writeln!(
        f,
        "a: {0:x}, b: {1:x}, c: {2:x}, d: {3:x}, s: {4:x}, i: {5:x}, f: {6:x}",
        reg_a, reg_b, reg_c, reg_d, reg_s, reg_i, reg_f
    )
}

impl fmt::Display for VMState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code;
        let mem = self.mem;
        let reg_a = self.reg_a;
        let reg_b = self.reg_b;
        let reg_c = self.reg_c;
        let reg_d = self.reg_d;
        let reg_s = self.reg_s;
        let reg_i = self.reg_i;
        let reg_f = self.reg_f;

        let dashes = "-".repeat(0x10 * 3);

        writeln!(f, "{}", dashes)?;
        print_code(code, f)?;

        writeln!(f, "{}", dashes)?;
        print_mem(mem, f)?;

        writeln!(f, "{}", dashes)?;
        print_regs(reg_a, reg_b, reg_c, reg_d, reg_s, reg_i, reg_f, f)?;

        write!(f, "{}", dashes)
    }
}
