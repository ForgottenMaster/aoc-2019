// The range to search in for the passwords
const LOWER_BOUND: u32 = 108457;
const UPPER_BOUND: u32 = 562041;

/// Represents a single grouping of the same digit that appear together
/// in a block
struct Segment {
    digit: u8,
    count: u32
}

impl Segment {
    /// Creates a new Segment from a given digit and number of
    /// repetitions
    fn new(digit: u8, count: u32) -> Self {
        Self { digit, count }
    }
}

/// Takes a u32 and breaks it into a vector of its digits
fn break_into_digits(num: u32) -> Vec<u8> {
    num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect()
}

/// Takes a u32 and breaks it into a vector of segments which
/// are the groupings of the same digit
fn break_into_segments(num: u32) -> Vec<Segment> {
    let digits = break_into_digits(num);
    let mut segments = Vec::with_capacity(digits.len());
    let mut digit = digits[0];
    let mut count = 1;
    
    for i in 1..digits.len() {
        if digits[i] != digit {
            segments.push(Segment::new(digit, count));
            digit = digits[i];
            count = 1;
        } else {
            count += 1;
        }
    }
    
    segments.push(Segment::new(digit, count));
    segments
}

/// Determines if the given list of segments has a double that
/// is not part of a larger group
fn has_double(segments: &[Segment]) -> bool {
    let mut valid_double_digit = false;
    
    for segment in segments {
        if segment.count == 2 {
            valid_double_digit = true;
        }
    }
    
    valid_double_digit
}

/// Determines if the given list of segments are descending
fn are_segments_descending(segments: &[Segment]) -> bool {
    for i in 0..segments.len()-1 {
        if segments[i].digit > segments[i+1].digit {
            return false;
        }
    }
    true
}

/// Determines if a given u32 is a valid password according to the
/// criteria
fn is_password_valid(num: u32) -> bool {
    let segments = break_into_segments(num);
    if has_double(&segments) {
        are_segments_descending(&segments)
    } else {
        false
    }
}

fn main() {
    let count = (LOWER_BOUND..=UPPER_BOUND)
        .filter(|num| is_password_valid(*num))
        .count();
    println!("Count is {}", count);
}