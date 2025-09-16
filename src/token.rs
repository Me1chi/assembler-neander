pub enum Token {
    Nop,
    Sta,
    Lda,
    Add,
    Or,
    And,
    Not,
    Jmp,
    Jn,
    Jz,
    Hlt,
}

impl Token {

    pub fn from_text(s: &str) -> Self {

        use Token::*;

        match s {
            "nop" => Nop,
            "sta" => Sta,
            "lda" => Lda,
            "add" => Add,
            "or" => Or,
            "and" => And,
            "not" => Not,
            "jmp" => Jmp,
            "jn" => Jn,
            "jz" => Jz,
            "hlt" => Hlt,
            _ => Nop
        }

    }

    pub fn to_opcode(&self) -> u8 {

        use Token::*;

        16 * match self {
            Nop => 0b0001,
            Sta => 0b0010,
            Lda => 0b0010,
            Add => 0b0011,
            Or => 0b0100,
            And => 0b0101,
            Not => 0b0110,
            Jmp => 0b1000,
            Jn => 0b1001,
            Jz => 0b1010,
            Hlt => 0b1111,
        }
    }
}
