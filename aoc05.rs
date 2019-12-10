/// The value of the program input.
const INPUT: i32 = 5;

/// The initial state of the memory layout for the program.
const INITIAL_MEMORY_LAYOUT: &'static [i32] = &[
3,225,1,225,6,6,1100,1,238,225,104,0,1102,68,5,225,1101,71,12,225,1,117,166,224,1001,224,-100,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1001,66,36,224,101,-87,224,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1101,26,51,225,1102,11,61,224,1001,224,-671,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,59,77,224,101,-136,224,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,1101,11,36,225,1102,31,16,225,102,24,217,224,1001,224,-1656,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,101,60,169,224,1001,224,-147,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,38,69,225,1101,87,42,225,2,17,14,224,101,-355,224,224,4,224,102,8,223,223,1001,224,2,224,1,224,223,223,1002,113,89,224,101,-979,224,224,4,224,1002,223,8,223,1001,224,7,224,1,224,223,223,1102,69,59,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,677,224,1002,223,2,223,1006,224,329,1001,223,1,223,1007,226,226,224,1002,223,2,223,1006,224,344,1001,223,1,223,1108,226,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,374,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,419,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,434,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,449,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,464,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,479,101,1,223,223,1007,226,677,224,102,2,223,223,1006,224,494,101,1,223,223,107,677,677,224,102,2,223,223,1005,224,509,101,1,223,223,108,677,677,224,102,2,223,223,1006,224,524,1001,223,1,223,8,226,677,224,102,2,223,223,1005,224,539,101,1,223,223,107,677,226,224,102,2,223,223,1005,224,554,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,569,1001,223,1,223,7,677,226,224,1002,223,2,223,1005,224,584,1001,223,1,223,1108,226,226,224,102,2,223,223,1005,224,599,1001,223,1,223,1107,677,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,629,1001,223,1,223,108,226,226,224,102,2,223,223,1005,224,644,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,659,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226
];

/// Represents a parameter mode defining how a parameter to an operation will be used.
#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

impl std::convert::From<u8> for ParameterMode {
    /// Converts from a u8 (which should be either 0 or 1) to the appropriate
    /// parameter mode.
    fn from(input: u8) -> Self {
        match input {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("Parameter mode is not valid")
        }
    }
}

/// Represents a single parameter in an operation. A parameter will contain a mode
/// along with a value.
#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32
}

impl Parameter {
    /// Creates a new Parameter with the given values
    fn new(mode: ParameterMode, value: i32) -> Self {
        Self {
            mode,
            value
        }
    }
    
    /// Resolves the Parameter into a number, handling the parameter
    /// mode correctly
    fn resolve(&self, memory: &mut [i32]) -> i32 {
        match self.mode {
            ParameterMode::Position => memory[self.value as usize],
            ParameterMode::Immediate => self.value
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
    In(&'input dyn Fn() -> i32, Parameter),
    Out(&'output dyn Fn(i32), Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equal(Parameter, Parameter, Parameter)
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
            Operation::Equal(..) => 3
        }
    }
    
    /// Applies the operation to the given memory block.
    /// Returns a program counter override if applicable.
    fn execute(&self, memory: &mut [i32]) -> Option<usize> {
        match self {
            Operation::Add(input_1, input_2, output) => {
                memory[output.resolve(memory) as usize] = input_1.resolve(memory) + input_2.resolve(memory);
                None
            },
            Operation::Mult(input_1, input_2, output) => {
                memory[output.resolve(memory) as usize] = input_1.resolve(memory) * input_2.resolve(memory);
                None
            },
            Operation::In(input, output) => {
                memory[output.resolve(memory) as usize] = input();
                None
            },
            Operation::Out(output, input) => {
                output(input.resolve(memory));
                None
            },
            Operation::JumpIfTrue(input_1, input_2) => {
                if input_1.resolve(memory) != 0 {
                    Some(input_2.resolve(memory) as usize)
                } else {
                    None
                }
            },
            Operation::JumpIfFalse(input_1, input_2) => {
                if input_1.resolve(memory) == 0 {
                    Some(input_2.resolve(memory) as usize)
                } else {
                    None
                }
            },
            Operation::LessThan(input_1, input_2, output) => {
                memory[output.resolve(memory) as usize] = if input_1.resolve(memory) < input_2.resolve(memory) { 1 } else { 0 };
                None
            },
            Operation::Equal(input_1, input_2, output) => {
                memory[output.resolve(memory) as usize] = if input_1.resolve(memory) == input_2.resolve(memory) { 1 } else { 0 };
                None
            }
        }
    }
}

/// Represents a program with some memory containing both the code and data for the program
/// and the program counter.
struct Program {
    memory: Box<[i32]>,
    pc: usize
}

impl Program {
    /// Creates a new Program with the given initial memory layout.
    /// Initialises the program counter to 0.
    fn new(memory: &[i32]) -> Self {
        let memory = memory.to_vec().into_boxed_slice();
        let pc = 0;
        
        Self {
            memory,
            pc
        }
    }
    
    fn read_next_operation<'input, 'output>(
        &mut self, 
        input: &'input dyn Fn() -> i32, 
        output: &'output dyn Fn(i32)
    ) -> Option<Operation<'input, 'output>> {
        let opcode_parts = OpcodeParts::from_opcode(self.memory[self.pc] as u32);
        match opcode_parts.code {
            1 => Some(Operation::Add(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2]),
                    Parameter::new(ParameterMode::Immediate, self.memory[self.pc+3])
                 )),
            2 => Some(Operation::Mult(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2]),
                    Parameter::new(ParameterMode::Immediate, self.memory[self.pc+3])
                 )),
            3 => Some(Operation::In(
                    input,
                    Parameter::new(ParameterMode::Immediate, self.memory[self.pc+1])
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
                    Parameter::new(ParameterMode::Immediate, self.memory[self.pc+3])
                 )),
            8 => Some(Operation::Equal(
                    Parameter::new(opcode_parts.param_1_mode, self.memory[self.pc+1]),
                    Parameter::new(opcode_parts.param_2_mode, self.memory[self.pc+2]),
                    Parameter::new(ParameterMode::Immediate, self.memory[self.pc+3])
                 )),
            _ => None
        }
    }
    
    /// Consumes the program and returns the output value
    /// if the output instruction was encountered.
    /// Parameter 'input' is a callback which can be called to get the input value.
    /// Parameter 'output' is a callback which can be called with an output value and do something with it.
    fn run(mut self, input: impl Fn() -> i32, output: impl Fn(i32)) {
        while let Some(operation) = self.read_next_operation(&input, &output) {
            if let Some(pc_override) = operation.execute(&mut self.memory) {
                self.pc = pc_override;
            } else {
                self.pc += (1 + operation.parameter_count()) as usize;
            }
        }
    }
}

/// Creates the program and runs it with input, printing the
/// output.
fn main() {
    let input_func = || INPUT;
    let output_func = |output: i32| println!("outputting {}", output);
    Program::new(INITIAL_MEMORY_LAYOUT).run(input_func, output_func);
}
