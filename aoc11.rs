/// The initial state of the memory layout for the program.
const INITIAL_MEMORY_LAYOUT: &'static [i64] = &[
3,8,1005,8,330,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,29,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,51,1,1103,2,10,1006,0,94,1006,0,11,1,1106,13,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,87,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,109,2,1105,5,10,2,103,16,10,1,1103,12,10,2,105,2,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,146,1006,0,49,2,1,12,10,2,1006,6,10,1,1101,4,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,183,1,6,9,10,1006,0,32,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,213,2,1101,9,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,239,1006,0,47,1006,0,4,2,6,0,10,1006,0,58,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,274,2,1005,14,10,1006,0,17,1,104,20,10,1006,0,28,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,309,101,1,9,9,1007,9,928,10,1005,10,15,99,109,652,104,0,104,1,21101,0,937263411860,1,21102,347,1,0,1105,1,451,21101,932440724376,0,1,21102,1,358,0,1105,1,451,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,29015167015,1,21101,0,405,0,1106,0,451,21102,1,3422723163,1,21101,0,416,0,1106,0,451,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,868389376360,1,21101,0,439,0,1105,1,451,21102,825544712960,1,1,21102,1,450,0,1106,0,451,99,109,2,21201,-1,0,1,21101,0,40,2,21102,482,1,3,21102,1,472,0,1106,0,515,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,477,478,493,4,0,1001,477,1,477,108,4,477,10,1006,10,509,1101,0,0,477,109,-2,2106,0,0,0,109,4,2101,0,-1,514,1207,-3,0,10,1006,10,532,21102,1,0,-3,22101,0,-3,1,22102,1,-2,2,21102,1,1,3,21101,551,0,0,1106,0,556,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,579,2207,-4,-2,10,1006,10,579,22102,1,-4,-4,1106,0,647,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,598,0,1106,0,556,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,617,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,639,21201,-1,0,1,21102,639,1,0,105,1,514,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0
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

/////////////////////////////////////////////////////////////////////////////////////

/// The filepath in which to save the output image.
const OUTPUT_PATH: &'static str = "Output.bmp";

/// Represents the color of one of the tiles that the
/// painting robot has painted.
#[derive(Clone)]
enum Color {
    Black,
    White
}

impl std::convert::From<&i64> for Color {
    fn from(value: &i64) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("Invalid Value")
        }
    }
}

impl std::convert::From<&Color> for i64 {
    fn from(value: &Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1
        }
    }
}

/// Represents the direction of a turn, either clockwise
/// or anti-clockwise.
enum TurnDirection {
    Clockwise,
    AntiClockwise
}

impl std::convert::From<&i64> for TurnDirection {
    fn from(value: &i64) -> Self {
        match value {
            0 => TurnDirection::AntiClockwise,
            1 => TurnDirection::Clockwise,
            _ => panic!("Invalid input")
        }
    }
}

/// The current direction of the robot.
enum Direction {
    Up,
    Down,
    Left,
    Right
}

/// Represents the bot that can move around and paint
/// the grid.
struct PaintingBot {
    location: (i64, i64),
    direction: Direction,
    grid: std::collections::HashMap<(i64, i64), Color>
}

impl Default for PaintingBot {
    fn default() -> Self {
        let location = (0, 0);
        let direction = Direction::Up;
        let grid = std::collections::HashMap::new();
        Self { location, direction, grid }
    }
}

impl PaintingBot {
    fn new(color: &Color) -> Self {
        let mut bot = Self::default();
        match color {
            Color::Black => (),
            Color::White => { bot.grid.insert(bot.location, Color::White); }
        }
        bot
    }

    fn turn(&mut self, direction: &TurnDirection) {
        self.direction = match self.direction {
            Direction::Up => {
                match direction {
                    TurnDirection::Clockwise => Direction::Right,
                    TurnDirection::AntiClockwise => Direction::Left
                }
            },
            Direction::Down => {
                match direction {
                    TurnDirection::Clockwise => Direction::Left,
                    TurnDirection::AntiClockwise => Direction::Right
                }
            },
            Direction::Left => {
                match direction {
                    TurnDirection::Clockwise => Direction::Up,
                    TurnDirection::AntiClockwise => Direction::Down
                }
            },
            Direction::Right => {
                match direction {
                    TurnDirection::Clockwise => Direction::Down,
                    TurnDirection::AntiClockwise => Direction::Up
                }
            }
        }
    }

    fn color(&self) -> Color {
        if let Some(color) = self.grid.get(&self.location) {
            (*color).clone()
        } else {
            Color::Black
        }
    }

    fn paint(&mut self, color: &Color) {
        self.grid.insert(self.location.clone(), (*color).clone());
    }

    fn forward(&mut self) {
        self.location = match self.direction {
            Direction::Up => (self.location.0, self.location.1 - 1),
            Direction::Down => (self.location.0, self.location.1 + 1),
            Direction::Left => (self.location.0 - 1, self.location.1),
            Direction::Right => (self.location.0 + 1, self.location.1)
        }
    }

    fn save_grid(&self, path: impl AsRef<std::path::Path>) {
        // find the bounds of the grid.
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (std::i64::MAX, std::i64::MAX, std::i64::MIN, std::i64::MIN);
        for (x, y) in self.grid.keys() {
            min_x = std::cmp::min(min_x, *x);
            min_y = std::cmp::min(min_y, *y);
            max_x = std::cmp::max(max_x, *x);
            max_y = std::cmp::max(max_y, *y);
        }    

        // calculate width and height.
        let width = (max_x - min_x) + 1;
        let height = (max_y - min_y) + 1;

        // allocate a buffer.
        let mut data: Vec<u8> = Vec::with_capacity((width * height) as usize);

        // iterate the rows and columns
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(color) = self.grid.get(&(x, y)) {
                    let val: i64 = color.into();
                    data.push(val as u8 * 255);
                } else {
                    let val: i64 = (&Color::Black).into();
                    data.push(val as u8 * 255);
                }
            }
        }

        // write to the file.
        image::save_buffer(path, &data, width as u32, height as u32, image::ColorType::Gray(8)).unwrap();
    }
}

fn run_program(color: &Color) -> PaintingBot {
    let mut program = Program::new(&INITIAL_MEMORY_LAYOUT, PROGRAM_MEMORY_SIZE);
    let mut robot = PaintingBot::new(color);

    loop {
        let color = robot.color();
        let mut input = move || (&color).into();

        // paint the panel.
        let mut output = |value: i64| {
            robot.paint(&(&value).into());
        };
        if let CheckpointType::Terminated = program.run_to_next_checkpoint(&mut input, &mut output) {
            break;
        }

        // turn and move.
        let mut output = |value: i64| {
            robot.turn(&(&value).into());
            robot.forward();
        };
        program.run_to_next_checkpoint(&mut input, &mut output);
    }

    robot
}

/// Creates the program and runs it with input, printing the
/// output.
fn main() {
    // part 1
    println!("Number painted tiles: {}", run_program(&Color::Black).grid.len());

    // part 2
    run_program(&Color::White).save_grid(OUTPUT_PATH);
    println!("Written output to file: {}", OUTPUT_PATH);
}