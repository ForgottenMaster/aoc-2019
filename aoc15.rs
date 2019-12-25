/// The initial state of the memory layout for the program.
const INITIAL_MEMORY_LAYOUT: &'static [i64] = &[
3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1002,1034,1,1039,102,1,1036,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1106,0,124,1002,1034,1,1039,101,0,1036,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1105,1,124,1001,1034,-1,1039,1008,1036,0,1041,102,1,1035,1040,1002,1038,1,1043,1002,1037,1,1042,1106,0,124,1001,1034,1,1039,1008,1036,0,1041,1002,1035,1,1040,101,0,1038,1043,1002,1037,1,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,9,1032,1006,1032,165,1008,1040,3,1032,1006,1032,165,1102,2,1,1044,1105,1,224,2,1041,1043,1032,1006,1032,179,1101,1,0,1044,1106,0,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,29,1044,1105,1,224,1101,0,0,1044,1105,1,224,1006,1044,247,102,1,1039,1034,1002,1040,1,1035,1001,1041,0,1036,1002,1043,1,1038,102,1,1042,1037,4,1044,1106,0,0,19,27,41,9,17,87,2,1,91,14,15,99,17,13,40,13,7,33,23,28,7,21,75,15,41,83,18,4,28,1,21,99,3,2,4,60,16,5,16,22,59,18,37,21,62,96,11,63,46,16,27,76,7,36,38,28,53,18,84,52,12,47,25,93,10,57,64,21,41,75,52,9,80,60,21,86,60,21,70,21,13,72,78,22,61,17,28,54,51,93,18,3,87,21,4,98,17,59,2,17,18,71,5,20,16,39,66,18,7,62,15,37,25,52,27,17,15,10,48,11,39,18,20,68,83,22,36,9,3,69,56,64,21,39,93,1,90,18,57,52,14,41,32,57,5,7,72,18,35,66,21,22,88,2,31,52,7,35,25,50,14,35,7,11,92,38,14,66,3,28,84,18,17,48,15,34,40,4,21,92,52,27,5,4,53,65,59,24,88,24,66,88,85,26,8,26,10,64,99,9,44,38,14,26,74,75,24,31,7,6,62,9,57,75,18,22,52,57,15,3,87,21,39,24,12,8,70,8,19,3,89,16,36,15,36,16,30,28,8,89,12,99,98,16,78,24,11,63,87,55,51,19,57,18,28,9,90,15,95,56,57,1,93,77,24,36,14,44,46,25,66,37,23,8,12,10,58,27,66,4,72,1,2,16,91,16,66,26,24,53,25,20,41,8,75,23,2,20,91,19,3,12,32,30,3,33,85,17,21,92,17,1,12,73,9,34,12,85,42,5,69,67,4,87,70,6,49,96,12,5,37,62,54,72,13,52,14,21,84,68,54,22,78,11,93,12,90,55,7,19,44,21,98,4,46,50,27,30,2,99,27,35,8,5,62,1,91,65,12,80,16,17,81,14,73,60,69,24,23,13,74,57,10,26,21,80,60,10,79,3,9,37,77,73,16,10,3,13,95,4,91,65,11,86,16,24,71,22,6,63,90,56,15,64,8,25,46,77,71,24,13,72,96,22,8,15,79,39,19,19,47,14,16,92,69,73,23,76,23,28,60,84,14,54,62,11,8,30,75,44,16,4,30,82,14,80,11,1,70,85,10,14,73,70,9,54,25,26,12,51,23,86,92,18,11,19,74,55,51,10,73,7,13,43,89,5,55,2,18,82,2,14,63,71,28,7,94,61,10,51,8,53,63,22,39,19,79,20,99,2,66,22,7,68,71,17,19,45,10,14,42,99,9,9,13,75,84,14,83,75,19,92,22,47,4,83,18,46,91,22,61,28,6,71,17,10,1,81,6,60,83,21,14,13,71,11,68,73,52,10,25,30,91,6,25,86,89,19,39,18,95,1,52,23,91,20,14,41,91,26,59,16,85,99,4,15,96,51,19,25,51,73,3,48,79,14,14,41,5,17,59,8,51,43,21,15,47,3,28,53,12,22,23,2,94,74,23,53,20,20,98,21,14,46,61,26,6,55,20,69,28,6,41,19,70,48,6,9,32,32,28,20,21,62,22,38,7,90,3,32,24,92,49,23,72,63,17,18,89,85,33,28,23,27,5,42,52,7,54,18,17,21,63,98,8,9,84,31,24,80,70,22,51,28,61,77,6,25,68,66,8,47,22,7,44,26,37,15,28,68,23,18,18,14,34,3,85,99,31,41,53,28,20,43,90,22,13,70,27,27,17,35,48,11,92,4,60,84,4,38,27,25,89,99,74,2,31,63,13,50,1,54,4,59,3,59,2,54,15,37,19,74,45,75,7,84,19,96,72,75,9,34,18,52,23,99,11,45,81,53,7,71,24,80,26,31,11,74,27,57,0,0,21,21,1,10,1,0,0,0,0,0,0
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

/// Represents the direction that the bot can travel in.
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

impl std::convert::From<i64> for Direction {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::North,
            2 => Self::South,
            3 => Self::West,
            4 => Self::East,
            _ => panic!("Invalid value")
        }
    }
}

impl std::convert::From<&Direction> for i64 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4
        }
    }
}

impl Direction {
    fn map_coordinate(&self, input: &(i64, i64)) -> (i64, i64) {
        match self {
            Direction::North => (input.0, input.1 - 1),
            Direction::South => (input.0, input.1 + 1),
            Direction::West => (input.0 - 1, input.1),
            Direction::East => (input.0 + 1, input.1)
        }
    }
    
    fn get_reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West
        }
    }
}

/// Represents the type of the space which can either be
/// Starting, Empty, Wall, or Oxygen.
#[derive(Debug, PartialEq)]
enum SpaceType {
    Starting,
    Empty,
    Wall,
    Oxygen
}

impl SpaceType {
    fn get_color(&self) -> (u8, u8, u8) {
        match self {
            Self::Starting => (255, 0, 0),
            Self::Empty => (255, 255, 255),
            Self::Wall => (0, 0, 0),
            Self::Oxygen => (0, 255, 0)
        }
    }
}

/// Represents the result we get back from the program.
enum OutputType {
    HitWall,
    Moved,
    MovedAndFoundOxygen
}

impl std::convert::From<i64> for OutputType {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::HitWall,
            1 => Self::Moved,
            2 => Self::MovedAndFoundOxygen,
            _ => panic!("Invalid value")
        }
    }
}

/// Represents the robot that can move, tracks the state
/// of the historical coordinates of the bot so we can
/// avoid visiting the same space twice if possible.
struct Robot {
    maze: std::collections::HashMap<(i64, i64), SpaceType>, // the entire maze coordinates and space types.
    stack: Vec<Direction>,                                  // a stack of directions that when executed will return the bot to the start.
    current: (i64, i64)                                     // the current coordinate that the bot is at.
}

impl Default for Robot {
    fn default() -> Self {
        let mut maze = std::collections::HashMap::new();
        maze.insert((0, 0), SpaceType::Starting);
        let stack = vec![];
        let current = (0, 0);
        Self { maze, stack, current }
    }
}

impl Robot {
    /// Recursively scouts the entire maze to find all the walls and
    /// empty spaces.
    fn scout_all(&mut self, mut program: Program) {
        loop {
            // determine if we have popped from the stack.
            let mut popped_off_stack = false;
        
            // determine the direction to move in.
            if let Some(direction) = if !self.maze.contains_key(&Direction::North.map_coordinate(&self.current)) {
                Some(Direction::North)
            } else if !self.maze.contains_key(&Direction::South.map_coordinate(&self.current)) {
                Some(Direction::South)
            } else if !self.maze.contains_key(&Direction::West.map_coordinate(&self.current)) {
                Some(Direction::West)
            } else if !self.maze.contains_key(&Direction::East.map_coordinate(&self.current)) {
                Some(Direction::East)
            } else if let Some(direction) = self.stack.pop() {
                popped_off_stack = true;
                Some(direction)
            } else {
                None
            } {
                // input just provides the direction to move in.
                let mut input = || (&direction).into();
                
                // output provides the result of the move.
                let mut output = |value:i64| {
                    let value: OutputType = value.into();
                    match value {
                        OutputType::HitWall => {
                            self.maze.insert(direction.map_coordinate(&self.current), SpaceType::Wall);
                        },
                        OutputType::Moved => {
                            self.current = direction.map_coordinate(&self.current);
                            self.maze.entry(self.current.clone()).or_insert(SpaceType::Empty);
                            if !popped_off_stack {
                                self.stack.push(direction.get_reverse());
                            }
                        },
                        OutputType::MovedAndFoundOxygen => {
                            self.current = direction.map_coordinate(&self.current);
                            self.maze.entry(self.current.clone()).or_insert(SpaceType::Oxygen);
                            if !popped_off_stack {
                                self.stack.push(direction.get_reverse());
                            }
                        }
                    }
                };
                
                // run the next checkpoint.
                program.run_to_next_checkpoint(&mut input, &mut output);
            } else {
                break; // finished, nowhere to go.
            }
        }
    }
    
    fn save_map(&self, path: impl AsRef<std::path::Path>, shortest_path: &Box<[(i64, i64)]>) {
        // determine the min_x, max_x, min_y, max_y values.
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (
            std::i64::MAX,
            std::i64::MIN,
            std::i64::MAX,
            std::i64::MIN
        );
        
        // iterate the map coordinates.
        for &(x, y) in self.maze.keys() {
            min_x = std::cmp::min(min_x, x);
            max_x = std::cmp::max(max_x, x);
            min_y = std::cmp::min(min_y, y);
            max_y = std::cmp::max(max_y, y);
        }
        
        // determine the width and height of the maze.
        let (width, height) = (max_x - min_x + 1, max_y - min_y + 1);
        
        // determine the storage required for the pixels.
        let mut buffer = Vec::with_capacity((width * height * 3) as usize);
        
        // fill the buffer.
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let (mut r, mut g, mut b) = if let Some(space_type) = self.maze.get(&(x, y)) {
                    space_type.get_color()
                } else {
                    SpaceType::Wall.get_color()
                };
                
                if shortest_path.contains(&(x, y)) {
                    r = 0;
                    g = 255;
                    b = 0;
                }
                
                buffer.push(r);
                buffer.push(g);
                buffer.push(b);
            }
        }
        
        // save to file.
        image::save_buffer(
            path,
            &buffer,
            width as u32,
            height as u32,
            image::ColorType::RGB(8)
        ).unwrap();
    }
    
    fn get_start_coords(&self) -> (i64, i64) {
        *(self.maze.iter().filter(|(key, value)| {
            if let SpaceType::Starting = value {
                true
            } else {
                false
            }
        }).next().unwrap().0)
    }
    
    fn get_end_coords(&self) -> (i64, i64) {
        *(self.maze.iter().filter(|(key, value)| {
            if let SpaceType::Oxygen = value {
                true
            } else {
                false
            }
        }).next().unwrap().0)
    }
    
    fn get_valid_neighbours<'a>(&'a self, coordinate: &(i64, i64)) -> impl Iterator<Item=(i64, i64)> + 'a {
        std::iter::once(Direction::North.map_coordinate(coordinate))
            .chain(std::iter::once(Direction::South.map_coordinate(coordinate)))
            .chain(std::iter::once(Direction::West.map_coordinate(coordinate)))
            .chain(std::iter::once(Direction::East.map_coordinate(coordinate)))
            .filter(move |coord| {
                if let Some(space_type) = self.maze.get(coord) {
                    if let SpaceType::Wall = space_type {
                        false
                    } else {
                        true
                    }
                } else { 
                    false
                }
            })
    }
    
    fn find_shortest_path(&self) -> Option<Box<[(i64, i64)]>> {    
        // find the start and end coordinates.
        let (start, end) = (self.get_start_coords(), self.get_end_coords());
        
        // the heuristic function (as an estimate, use the
        // manhattan distance from this coord to the goal).
        let h = |&(x, y):&(i64, i64)| -> i64 {
            (start.0 - x).abs() + (start.1 - y).abs()
        };
        
        // create the open set for nodes to consider
        let mut open_set = vec![start];
        
        // mapping for tracking which nodes came from which
        let mut came_from = std::collections::HashMap::<(i64, i64), (i64, i64)>::new();
        
        // grab the min/max x/y coordinates so we can iterate.
        let (min_x, max_x, min_y, max_y) = {
            let (mut min_x, mut max_x, mut min_y, mut max_y) = (
                std::i64::MAX,
                std::i64::MIN,
                std::i64::MAX,
                std::i64::MIN
            );
            
            for &(x, y) in self.maze.keys() {
                min_x = std::cmp::min(min_x, x);
                max_x = std::cmp::max(max_x, x);
                min_y = std::cmp::min(min_y, y);
                max_y = std::cmp::max(max_y, y);
            }
            
            (min_x, max_x, min_y, max_y)
        };
        
        // initialize a g_score mapping.
        let mut g_score = (min_x..=max_x)
            .flat_map(|x| {
                (min_y..=max_y)
                    .map(move |y| {
                        (x, y)
                    })
            })
            .map(|coord| (coord, std::i64::MAX))
            .collect::<std::collections::HashMap<_, _>>();
        *g_score.get_mut(&start).unwrap() = 0;
        
        // initialise an f_score mapping.
        let mut f_score = g_score.clone();
        *f_score.get_mut(&start).unwrap() = h(&start);
        
        // while there are more in the open set.
        while !open_set.is_empty() {
            // select the open node with the lowest f score.
            let current = {
                let mut scores = open_set.iter().map(|coord| {
                    (coord, *f_score.get(coord).unwrap())
                }).collect::<Vec<_>>();
                scores.sort_by(|pair_1, pair_2| pair_1.1.partial_cmp(&pair_2.1).unwrap());
                scores.into_iter().map(|pair| pair.0).next().unwrap().clone()
            };
            
            // if current is end, then we're done and can reconstruct the
            // path.
            if current == end {
                return Some(reconstruct_path(came_from, current).into_boxed_slice());
            }
            
            // remove the current from the open set.
            open_set.retain(|item| item != &current);
            
            // iterate the neighbouring coordinates of the
            // current coordinate.
            for neighbour in self.get_valid_neighbours(&current) {
                let tentative_g_score = *g_score.get(&current).unwrap();
                if tentative_g_score < *g_score.get(&neighbour).unwrap() {
                    // update the path to the neigbour
                    *came_from.entry(neighbour).or_insert((0, 0)) = current;
                    *g_score.get_mut(&neighbour).unwrap() = tentative_g_score;
                    *f_score.get_mut(&neighbour).unwrap() = *g_score.get(&neighbour).unwrap() + h(&neighbour);
                    if !open_set.contains(&neighbour) {
                        open_set.push(neighbour);
                    }
                }
            }
        }
        
        // failed to find a path.
        None
    }
    
    fn contains_non_oxygenated_spaces(&self) -> bool {
        self.maze.values().filter(|space_type| {
            (**space_type == SpaceType::Empty) || (**space_type == SpaceType::Starting)
        }).count() > 0
    }
    
    fn spread_oxygen(&mut self) {
        let oxygenated_coords: Vec<(i64, i64)> = self.maze.iter().filter(|(key, value)| {
            **value == SpaceType::Oxygen
        }).map(|(key, value)| {
            (*key).clone()
        }).collect();
        
        let empty_neighbours: std::collections::HashSet<(i64, i64)> = oxygenated_coords.into_iter().flat_map(|coord| {
            std::iter::once(Direction::North.map_coordinate(&coord))
            .chain(std::iter::once(Direction::South.map_coordinate(&coord)))
            .chain(std::iter::once(Direction::West.map_coordinate(&coord)))
            .chain(std::iter::once(Direction::East.map_coordinate(&coord)))
            .filter(|coord| {
                let space_type = self.maze.get(coord).unwrap();
                (*space_type == SpaceType::Empty) || (*space_type == SpaceType::Starting)
            })    
        }).collect();
        
        // set the coords as Oxygenated.
        for coord in empty_neighbours {
            *self.maze.get_mut(&coord).unwrap() = SpaceType::Oxygen;
        }
    }
}

/// Takes a mapping of coordinate to previous coordinate and
/// reconstructs the entire path that the algorithm has found.
fn reconstruct_path(came_from: std::collections::HashMap<(i64, i64), (i64, i64)>, mut current: (i64, i64)) -> Vec<(i64, i64)> {
    let mut total_path = Vec::with_capacity(came_from.len());
    while let Some(previous) = came_from.get(&current) {
        current = *previous;
        total_path.insert(0, current);
    }
    total_path
}

/// Creates the program and runs it with input, printing the
/// output.
fn main() {
    // scout the map/maze to find all the walls and the
    // start/end points we want to pathfind to.
    let mut robot = {
        let program = Program::new(&INITIAL_MEMORY_LAYOUT, PROGRAM_MEMORY_SIZE);
        let mut robot = Robot::default();
        robot.scout_all(program);
        robot
    };
 
    // Part 1 - Shortest Path
    println!("Shortest Path = {}", robot.find_shortest_path().unwrap().len());
    
    // Part 2 - Flooding
    let mut minutes = 0;
    while robot.contains_non_oxygenated_spaces() {
        minutes += 1;
        robot.spread_oxygen();
    }
    println!("Oxygenation Time = {} minutes", minutes);
}
