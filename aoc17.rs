/// The initial state of the memory layout for the program.
const INITIAL_MEMORY_LAYOUT: &'static [i64] = &[
1,330,331,332,109,4374,1101,0,1182,15,1101,1467,0,24,1001,0,0,570,1006,570,36,1001,571,0,0,1001,570,-1,570,1001,24,1,24,1105,1,18,1008,571,0,571,1001,15,1,15,1008,15,1467,570,1006,570,14,21101,58,0,0,1105,1,786,1006,332,62,99,21102,333,1,1,21102,1,73,0,1105,1,579,1101,0,0,572,1102,0,1,573,3,574,101,1,573,573,1007,574,65,570,1005,570,151,107,67,574,570,1005,570,151,1001,574,-64,574,1002,574,-1,574,1001,572,1,572,1007,572,11,570,1006,570,165,101,1182,572,127,1002,574,1,0,3,574,101,1,573,573,1008,574,10,570,1005,570,189,1008,574,44,570,1006,570,158,1106,0,81,21102,340,1,1,1105,1,177,21102,477,1,1,1106,0,177,21101,514,0,1,21101,176,0,0,1105,1,579,99,21102,1,184,0,1105,1,579,4,574,104,10,99,1007,573,22,570,1006,570,165,1002,572,1,1182,21102,375,1,1,21102,211,1,0,1105,1,579,21101,1182,11,1,21102,222,1,0,1106,0,979,21102,1,388,1,21101,233,0,0,1106,0,579,21101,1182,22,1,21102,1,244,0,1106,0,979,21102,1,401,1,21102,255,1,0,1106,0,579,21101,1182,33,1,21101,266,0,0,1105,1,979,21102,1,414,1,21102,1,277,0,1106,0,579,3,575,1008,575,89,570,1008,575,121,575,1,575,570,575,3,574,1008,574,10,570,1006,570,291,104,10,21101,0,1182,1,21102,313,1,0,1106,0,622,1005,575,327,1101,1,0,575,21101,0,327,0,1106,0,786,4,438,99,0,1,1,6,77,97,105,110,58,10,33,10,69,120,112,101,99,116,101,100,32,102,117,110,99,116,105,111,110,32,110,97,109,101,32,98,117,116,32,103,111,116,58,32,0,12,70,117,110,99,116,105,111,110,32,65,58,10,12,70,117,110,99,116,105,111,110,32,66,58,10,12,70,117,110,99,116,105,111,110,32,67,58,10,23,67,111,110,116,105,110,117,111,117,115,32,118,105,100,101,111,32,102,101,101,100,63,10,0,37,10,69,120,112,101,99,116,101,100,32,82,44,32,76,44,32,111,114,32,100,105,115,116,97,110,99,101,32,98,117,116,32,103,111,116,58,32,36,10,69,120,112,101,99,116,101,100,32,99,111,109,109,97,32,111,114,32,110,101,119,108,105,110,101,32,98,117,116,32,103,111,116,58,32,43,10,68,101,102,105,110,105,116,105,111,110,115,32,109,97,121,32,98,101,32,97,116,32,109,111,115,116,32,50,48,32,99,104,97,114,97,99,116,101,114,115,33,10,94,62,118,60,0,1,0,-1,-1,0,1,0,0,0,0,0,0,1,26,26,0,109,4,2101,0,-3,587,20102,1,0,-1,22101,1,-3,-3,21101,0,0,-2,2208,-2,-1,570,1005,570,617,2201,-3,-2,609,4,0,21201,-2,1,-2,1106,0,597,109,-4,2106,0,0,109,5,1201,-4,0,629,21002,0,1,-2,22101,1,-4,-4,21102,1,0,-3,2208,-3,-2,570,1005,570,781,2201,-4,-3,652,21001,0,0,-1,1208,-1,-4,570,1005,570,709,1208,-1,-5,570,1005,570,734,1207,-1,0,570,1005,570,759,1206,-1,774,1001,578,562,684,1,0,576,576,1001,578,566,692,1,0,577,577,21102,1,702,0,1105,1,786,21201,-1,-1,-1,1105,1,676,1001,578,1,578,1008,578,4,570,1006,570,724,1001,578,-4,578,21101,731,0,0,1106,0,786,1106,0,774,1001,578,-1,578,1008,578,-1,570,1006,570,749,1001,578,4,578,21102,756,1,0,1105,1,786,1105,1,774,21202,-1,-11,1,22101,1182,1,1,21102,774,1,0,1106,0,622,21201,-3,1,-3,1105,1,640,109,-5,2106,0,0,109,7,1005,575,802,21002,576,1,-6,21001,577,0,-5,1105,1,814,21101,0,0,-1,21101,0,0,-5,21101,0,0,-6,20208,-6,576,-2,208,-5,577,570,22002,570,-2,-2,21202,-5,51,-3,22201,-6,-3,-3,22101,1467,-3,-3,2101,0,-3,843,1005,0,863,21202,-2,42,-4,22101,46,-4,-4,1206,-2,924,21101,0,1,-1,1105,1,924,1205,-2,873,21102,1,35,-4,1105,1,924,2102,1,-3,878,1008,0,1,570,1006,570,916,1001,374,1,374,1201,-3,0,895,1101,0,2,0,2101,0,-3,902,1001,438,0,438,2202,-6,-5,570,1,570,374,570,1,570,438,438,1001,578,558,922,20101,0,0,-4,1006,575,959,204,-4,22101,1,-6,-6,1208,-6,51,570,1006,570,814,104,10,22101,1,-5,-5,1208,-5,57,570,1006,570,810,104,10,1206,-1,974,99,1206,-1,974,1101,1,0,575,21101,973,0,0,1106,0,786,99,109,-7,2105,1,0,109,6,21101,0,0,-4,21101,0,0,-3,203,-2,22101,1,-3,-3,21208,-2,82,-1,1205,-1,1030,21208,-2,76,-1,1205,-1,1037,21207,-2,48,-1,1205,-1,1124,22107,57,-2,-1,1205,-1,1124,21201,-2,-48,-2,1106,0,1041,21101,-4,0,-2,1105,1,1041,21101,0,-5,-2,21201,-4,1,-4,21207,-4,11,-1,1206,-1,1138,2201,-5,-4,1059,2101,0,-2,0,203,-2,22101,1,-3,-3,21207,-2,48,-1,1205,-1,1107,22107,57,-2,-1,1205,-1,1107,21201,-2,-48,-2,2201,-5,-4,1090,20102,10,0,-1,22201,-2,-1,-2,2201,-5,-4,1103,1202,-2,1,0,1106,0,1060,21208,-2,10,-1,1205,-1,1162,21208,-2,44,-1,1206,-1,1131,1105,1,989,21101,439,0,1,1105,1,1150,21102,1,477,1,1106,0,1150,21102,1,514,1,21102,1149,1,0,1106,0,579,99,21101,0,1157,0,1105,1,579,204,-2,104,10,99,21207,-3,22,-1,1206,-1,1138,1202,-5,1,1176,2102,1,-4,0,109,-6,2105,1,0,4,9,42,1,7,1,42,1,7,1,42,1,7,1,42,1,7,1,42,1,7,1,42,11,48,1,1,1,48,1,1,1,48,1,1,1,48,7,46,1,3,1,46,1,3,1,5,11,9,7,14,1,3,1,5,1,9,1,9,1,5,1,14,1,3,1,5,1,9,1,9,1,5,1,14,1,3,1,5,1,9,1,9,1,5,1,10,11,3,1,9,1,9,1,5,1,10,1,3,1,3,1,1,1,3,1,9,1,9,1,5,1,10,1,3,11,9,1,9,1,5,1,10,1,7,1,1,1,13,1,9,1,5,1,6,13,1,1,13,1,3,13,6,1,3,1,9,1,13,1,3,1,5,1,6,11,9,1,11,13,6,1,5,1,13,1,11,1,1,1,3,1,12,1,5,1,13,1,11,1,1,7,10,1,5,1,13,1,11,1,5,1,1,1,10,1,5,1,13,1,5,13,1,1,10,1,5,1,13,1,11,1,7,1,10,1,5,1,13,13,7,1,10,1,5,1,33,1,10,7,33,1,50,1,50,1,50,1,50,1,50,1,38,13,38,1,50,1,50,1,50,1,50,1,50,1,50,1,50,1,50,1,50,7,50,1,50,1,50,1,50,1,50,1,50,1,50,1,50,1,50,1,50,1,16
];

/// The total size of the computer memory (program loaded at the beginning).
const PROGRAM_MEMORY_SIZE: u32 = 1_000_000; // 1MB

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Represents the type of the checkpoint reached when using the
/// "run_until_next_checkpoint" function.
enum CheckpointType {
    Output,
    Terminated
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum SpaceType {
    Empty,
    Scaffold,
    Robot(Direction),
    RobotTumbling
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn all() -> impl Iterator<Item=Direction> {
        std::iter::once(Direction::North)
        .chain(std::iter::once(Direction::South))
        .chain(std::iter::once(Direction::West))
        .chain(std::iter::once(Direction::East))
    }
    
    fn turn_direction_towards(&self, target: &Direction) -> Option<TurnDirection> {
        if let Direction::North = self {
            if let Direction::West = target {
                Some(TurnDirection::Left)
            } else if let Direction::East = target {
                Some(TurnDirection::Right)
            } else {
                None
            }
        } else if let Direction::South = self {
            if let Direction::West = target {
                Some(TurnDirection::Right)
            } else if let Direction::East = target {
                Some(TurnDirection::Left)
            } else {
                None
            }
        } else if let Direction::West = self {
            if let Direction::North = target {
                Some(TurnDirection::Right)
            } else if let Direction::South = target {
                Some(TurnDirection::Left)
            } else {
                None
            }
        } else if let Direction::East = self {
            if let Direction::North = target {
                Some(TurnDirection::Left)
            } else if let Direction::South = target {
                Some(TurnDirection::Right)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i64,
    y: i64
}

impl Coordinate {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    
    fn map(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::West => Self::new(self.x - 1, self.y),
            Direction::East => Self::new(self.x + 1, self.y)
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct Instruction {
    turn_direction: TurnDirection,
    move_amount: u32
}

impl Instruction {
    fn new(turn_direction: TurnDirection, move_amount: u32) -> Self {
        Self {
            turn_direction,
            move_amount
        }
    }
}

impl std::string::ToString for Instruction {
    fn to_string(&self) -> String {
        format!("{},{}", self.turn_direction, self.move_amount)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
enum TurnDirection {
    Left,
    Right
}

impl TurnDirection {
    fn apply_to(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::North => match self {
                TurnDirection::Left => Direction::West,
                TurnDirection::Right => Direction::East
            },
            Direction::South => match self {
                TurnDirection::Left => Direction::East,
                TurnDirection::Right => Direction::West
            },
            Direction::West => match self {
                TurnDirection::Left => Direction::South,
                TurnDirection::Right => Direction::North
            },
            Direction::East => match self {
                TurnDirection::Left => Direction::North,
                TurnDirection::Right => Direction::South
            }
        }
    }
    
    fn to_ascii_code(&self) -> u8 {
        match self {
            TurnDirection::Left => 'L' as u8,
            TurnDirection::Right => 'R' as u8
        }
    }
}

impl std::fmt::Display for TurnDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            TurnDirection::Left => "L",
            TurnDirection::Right => "R"
        })
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

trait ScaffoldHelpers {
    fn is_scaffold(&self, coordinate: &Coordinate) -> bool;
    fn is_intersection(&self, coordinate: &Coordinate) -> bool;
    fn determine_instruction(&self, previous: &std::collections::HashSet<Coordinate>, coordinate: &Coordinate, direction: &Direction) -> Option<(Instruction, Coordinate)>;
    fn determine_robot_coordinate_and_direction(&self) -> (Coordinate, Direction);
    fn determine_instruction_sequence(&self) -> Box<[Instruction]>;
    fn determine_travel_amount(&self, coordinate: &Coordinate, direction: &Direction) -> (u32, Coordinate);
}

impl ScaffoldHelpers for std::collections::HashMap<Coordinate, SpaceType> {
    fn is_scaffold(&self, coordinate: &Coordinate) -> bool {
        if let Some(space_type) = self.get(coordinate) {
            match space_type {
                SpaceType::Scaffold => true,
                SpaceType::Robot(..) => true,
                _ => false
            }
        } else {
            false
        }
    }

    fn is_intersection(&self, coordinate: &Coordinate) -> bool {
        if !self.is_scaffold(coordinate) {
            false
        } else if !self.is_scaffold(&coordinate.map(&Direction::North)) {
            false
        } else if !self.is_scaffold(&coordinate.map(&Direction::South)) {
            false
        } else if !self.is_scaffold(&coordinate.map(&Direction::West)) {
            false
        } else if !self.is_scaffold(&coordinate.map(&Direction::East)) {
            false
        } else {
            true
        }
    }
    
    fn determine_robot_coordinate_and_direction(&self) -> (Coordinate, Direction) {
        self
            .iter()
            .filter(|(key, value)| {
                match value {
                    SpaceType::Robot(_) => true,
                    _ => false
                }
            })
            .map(|(key, value)| {
                match value {
                    SpaceType::Robot(direction) => ((*key).clone(), (*direction).clone()),
                    _ => panic!("Impossible")
                }
            })
            .next()
            .unwrap()
    }
    
    fn determine_travel_amount(&self, coordinate: &Coordinate, direction: &Direction) -> (u32, Coordinate) {
        let mut count = 0;
        let mut end_coordinate = coordinate.clone();
        loop {
            let new_coordinate = end_coordinate.map(direction);
            if self.is_scaffold(&new_coordinate) {
                end_coordinate = new_coordinate;
                count += 1;
            } else {
                break;
            }
        }
        (count, end_coordinate)
    }
    
    fn determine_instruction(&self, previous: &std::collections::HashSet<Coordinate>, coordinate: &Coordinate, direction: &Direction) -> Option<(Instruction, Coordinate)> {
        let left_turn_direction = TurnDirection::Left.apply_to(direction);
        let right_turn_direction = TurnDirection::Right.apply_to(direction);
        let left_turn_coordinate = coordinate.map(&left_turn_direction);
        let right_turn_coordinate = coordinate.map(&right_turn_direction);
        
        if self.is_scaffold(&left_turn_coordinate) && !previous.contains(&left_turn_coordinate) {
            let (distance, end_coordinate) = self.determine_travel_amount(coordinate, &left_turn_direction);
            Some((Instruction::new(TurnDirection::Left, distance), end_coordinate))    
        } else if self.is_scaffold(&right_turn_coordinate) && !previous.contains(&right_turn_coordinate) {
            let (distance, end_coordinate) = self.determine_travel_amount(coordinate, &right_turn_direction);
            Some((Instruction::new(TurnDirection::Right, distance), end_coordinate))  
        } else {
            None
        }
    }
    
    fn determine_instruction_sequence(&self) -> Box<[Instruction]> {
        // determine the initial location and heading direction.
        let (mut coordinate, mut direction) = self.determine_robot_coordinate_and_direction();
        let mut previous = std::collections::HashSet::new();
        
        // the instruction list.
        let mut instructions = vec![];
        
        // keep getting instructions while there are more.
        while let Some((instruction, end_coordinate)) = self.determine_instruction(&previous, &coordinate, &direction) {
            previous.insert(end_coordinate.clone());
            instructions.push(instruction.clone());
            coordinate = end_coordinate;
            direction = instruction.turn_direction.apply_to(&direction);
        }
        
        // return the instruction listing.
        instructions.into_boxed_slice()
    }
} 

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Converts the given sequence of instructions into its ASCII representation.
fn to_ascii_seq(input: &[Instruction]) -> Box<[u8]> {
    let mut output = Vec::new();
    for i in 0..input.len() {
        let instruction = &input[i];
        output.push(instruction.turn_direction.to_ascii_code());
        output.push(',' as u8);
        output.extend(instruction.move_amount.to_string().chars().into_iter().map(|c| c as u8));
        if i < input.len() - 1 {
            output.push(',' as u8);
        }
    }
    output.into_boxed_slice()
}

// applies the given function set (triple of functions) to the given full
// sequence and returns the ascii output sequence if the given function set
// can be substituted wholly into the full sequence with no leftovers.
fn apply_function_set(set: &(Option<Box<[u8]>>, Option<Box<[u8]>>, Option<Box<[u8]>>), mut full: &[u8]) -> Option<Box<[u8]>> {
    let mut function_calls = Vec::<u8>::with_capacity(20);
    while full.len() > 0 {
        if set.0.is_some() && full.starts_with(set.0.as_ref().unwrap()) {
            full = &full[set.0.as_ref().unwrap().len()..];
            function_calls.push('A' as u8);
        } else if set.1.is_some() && full.starts_with(set.1.as_ref().unwrap()) {
            full = &full[set.1.as_ref().unwrap().len()..];
            function_calls.push('B' as u8);
        } else if set.2.is_some() && full.starts_with(set.2.as_ref().unwrap()) {
            full = &full[set.2.as_ref().unwrap().len()..];
            function_calls.push('C' as u8);
        } else {
            return None;
        }
        
        if full.len() > 0 {
            full = &full[1..];
            function_calls.push(',' as u8);
        }
    }
    
    Some(function_calls.into_boxed_slice())
}

/// Used by the below function to insert the codes from a given sequence into
/// a string.
fn add_instructions(string: &mut String, func: &[u8]) {
    for code in func {
        if *code != '\n' as u8 {
            string.push(*code as char);
        }
    }
}

/// Used for debugging purposes, reconstructs the full string sequence
/// given the main movement routine and function routines.
fn build_instructions(routine: &[u8], func1: &[u8], func2: &[u8], func3: &[u8]) -> String {
    let mut string = String::new();
    for routine_code in routine {
        if *routine_code == 'A' as u8 {
            add_instructions(&mut string, &func1);
        } else if *routine_code == 'B' as u8 {
            add_instructions(&mut string, &func2);
        } else if *routine_code == 'C' as u8 {
            add_instructions(&mut string, &func3);
        } else if *routine_code == ',' as u8 {
            string.push(',');
        }
    }
    string
}

/// Creates the program and runs it with input, printing the
/// output.
fn main() {
    // part 1
    let coords = {
        let coords = {
            let mut y = 0;
            let mut x = 0;
            let mut coords = std::collections::HashMap::new();
            
            let program = Program::new(&INITIAL_MEMORY_LAYOUT, PROGRAM_MEMORY_SIZE);
            let input = || 0;
            let output = |value: i64| {
                print!("{}", value as u8 as char);
                if let Some(space_type) = match value as u8 as char {
                    '\n' => {
                        y += 1;
                        x = 0;
                        None
                    },
                    '#' => Some(SpaceType::Scaffold),
                    '.' => Some(SpaceType::Empty),
                    '^' => Some(SpaceType::Robot(Direction::North)),
                    'v' => Some(SpaceType::Robot(Direction::South)),
                    '<' => Some(SpaceType::Robot(Direction::West)),
                    '>' => Some(SpaceType::Robot(Direction::East)),
                    'X' => Some(SpaceType::RobotTumbling),
                    _ => panic!("Character {} isn't supported", value)
                } {
                    coords.insert(Coordinate::new(x,y), space_type);
                    x += 1;
                };
            };
            program.run(input, output);
            coords
        };
        
        println!("calibration value: {}", coords.iter().filter(|(key, _)| {
            coords.is_intersection(key)
        }).map(|(Coordinate { x, y }, _)| {
            x * y
        }).sum::<i64>());
        
        coords
    };
    
    // part 2
    {
        const CHARACTER_LIMIT: usize = 20;
    
        // from the coords, we need to determine the path the robot will take
        // with the turns that are needed.
        let instruction_sequence = coords.determine_instruction_sequence();
        let mut all_subsequences = Vec::<Box<[Instruction]>>::new();
        let count = instruction_sequence.len();
        for i in 0..count {
            for j in i..count {
                all_subsequences.push((&instruction_sequence[i..=j]).iter().map(|val| val.clone()).collect::<Vec<_>>().into_boxed_slice());
            }
        }
        
        // filter all possible subsequences to only include those that aren't 
        // more than 20 characters when converted to ASCII.
        let all_subsequences = {
            let mut subsequences = Vec::with_capacity(all_subsequences.len() + 1);
            subsequences.extend(all_subsequences.into_iter().map(|seq| to_ascii_seq(&seq)).filter(|seq| seq.len() <= CHARACTER_LIMIT));
            subsequences.into_boxed_slice()
        };
        
        // determine the possible function sets.
        let function_sets = {
            let mut function_sets = Vec::with_capacity(all_subsequences.len() ^ 3);
            let count = all_subsequences.len();
            for i in 0..count {
                for j in i+1..count {
                    for k in j+1..count {
                        function_sets.push((Some(all_subsequences[i].clone()), Some(all_subsequences[j].clone()), Some(all_subsequences[k].clone())));
                    }
                    function_sets.push((Some(all_subsequences[i].clone()), Some(all_subsequences[j].clone()), None));
                }
                function_sets.push((Some(all_subsequences[i].clone()), None, None))
            }
            function_sets.push((None, None, None));
            function_sets
        };
        
        // grab the ascii sequence for the whole program.
        let full_ascii = to_ascii_seq(&instruction_sequence);
        
        // perform the substitutions of the function set into the ascii sequence.
        let (set, calls) = function_sets.into_iter().map(|set| (set.clone(), apply_function_set(&set, &full_ascii))).filter(|(set, calls)| calls.is_some() && calls.as_ref().unwrap().len() <= CHARACTER_LIMIT).next().unwrap();
        
        // run the program providing the function set and function calls to the program.
        let mut program_memory = INITIAL_MEMORY_LAYOUT.into_iter().cloned().collect::<Vec<_>>().into_boxed_slice();
        program_memory[0] = 2;
        let mut program = Program::new(&program_memory, PROGRAM_MEMORY_SIZE);
        
        // smoosh all the program input into one big massive buffer.
        let input_buffer = {
            let mut input_buffer = Vec::with_capacity(calls.as_ref().unwrap().len() + set.0.as_ref().unwrap().len() + set.1.as_ref().unwrap().len() + set.2.as_ref().unwrap().len() + 6);
            
            // add the main movement routine bytes.
            input_buffer.extend(calls.as_ref().unwrap().iter());
            input_buffer.push('\n' as u8);
            
            // add function a definition.
            input_buffer.extend(set.0.as_ref().unwrap().iter());
            input_buffer.push('\n' as u8);
            
            // add function b definition.
            input_buffer.extend(set.1.as_ref().unwrap().iter());
            input_buffer.push('\n' as u8);
            
            // add function c definition.
            input_buffer.extend(set.2.as_ref().unwrap().iter());
            input_buffer.push('\n' as u8);
            
            // add "n" to signal no real time tracing.
            input_buffer.push('n' as u8);
            input_buffer.push('\n' as u8);
            
            // convert to boxed slice.
            input_buffer.into_boxed_slice()
        };
        
        // make the input function and output functions for the program.
        let mut input_index = 0;
        let mut dust = 0;
        let mut input = || {
            input_index += 1;
            input_buffer[input_index-1] as i64
        };
        let mut output = |i| {
            dust = i;
        };
        
        // finally, run the function and print the accumulated dust.
        program.run(&mut input, &mut output);
        
        println!("dust collected: {}", dust);
    }
}
