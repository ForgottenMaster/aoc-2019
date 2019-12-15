/// The initial state of the memory layout for the program.
const INITIAL_MEMORY_LAYOUT: &'static [i64] = &[
1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,25,0,1016,1102,760,1,1023,1102,1,20,1003,1102,1,22,1015,1102,1,34,1000,1101,0,32,1006,1101,21,0,1017,1102,39,1,1010,1101,30,0,1005,1101,0,1,1021,1101,0,0,1020,1102,1,35,1007,1102,1,23,1014,1102,1,29,1019,1101,767,0,1022,1102,216,1,1025,1102,38,1,1011,1101,778,0,1029,1102,1,31,1009,1101,0,28,1004,1101,33,0,1008,1102,1,444,1027,1102,221,1,1024,1102,1,451,1026,1101,787,0,1028,1101,27,0,1018,1101,0,24,1013,1102,26,1,1012,1101,0,36,1002,1102,37,1,1001,109,28,21101,40,0,-9,1008,1019,41,63,1005,63,205,1001,64,1,64,1105,1,207,4,187,1002,64,2,64,109,-9,2105,1,5,4,213,1106,0,225,1001,64,1,64,1002,64,2,64,109,-9,1206,10,243,4,231,1001,64,1,64,1105,1,243,1002,64,2,64,109,-3,1208,2,31,63,1005,63,261,4,249,1106,0,265,1001,64,1,64,1002,64,2,64,109,5,21108,41,41,0,1005,1012,287,4,271,1001,64,1,64,1105,1,287,1002,64,2,64,109,6,21102,42,1,-5,1008,1013,45,63,1005,63,307,1105,1,313,4,293,1001,64,1,64,1002,64,2,64,109,-9,1201,0,0,63,1008,63,29,63,1005,63,333,1106,0,339,4,319,1001,64,1,64,1002,64,2,64,109,-13,2102,1,4,63,1008,63,34,63,1005,63,361,4,345,1105,1,365,1001,64,1,64,1002,64,2,64,109,5,1201,7,0,63,1008,63,33,63,1005,63,387,4,371,1105,1,391,1001,64,1,64,1002,64,2,64,109,7,1202,1,1,63,1008,63,32,63,1005,63,411,1105,1,417,4,397,1001,64,1,64,1002,64,2,64,109,20,1205,-7,431,4,423,1106,0,435,1001,64,1,64,1002,64,2,64,109,2,2106,0,-3,1001,64,1,64,1105,1,453,4,441,1002,64,2,64,109,-7,21101,43,0,-9,1008,1014,43,63,1005,63,479,4,459,1001,64,1,64,1105,1,479,1002,64,2,64,109,-5,21108,44,43,0,1005,1018,495,1105,1,501,4,485,1001,64,1,64,1002,64,2,64,109,-7,1205,9,517,1001,64,1,64,1105,1,519,4,507,1002,64,2,64,109,11,1206,-1,531,1106,0,537,4,525,1001,64,1,64,1002,64,2,64,109,-15,1208,0,36,63,1005,63,557,1001,64,1,64,1106,0,559,4,543,1002,64,2,64,109,7,2101,0,-7,63,1008,63,35,63,1005,63,581,4,565,1106,0,585,1001,64,1,64,1002,64,2,64,109,-3,21107,45,46,4,1005,1015,607,4,591,1001,64,1,64,1105,1,607,1002,64,2,64,109,-16,2102,1,10,63,1008,63,31,63,1005,63,631,1001,64,1,64,1106,0,633,4,613,1002,64,2,64,109,1,2107,33,10,63,1005,63,649,1106,0,655,4,639,1001,64,1,64,1002,64,2,64,109,17,2101,0,-9,63,1008,63,31,63,1005,63,679,1001,64,1,64,1106,0,681,4,661,1002,64,2,64,109,-6,2107,34,0,63,1005,63,703,4,687,1001,64,1,64,1106,0,703,1002,64,2,64,109,5,1207,-5,34,63,1005,63,719,1105,1,725,4,709,1001,64,1,64,1002,64,2,64,109,-15,1202,6,1,63,1008,63,20,63,1005,63,751,4,731,1001,64,1,64,1105,1,751,1002,64,2,64,109,21,2105,1,5,1001,64,1,64,1106,0,769,4,757,1002,64,2,64,109,5,2106,0,5,4,775,1001,64,1,64,1106,0,787,1002,64,2,64,109,-27,1207,4,35,63,1005,63,809,4,793,1001,64,1,64,1106,0,809,1002,64,2,64,109,13,2108,33,-1,63,1005,63,831,4,815,1001,64,1,64,1106,0,831,1002,64,2,64,109,4,21107,46,45,1,1005,1014,851,1001,64,1,64,1105,1,853,4,837,1002,64,2,64,109,3,21102,47,1,-3,1008,1013,47,63,1005,63,875,4,859,1106,0,879,1001,64,1,64,1002,64,2,64,109,-9,2108,28,2,63,1005,63,895,1106,0,901,4,885,1001,64,1,64,4,64,99,21101,27,0,1,21102,1,915,0,1106,0,922,21201,1,59074,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,942,1,0,1105,1,922,21201,1,0,-1,21201,-2,-3,1,21102,1,957,0,1105,1,922,22201,1,-1,-2,1106,0,968,22102,1,-2,-2,109,-3,2105,1,0
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
    
    /// Consumes the program and returns the output value
    /// if the output instruction was encountered.
    /// Parameter 'input' is a callback which can be called to get the input value.
    /// Parameter 'output' is a callback which can be called with an output value and do something with it.
    fn run(mut self, mut input: impl FnMut() -> i64, mut output: impl FnMut(i64)) {
        while let CheckpointType::Output = self.run_to_next_checkpoint(&mut input, &mut output) {}
    }
}

/// Creates the program and runs it with input, printing the
/// output.
fn main() {
    let program = Program::new(&INITIAL_MEMORY_LAYOUT, PROGRAM_MEMORY_SIZE);
    let input = || 2;
    let output = |value: i64| println!("Distress Signal Coordinates = {}", value);
    program.run(input, output);
}