/// The initial state of the memory layout for the program.
const INITIAL_MEMORY_LAYOUT: &'static [i32] = &[
3,8,1001,8,10,8,105,1,0,0,21,38,59,84,97,110,191,272,353,434,99999,3,9,1002,9,2,9,101,4,9,9,1002,9,2,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,1002,9,5,9,101,5,9,9,4,9,99,3,9,102,5,9,9,101,5,9,9,1002,9,3,9,101,2,9,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,1002,9,3,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99
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
    In(&'input mut dyn FnMut() -> i32, Parameter),
    Out(&'output mut dyn FnMut(i32), Parameter),
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
    fn execute(&mut self, memory: &mut [i32]) -> Option<usize> {
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
        input: &'input mut dyn FnMut() -> i32, 
        output: &'output mut dyn FnMut(i32)
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
    
    /// Runs the program only until the next Output instruction or until the
    /// program has terminated.
    /// Returns whether or not the program has terminated.
    fn run_to_next_checkpoint(&mut self, input: &mut impl FnMut() -> i32, output: &mut impl FnMut(i32)) -> CheckpointType {
        while let Some(mut operation) = self.read_next_operation(input, output) {
            let is_output = if let Operation::Out(..) = &operation { true } else { false };
            if let Some(pc_override) = operation.execute(&mut self.memory) {
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
    fn run(mut self, mut input: impl FnMut() -> i32, mut output: impl FnMut(i32)) {
        while let CheckpointType::Output = self.run_to_next_checkpoint(&mut input, &mut output) {}
    }
}

/// Represents a single amplifier which is basically a black
/// box that has a program which is progressed when it's told via
/// a previous Amplifier for example. It has a phase which is set
/// at construction time and passed as the first input instruction.
/// Any future input instructions are passed in the run to next checkpoint
/// function....output is stored in a variable too for reading.
struct Amplifier {
    phase: i32,
    sent_phase: bool,
    program: Program,
    output: i32
}

impl Amplifier {
    // Creates a new Amplifier in an initial state for running the program.
    // The phase given is set on first input operation.
    fn new(phase: i32, program: Program) -> Self {
        Self {
            phase,
            sent_phase: false,
            program,
            output: 0
        }
    }
    
    // Runs the Amplifier with the given input until the next checkpoint, returning
    // the next checkpoint (termination/output).
    fn run_to_next_checkpoint(&mut self, input: i32) -> CheckpointType {
        let (mut sent_phase, phase, mut output) = (self.sent_phase, self.phase, self.output);
        let mut input_func = || {
            if sent_phase {
                input
            } else {
                sent_phase = true;
                phase
            }
        };
        let mut output_func = |value| { output = value; };
        let result = self.program.run_to_next_checkpoint(&mut input_func, &mut output_func);
        self.sent_phase = sent_phase;
        self.output = output;
        result
    }
} 

/// A wrapper around a list of amplifiers that provides
/// a black box for adding input in one end and passing it through
/// the amplifiers to get an output.
struct AmplifierList {
    amplifiers: Vec<Amplifier>
}

impl AmplifierList {
    /// Constructs a new AmplifierList from the given vector of
    /// Amplifier instances.
    fn new(amplifiers: Vec<Amplifier>) -> Self {
        Self { amplifiers }
    }

    /// Consumes the whole amplifier list and runs the given input
    /// to completion and provides the final output.
    fn run(mut self, input: i32) -> i32 {
        let mut i = 0;
        let mut output = input;
        loop {
            let result = self.amplifiers[i].run_to_next_checkpoint(output);
            output = self.amplifiers[i].output;
            if i == self.amplifiers.len() - 1 {
                if let CheckpointType::Terminated = result {
                    break; // the last amplifier terminated.
                }
            }
            i = (i + 1) % self.amplifiers.len();
        }
        self.amplifiers[self.amplifiers.len()-1].output
    }
}

/// Attempts the provided combination of phase sequences.
/// This will run the program a number of times equal to the
/// number of phases provided, the input function will provide first
/// the phase sequence and then the previous output. The output will be
/// collected at the end
fn try_combination(phases: &[i32], program: &Program) -> i32 {
    let mut amplifier_list = AmplifierList::new(phases.iter().map(|phase| Amplifier::new(*phase, (*program).clone())).collect());
    amplifier_list.run(0)
}

/// Creates the program and runs it with input, printing the
/// output.
fn main() {
    let program = Program::new(INITIAL_MEMORY_LAYOUT);
    
    // do the testing of all combinations the long way for now. This will
    // and should be replaced by some kind of cartesian product/combination
    // iterator instead.
    let mut max = 0;
    for a in 5..10 {
        for b in (5..10).filter(|val| val != &a) {
            for c in (5..10).filter(|val| (val != &a) && (val != &b)) {
                for d in (5..10).filter(|val| (val != &a) && (val != &b) && (val != &c)) {
                    for e in (5..10).filter(|val| (val != &a) && (val != &b) && (val != &c) && (val != &d)) {
                        max = std::cmp::max(max, try_combination(&vec![a, b, c, d, e], &program));
                    }
                }
            }
        }
    }
    
    // print the result.
    println!("Maximum Thruster Input = {}", max);
}
