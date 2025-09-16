//
//  - Now THERE'RE LITERALLY ONLY INSTRUCTIONS AND NUMBERS
//      - Just parse each line to a pair (Instruction, Option<Operand>)
//
//  - Write the output.mem
//  - Done

pub const BUILD_FILE_NAME: &str = "build";

pub enum CustomError {
    Instruction,
    Label,
    Directive,
    MalformedLine,
    NotEnoughMemory,
}

pub struct ErrorLog {
    errs: Vec<(CustomError, usize)>, // usize is used to store the line of the error
}



