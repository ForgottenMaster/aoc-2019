// The unadulterated puzzle input (we'll clone when running the program).
const INPUT: &'static [usize] = &[
1,0,0,3,
1,1,2,3,
1,3,4,3,
1,5,0,3,
2,1,10,19,
1,9,19,23,
1,13,23,27,
1,5,27,31,
2,31,6,35,
1,35,5,39,
1,9,39,43,
1,43,5,47,
1,47,5,51,
2,10,51,55,
1,5,55,59,
1,59,5,63,
2,63,9,67,
1,67,5,71,
2,9,71,75,
1,75,5,79,
1,10,79,83,
1,83,10,87,
1,10,87,91,
1,6,91,95,
2,95,6,99,
2,99,9,103,
1,103,6,107,
1,13,107,111,
1,13,111,115,
2,115,9,119,
1,119,6,123,
2,9,123,127,
1,127,5,131,
1,131,5,135,
1,135,5,139,
2,10,139,143,
2,143,10,147,
1,147,5,151,
1,151,2,155,
1,155,13,0,
99,
2,14,0,0
];

// the target output we are looking for.
const TARGET_OUTPUT: usize = 19690720;

// An enum representing the (non-terminating) operations that
// can be encountered in the program input.
enum Operation {
    Add(usize, usize, usize),
    Mult(usize, usize, usize)
}

// Reads the next operation from the input, returns None if the program
// should terminate.
fn read_next_operation(input_codes: &[usize], pc: usize) -> Option<Operation> {
    let opcode = input_codes[pc];
    match opcode {
        1 => Some(Operation::Add(input_codes[pc+1], input_codes[pc+2], input_codes[pc+3])),
        2 => Some(Operation::Mult(input_codes[pc+1], input_codes[pc+2], input_codes[pc+3])),
        _ => None
    }
}

// Updates the program counter based on the operation that was processed.
fn update_pc(pc: usize, operation: &Operation) -> usize {
    pc + match operation {
        &Operation::Add(..) => 4,
        &Operation::Mult(..) => 4
    }
}

// Applies the operation that was read to the input codes.
fn apply_operation(input_codes: &mut [usize], operation: &Operation) {
    match operation {
        &Operation::Add(input1, input2, output) => input_codes[output] = input_codes[input1] + input_codes[input2],
        &Operation::Mult(input1, input2, output) => input_codes[output] = input_codes[input1] * input_codes[input2]
    }
}

// Performs a complete run of the program, but initialising the values at indices
// 1 and 2 to the given values first.
fn run_with_input(noun: usize, verb: usize) -> usize {
    let mut input_codes = INPUT
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    input_codes[1] = noun;
    input_codes[2] = verb;
    let mut pc = 0;
    while let Some(operation) = read_next_operation(&mut input_codes, pc) {
        apply_operation(&mut input_codes, &operation);
        pc = update_pc(pc, &operation);
    }
    input_codes[0]
}

fn main() {
    let (noun, verb) = {
        let mut out_noun = 0;
        let mut out_verb = 0;
        
        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                if run_with_input(noun, verb) == TARGET_OUTPUT {
                    out_noun = noun;
                    out_verb = verb;
                    break 'outer;
                }
            }
        }
        
        (out_noun, out_verb)
    };
    println!("{}", 100 * noun +verb);
}