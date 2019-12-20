/// The initial state of the memory layout for the program.
const INITIAL_MEMORY_LAYOUT: &'static [i64] = &[
2,380,379,385,1008,2799,144351,381,1005,381,12,99,109,2800,1102,0,1,383,1101,0,0,382,21001,382,0,1,21001,383,0,2,21102,37,1,0,1106,0,578,4,382,4,383,204,1,1001,382,1,382,1007,382,45,381,1005,381,22,1001,383,1,383,1007,383,24,381,1005,381,18,1006,385,69,99,104,-1,104,0,4,386,3,384,1007,384,0,381,1005,381,94,107,0,384,381,1005,381,108,1105,1,161,107,1,392,381,1006,381,161,1102,1,-1,384,1105,1,119,1007,392,43,381,1006,381,161,1101,0,1,384,20102,1,392,1,21102,1,22,2,21102,0,1,3,21101,0,138,0,1106,0,549,1,392,384,392,20102,1,392,1,21102,1,22,2,21102,3,1,3,21102,161,1,0,1105,1,549,1102,1,0,384,20001,388,390,1,20102,1,389,2,21102,180,1,0,1105,1,578,1206,1,213,1208,1,2,381,1006,381,205,20001,388,390,1,20101,0,389,2,21102,205,1,0,1105,1,393,1002,390,-1,390,1101,1,0,384,21002,388,1,1,20001,389,391,2,21101,228,0,0,1106,0,578,1206,1,261,1208,1,2,381,1006,381,253,20101,0,388,1,20001,389,391,2,21101,253,0,0,1105,1,393,1002,391,-1,391,1102,1,1,384,1005,384,161,20001,388,390,1,20001,389,391,2,21102,1,279,0,1106,0,578,1206,1,316,1208,1,2,381,1006,381,304,20001,388,390,1,20001,389,391,2,21101,304,0,0,1106,0,393,1002,390,-1,390,1002,391,-1,391,1102,1,1,384,1005,384,161,20101,0,388,1,20101,0,389,2,21102,1,0,3,21101,0,338,0,1106,0,549,1,388,390,388,1,389,391,389,20101,0,388,1,21002,389,1,2,21101,4,0,3,21101,365,0,0,1105,1,549,1007,389,23,381,1005,381,75,104,-1,104,0,104,0,99,0,1,0,0,0,0,0,0,462,20,19,1,1,22,109,3,21202,-2,1,1,21201,-1,0,2,21102,0,1,3,21101,414,0,0,1106,0,549,22102,1,-2,1,21201,-1,0,2,21102,1,429,0,1105,1,601,2101,0,1,435,1,386,0,386,104,-1,104,0,4,386,1001,387,-1,387,1005,387,451,99,109,-3,2105,1,0,109,8,22202,-7,-6,-3,22201,-3,-5,-3,21202,-4,64,-2,2207,-3,-2,381,1005,381,492,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,481,21202,-4,8,-2,2207,-3,-2,381,1005,381,518,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,507,2207,-3,-4,381,1005,381,540,21202,-4,-1,-1,22201,-3,-1,-3,2207,-3,-4,381,1006,381,529,21202,-3,1,-7,109,-8,2105,1,0,109,4,1202,-2,45,566,201,-3,566,566,101,639,566,566,1202,-1,1,0,204,-3,204,-2,204,-1,109,-4,2106,0,0,109,3,1202,-1,45,594,201,-2,594,594,101,639,594,594,20101,0,0,-2,109,-3,2105,1,0,109,3,22102,24,-2,1,22201,1,-1,1,21102,1,547,2,21102,1,67,3,21101,1080,0,4,21102,1,630,0,1105,1,456,21201,1,1719,-2,109,-3,2106,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,2,2,0,2,2,2,2,2,0,2,0,0,2,0,2,2,2,0,2,2,2,2,0,2,2,2,2,0,2,2,2,0,2,0,0,2,2,2,0,0,1,1,0,2,0,2,2,2,2,0,0,2,2,0,0,2,2,0,2,2,2,2,0,2,2,2,2,0,0,0,2,2,2,0,2,0,2,2,2,2,2,2,2,0,0,1,1,0,0,0,2,2,2,2,0,0,0,2,0,0,2,2,2,2,2,2,2,2,0,0,2,0,0,0,2,2,2,2,2,0,2,2,0,0,2,2,0,2,2,0,1,1,0,2,2,0,2,2,0,0,0,2,0,2,2,0,2,0,2,2,2,2,2,0,0,2,2,2,0,2,0,0,2,2,0,2,0,2,0,2,2,2,2,2,0,1,1,0,2,0,0,2,2,2,0,0,2,2,0,0,2,2,2,2,2,2,0,2,2,2,2,2,0,2,2,0,2,0,0,2,2,2,2,2,2,2,0,0,2,0,1,1,0,2,0,2,2,2,2,2,2,0,0,2,0,2,2,2,2,0,2,2,2,2,2,2,2,0,2,2,2,0,2,2,2,0,2,0,2,2,2,0,2,2,0,1,1,0,2,2,2,2,2,2,0,2,2,2,2,0,0,2,0,2,0,2,0,0,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,1,1,0,2,0,2,2,2,2,2,2,0,2,2,0,2,2,2,2,2,2,2,0,0,0,2,2,0,0,2,2,2,2,0,2,0,2,2,2,2,2,2,2,2,0,1,1,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,0,2,0,2,0,2,0,2,2,2,2,0,0,2,2,2,2,0,2,2,2,2,0,1,1,0,2,2,0,2,2,0,0,2,2,2,2,2,2,2,2,2,0,2,2,0,2,0,2,2,0,2,2,2,2,2,2,0,0,2,2,2,2,2,0,2,0,0,1,1,0,0,2,2,0,2,0,2,2,2,2,2,0,0,0,2,2,2,0,0,2,2,2,2,2,2,0,2,2,2,0,2,2,0,0,2,2,2,2,0,2,0,0,1,1,0,0,2,2,2,0,2,2,0,2,2,2,0,2,2,2,2,0,0,2,2,2,2,2,2,2,2,2,0,2,2,0,2,2,0,2,2,2,0,2,2,2,0,1,1,0,2,2,2,0,2,2,2,2,2,2,2,0,2,0,2,0,2,2,2,2,2,2,2,0,0,0,0,2,2,2,2,0,0,2,2,2,2,2,2,2,2,0,1,1,0,2,2,0,0,0,0,2,2,2,2,2,0,2,2,2,0,0,0,2,2,0,2,2,0,0,0,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,0,1,1,0,0,2,2,2,0,0,2,2,0,2,0,2,2,2,0,0,2,2,0,2,0,0,0,2,0,0,2,0,2,2,2,0,0,2,2,0,2,2,2,2,2,0,1,1,0,0,2,2,2,0,0,2,2,0,0,0,2,0,2,2,0,2,0,2,2,2,2,2,2,2,2,2,2,0,2,2,0,2,2,2,0,2,2,2,2,2,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,9,39,41,83,13,97,81,2,94,70,54,80,64,53,86,83,73,14,8,90,38,79,87,81,5,14,43,32,46,39,20,18,47,85,63,71,38,16,83,50,21,69,67,20,94,57,26,11,65,31,41,24,60,79,15,34,87,60,81,76,46,90,47,31,33,98,10,70,60,94,50,41,27,24,34,12,17,91,45,97,40,89,36,47,94,47,40,58,44,50,27,57,4,86,45,93,75,55,57,34,98,58,60,31,26,62,92,50,47,71,31,14,19,31,11,11,61,60,41,32,43,3,74,88,14,60,47,98,11,71,76,79,60,14,49,69,5,1,64,43,63,23,59,81,55,96,43,36,80,1,2,94,56,79,66,1,41,27,35,22,64,89,41,77,1,98,13,85,48,55,73,70,56,13,50,73,95,19,15,82,27,74,18,11,90,44,11,25,57,76,15,94,90,85,97,21,67,21,82,17,64,40,11,46,82,73,62,46,82,80,90,1,91,4,3,93,31,3,65,57,14,66,30,25,9,66,69,56,71,25,50,19,57,44,16,34,81,61,95,71,33,97,60,59,43,75,6,30,55,90,69,70,45,37,89,5,9,81,51,28,4,84,21,73,75,53,82,67,36,26,45,82,93,16,17,89,55,20,62,37,39,38,55,71,44,80,28,95,18,83,40,82,46,70,74,70,53,8,92,38,9,29,66,95,24,75,85,20,23,88,71,66,2,39,58,78,44,15,74,52,77,87,2,4,17,18,45,2,71,2,17,86,68,24,6,19,36,12,40,40,79,58,52,53,14,79,48,62,94,9,4,86,8,33,76,62,8,80,81,51,74,49,26,56,12,40,3,32,48,51,74,12,77,73,53,58,82,92,92,9,53,53,11,92,33,73,80,68,93,85,51,95,69,83,74,66,90,90,98,6,96,83,51,16,46,41,87,4,86,90,28,52,54,70,98,26,26,1,74,49,64,56,16,67,50,60,45,61,32,66,19,23,73,20,12,27,71,29,52,61,24,24,92,3,97,84,48,40,23,64,50,25,12,76,25,3,88,31,8,29,87,24,3,2,77,95,8,90,31,71,5,76,41,54,51,2,17,85,23,79,62,80,8,64,31,31,14,83,88,90,29,53,74,80,63,15,61,68,86,17,3,83,87,10,63,9,37,92,23,16,64,66,45,28,92,15,61,34,4,27,72,10,19,96,49,29,87,62,46,97,80,81,52,31,11,5,51,10,75,52,16,97,89,96,21,26,98,14,11,92,66,69,30,60,71,76,38,70,74,88,78,42,42,60,6,69,9,23,59,92,9,72,27,90,55,10,36,56,74,61,95,28,69,57,6,96,36,21,88,22,54,65,13,13,91,64,52,43,45,49,37,80,13,55,78,57,63,88,51,46,13,58,44,23,5,15,14,84,39,94,25,63,93,82,2,90,36,62,1,68,2,54,66,79,34,94,55,44,4,53,39,65,40,41,42,40,89,69,23,43,80,73,7,5,71,14,54,38,46,46,58,89,61,6,49,56,40,35,76,8,69,95,31,34,36,14,91,83,98,76,5,7,68,79,87,85,85,61,79,44,6,20,26,4,27,15,80,62,27,85,3,76,10,48,28,12,48,93,44,36,41,92,67,36,76,58,26,80,40,19,98,68,53,28,19,96,20,39,19,13,60,64,18,12,91,50,7,11,32,31,69,92,39,70,50,42,66,12,38,92,1,27,47,52,59,12,76,2,90,65,80,50,40,54,29,97,87,73,39,80,94,70,81,97,4,98,16,51,31,77,77,34,74,62,30,41,37,40,5,62,74,51,47,58,65,37,60,77,35,32,83,35,36,77,70,17,74,19,97,16,39,58,56,75,3,1,69,4,73,39,52,65,33,62,22,70,25,88,97,76,34,43,80,29,14,89,39,81,12,6,79,90,85,14,31,59,64,59,51,89,34,66,90,95,92,31,16,37,23,7,85,83,91,64,41,89,3,12,32,72,92,94,93,66,59,51,52,69,78,73,45,38,62,20,64,8,21,61,5,62,98,23,46,77,4,73,74,43,2,77,94,75,42,76,91,67,82,40,54,58,86,92,3,51,28,69,78,39,35,7,63,14,2,96,17,53,44,75,20,66,92,67,42,44,58,54,15,40,65,61,75,85,92,74,57,74,11,6,20,33,21,36,7,20,28,89,15,28,15,11,97,9,48,70,94,7,10,72,86,64,23,36,93,55,91,36,44,35,43,10,53,4,34,70,96,41,70,98,7,28,30,9,97,36,89,25,21,77,51,4,19,26,65,56,76,51,56,73,88,56,4,52,89,6,10,26,36,73,32,54,57,92,72,48,74,76,14,19,33,56,70,6,38,20,6,11,55,61,60,52,58,61,88,30,7,86,60,73,72,46,56,73,78,50,58,3,32,45,87,78,64,86,72,73,83,36,56,1,70,98,80,62,38,45,90,13,70,9,28,72,144351
];

/// The total size of the computer memory (program loaded at the beginning).
const PROGRAM_MEMORY_SIZE: u32 = 1_000_000; // 1MB

/// Represents a parameter mode defining how a parameter to an operation will be used.
#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative
}

impl std::convert::From<u8> for ParameterMode {
    /// Converts from a u8 (which should be either 0 or 1) to the appropriate
    /// parameter mode.
    fn from(input: u8) -> Self {
        match input {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("Parameter mode is not valid")
        }
    }
}

/// Represents a single parameter in an operation. A parameter will contain a mode
/// along with a value.
#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i64
}

impl Parameter {
    /// Creates a new Parameter with the given values
    fn new(mode: ParameterMode, value: i64) -> Self {
        Self {
            mode,
            value
        }
    }
    
    /// Resolves the Parameter into a number, handling the parameter
    /// mode correctly
    fn resolve_get(&self, memory: &[i64], relative_base: i64) -> i64 {
        match self.mode {
            ParameterMode::Position => memory[self.value as usize],
            ParameterMode::Immediate => self.value,
            ParameterMode::Relative => memory[(relative_base + self.value) as usize]
        }
    }

    fn resolve_set(&mut self, memory: &mut [i64], relative_base: i64, value: i64) {
        match self.mode {
            ParameterMode::Position => memory[self.value as usize] = value,
            ParameterMode::Relative => memory[(relative_base + self.value) as usize] = value,
            _ => panic!("Invalid!")
        }
    }
}

/// Structure holding the various parts of the opcode. These
/// parts are the opcode proper, as well as the parameter modes
/// of the (upto) three parameters that the operation takes.
struct OpcodeParts {
    code: u8,
    param_1_mode: ParameterMode,
    param_2_mode: ParameterMode,
    param_3_mode: ParameterMode
}

impl OpcodeParts {
    /// Constructs a new OpcodeParts instance from the raw components.
    fn new(code: u8, param_1_mode: ParameterMode, param_2_mode: ParameterMode, param_3_mode: ParameterMode) -> Self {
        Self {
            code,
            param_1_mode,
            param_2_mode,
            param_3_mode
        }
    }
    
    /// Takes a u32 opcode and splits it into the individual parts.
    fn from_opcode(mut opcode: u32) -> Self {
        // split off the ten thousands column identifying the third parameter
        // mode
        let param_3_mode = (opcode / 10_000) as u8;
        opcode %= 10_000;
        
        // split off the thousands column identifying the second parameter mode
        let param_2_mode = (opcode / 1_000) as u8;
        opcode %= 1_000;
        
        // split off the hundreds column identifying the first parameter mode
        let param_1_mode = (opcode / 100) as u8;
        opcode %= 100;
        
        // the code proper is just what's left
        let code = opcode as u8;
        
        // just throw all these together into a new OpcodeParts instance
        Self::new(code, param_1_mode.into(), param_2_mode.into(), param_3_mode.into())
    }
}

/// Represents an operation in the program. An operation will have an opcode identifying
/// what kind of operation to perform, along with one or more parameters. A parameter
/// will have a mode identifier and a value.
enum Operation<'input, 'output> {
    Add(Parameter, Parameter, Parameter),
    Mult(Parameter, Parameter, Parameter),
    In(&'input mut dyn FnMut() -> i64, Parameter),
    Out(&'output mut dyn FnMut(i64), Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equal(Parameter, Parameter, Parameter),
    RelativeBaseChange(Parameter)
}

impl Operation<'_, '_> {
    /// Counts the number of parameters a particular Operation uses
    /// so that the program counter can be incremented appropriately.
    fn parameter_count(&self) -> u8 {
        match self {
            Operation::Add(..) => 3,
            Operation::Mult(..) => 3,
            Operation::In(..) => 1,
            Operation::Out(..) => 1,
            Operation::JumpIfTrue(..) => 2,
            Operation::JumpIfFalse(..) => 2,
            Operation::LessThan(..) => 3,
            Operation::Equal(..) => 3,
            Operation::RelativeBaseChange(..) => 1
        }
    }
    
    /// Applies the operation to the given memory block.
    /// Returns a program counter override if applicable.
    fn execute(&mut self, memory: &mut [i64], relative_base: &mut i64) -> Option<usize> {
        match self {
            Operation::Add(input_1, input_2, output) => {
                let val = input_1.resolve_get(memory, *relative_base) + input_2.resolve_get(memory, *relative_base);
                output.resolve_set(memory, *relative_base, val);
                None
            },
            Operation::Mult(input_1, input_2, output) => {
                let val = input_1.resolve_get(memory, *relative_base) * input_2.resolve_get(memory, *relative_base);
                output.resolve_set(memory, *relative_base, val);
                None
            },
            Operation::In(input, output) => {
                output.resolve_set(memory, *relative_base, input());
                None
            },
            Operation::Out(output, input) => {
                output(input.resolve_get(memory, *relative_base));
                None
            },
            Operation::JumpIfTrue(input_1, input_2) => {
                if input_1.resolve_get(memory, *relative_base) != 0 {
                    Some(input_2.resolve_get(memory, *relative_base) as usize)
                } else {
                    None
                }
            },
            Operation::JumpIfFalse(input_1, input_2) => {
                if input_1.resolve_get(memory, *relative_base) == 0 {
                    Some(input_2.resolve_get(memory, *relative_base) as usize)
                } else {
                    None
                }
            },
            Operation::LessThan(input_1, input_2, output) => {
                let val = if input_1.resolve_get(memory, *relative_base) < input_2.resolve_get(memory, *relative_base) { 1 } else { 0 };
                output.resolve_set(memory, *relative_base, val); 
                None
            },
            Operation::Equal(input_1, input_2, output) => {
                let val = if input_1.resolve_get(memory, *relative_base) == input_2.resolve_get(memory, *relative_base) { 1 } else { 0 };
                output.resolve_set(memory, *relative_base, val);
                None
            },
            Operation::RelativeBaseChange(input) => {
                *relative_base += input.resolve_get(memory, *relative_base);
                None
            }
        }
    }
}

/// Represents the type of the checkpoint reached when using the
/// "run_until_next_checkpoint" function.
enum CheckpointType {
    Output,
    Terminated
}

/// Represents a program with some memory containing both the code and data for the program
/// and the program counter.
#[derive(Clone)]
struct Program {
    memory: Box<[i64]>,
    pc: usize,
    relative_base: i64
}

impl Program {
    /// Creates a new Program with the given initial memory layout.
    /// Initialises the program counter to 0.
    fn new(memory: &[i64], total_memory: u32) -> Self {
        let memory = {
            let mut memory = memory.to_vec();
            memory.resize(total_memory as usize, 0);
            memory.into_boxed_slice()
        };
        let pc = 0;
        
        Self {
            memory,
            pc,
            relative_base: 0
        }
    }
    
    fn read_next_operation<'input, 'output>(
        &mut self, 
        input: &'input mut dyn FnMut() -> i64, 
        output: &'output mut dyn FnMut(i64)
    ) -> Option<Operation<'input, 'output>> {
        let opcode_parts = OpcodeParts::from_opcode(self.memory[self.pc] as u32);
        match opcode_parts.code {
            1 => Some(Operation::Add(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2]),
                    Parameter::new(opcode_parts.param_3_mode, self.memory[self.pc+3])
                 )),
            2 => Some(Operation::Mult(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2]),
                    Parameter::new(opcode_parts.param_3_mode, self.memory[self.pc+3])
                 )),
            3 => Some(Operation::In(
                    input,
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1])
                 )),
            4 => Some(Operation::Out(
                    output,
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1])
                 )),
            5 => Some(Operation::JumpIfTrue(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2])
                 )),
            6 => Some(Operation::JumpIfFalse(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2])
                 )),
            7 => Some(Operation::LessThan(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2]),
                    Parameter::new(opcode_parts.param_3_mode, self.memory[self.pc+3])
                 )),
            8 => Some(Operation::Equal(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2]),
                    Parameter::new(opcode_parts.param_3_mode, self.memory[self.pc+3])
                 )),
            9 => Some(Operation::RelativeBaseChange(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1])
                 )),
            _ => None
        }
    }
    
    /// Runs the program only until the next Output instruction or until the
    /// program has terminated.
    /// Returns whether or not the program has terminated.
    fn run_to_next_checkpoint(&mut self, input: &mut impl FnMut() -> i64, output: &mut impl FnMut(i64)) -> CheckpointType {
        while let Some(mut operation) = self.read_next_operation(input, output) {
            let is_output = if let Operation::Out(..) = &operation { true } else { false };
            if let Some(pc_override) = operation.execute(&mut self.memory, &mut self.relative_base) {
                self.pc = pc_override;
            } else {
                self.pc += (1 + operation.parameter_count()) as usize;
            }
            
            if is_output {
                return CheckpointType::Output
            }
        }
        CheckpointType::Terminated
    }
}

/// Represents a single tile in the game.
#[derive(Debug)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

impl std::convert::From<i64> for TileType {
    fn from(input: i64) -> Self {
        match input {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!("Invalid input")
        }
    }
}

/// Creates the program and runs it with input, printing the
/// output.
fn main() {
    let mut program = Program::new(&INITIAL_MEMORY_LAYOUT, PROGRAM_MEMORY_SIZE);
    
    // Used to track the tiles
    // We require a RefCell because we use it in both the input closure and the output closure
    // for updating the mapping. However, we know we won't be accessing both at the same time
    // and so we are safe to bump off checking to runtime without causing a panic. 
    let tiles = std::cell::RefCell::new(std::collections::HashMap::<(i64, i64), TileType>::new());
    
    // used to track the score
    let mut score = 0;
    
    // used to provide the joystick input to the arcade cabinet.
    let mut input = || {
        // determine where in the tiles mapping the ball and paddle are. Basically
        // we'll want to keep the paddle under the ball. This will let us determine how
        // the joystick needs to move.
        let ball_x = {
            if let Some((coord, _tile)) = tiles.borrow_mut().iter().filter(|(_coord, tile)| if let TileType::Ball = tile { true } else { false }).next() {
                coord.0
            } else {
                -1
            }
        };
        let paddle_x = {
            if let Some((coord, _tile)) = tiles.borrow_mut().iter().filter(|(_coord, tile)| if let TileType::Paddle = tile { true } else { false }).next() {
                coord.0
            } else {
                -1
            }
        };
        
        if paddle_x < ball_x {
            1
        } else if paddle_x > ball_x {
            -1
        } else {
            0
        }
    };
    
    // while we aren't terminated, we need to read the three outputs to get the
    // x and y coordinates along with the tile type.
    loop {   
        // try reading the x coordinate in.
        let mut x = -1;
        if let CheckpointType::Output = program.run_to_next_checkpoint(&mut input, &mut |value:i64| x = value) {
            // read the y coordinate in.
            let mut y = -1;
            program.run_to_next_checkpoint(&mut input, &mut |value:i64| y = value);
            
            // if (x, y) == (-1, 0) then this is a score updating
            // function, otherwise it's a tile updating function.
            if (x == -1) && (y == 0) {
                program.run_to_next_checkpoint(&mut input, &mut |value:i64| score = value);
            } else {
                let mut tile_type = TileType::Empty;
                program.run_to_next_checkpoint(&mut input, &mut |value:i64| tile_type = value.into());
                tiles.borrow_mut().insert((x, y), tile_type);
            }
        } else {
            break;
        }
    }
    
    // see how many "Block" tiles we have on the screen.
    println!("At the end of the game, there are {} blocks on screen and the score is {}", tiles.borrow_mut().values().filter(|value| {
        if let TileType::Block = value { true } else { false }
    }).count(), score);
}
