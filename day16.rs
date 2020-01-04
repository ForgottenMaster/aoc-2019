///////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn make_pattern(element_number: usize, input_count: usize) -> impl Iterator<Item=i32> {
    std::iter::repeat(0).take(element_number)
    .chain(std::iter::repeat(1).take(element_number))
    .chain(std::iter::repeat(0).take(element_number))
    .chain(std::iter::repeat(-1).take(element_number))
    .cycle()
    .skip(1)
    .take(input_count)
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn apply_pattern(pattern: impl Iterator<Item=i32>, input: &[i32]) -> i32 {
    let total = pattern.enumerate().map(|(index, element)| {
        input[index] * element
    }).sum::<i32>();
    total.abs() % 10
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn split_digits(input: &str) -> Box<[i32]> {
    input.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>().into_boxed_slice()
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn run_phase(input: &mut [i32]) {
    let input_len = input.len();
    let mut output = (1..=input_len).map(|i| {
        apply_pattern(make_pattern(i, input_len), input)
    }).collect::<Vec<_>>();
    input.swap_with_slice(&mut output);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn run_fast_phase(input: &mut [i32], offset: usize) {
    let input = &mut input[offset..];
    let mut rolling_total = 0;
    for i in (0..input.len()).rev() {
        rolling_total += input[i];
        input[i] = rolling_total.abs() % 10;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    const INPUT: &'static str = "59715091976660977847686180472178988274868874248912891927881770506416128667679122958792624406231072013221126623881489317912309763385182133601840446469164152094801911846572235367585363091944153574934709408511688568362508877043643569519630950836699246046286262479407806494008328068607275931633094949344281398150800187971317684501113191184838118850287189830872128812188237680673513745269645219228183633986701871488467284716433953663498444829748364402022393727938781357664034739772457855166471802886565257858813291667525635001823584650420815316132943869499800374997777130755842319153463895364409226260937941771665247483191282218355610246363741092810592458";
    
    // part 1
    {
        let mut input = split_digits(INPUT);
        for _ in 0..100 {
            run_phase(&mut input);
        }
        println!("After 100 phases, first 8 digits are: {}", &input[0..8].into_iter().map(|c| c.to_string()).collect::<String>());
    }
    
    // part 2
    // This solution only works when the offset is located in the latter half
    // of the message and relies on the fact that between the end of the message and the
    // halfway point, the pattern applied becomes [..., 0, 0, 0, 0, 1] then [..., 0, 0, 0, 1, 1], etc.
    // as we work our way back through the string. This means it boils down to a simple summation in reverse
    // order from the back of the list.
    {
        let input = INPUT.chars().cycle().take(INPUT.chars().count() * 10_000).collect::<String>();
        let mut input = split_digits(&input);
        let offset = input[0..7].iter().map(|digit| {
            std::char::from_digit(*digit as u32, 10).unwrap()
        }).collect::<String>().parse::<usize>().unwrap();
        
        // for 100 phases, update backwards from the end of the list to the offset point.
        // it will just be summing the previous value in the new list with the next in the input list.
        for _ in 0..100 {
            run_fast_phase(&mut input, offset);
        }
        println!("After 100 phases, offset 8 digits are: {}", &input[offset..offset+8].into_iter().map(|c| c.to_string()).collect::<String>());
    }
}
