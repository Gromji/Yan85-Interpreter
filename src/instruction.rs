use std::fmt;
use std::fmt::Formatter;

pub struct Instruction{
    pub op_code: u8,
    pub first: u8,
    pub second: u8
}

pub fn describe_reg(reg: u8) -> String {
    match reg {
        0x1 => String::from("a"),
        0x2 => String::from("b"),
        0x4 => String::from("c"),
        0x8 => String::from("d"),
        0x10 => String::from("s"),
        0x20 => String::from("i"),
        0x40 => String::from("f"),
        _ => panic!("Unknown register")
    }
}

pub fn describe_flag(flag: u8) -> String{
    let mut result = String::new();
    if flag & 0x1{
        result += "L";
    }
    if flag & 0x2{
        result += "G";
    }
    if flag & 0x4{
        result += "E";
    }
    if flag & 0x8{
        result += "N";
    }
    if flag & 0x10{
        result += "Z";
    }
    if !flag{
        result += "*";
    }
    result
}

impl fmt::Display for Instruction{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.op_code {
            0x1 => writeln!(f, "IMM {} = 0x{:x}", describe_reg(self.first), self.second)?,
            0x2 => writeln!(f, "ADD {} {}", describe_reg(self.first), describe_reg(self.second))?,
            0x4 => writeln!(f, "STK {} {}", describe_reg(self.first), describe_reg(self.second))?,
            0x8 => writeln!(f, "STM *{} = {}", describe_reg(self.first), describe_reg(self.second))?,
            0x10 => writeln!(f, "LDM {} = *{}", describe_reg(self.first), describe_reg(self.second))?,
            0x20 => writeln!(f, "CMP {} {}", describe_reg(self.first), describe_reg(self.second))?,
            0x40 => writeln!(f, "JMP {} {}", describe_reg(self.first), describe_flag(self.second))?,
            0x80 => writeln!(f, "SYS {} {}", self.first, describe_reg(self.second))?,
            _ => panic!("Unknown instruction!")
        }
        writeln!()
    }
}