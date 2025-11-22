/// A problem contains the input data for part1 and optionally part2.
/// It may contain solutions for these problems.
pub struct Input {
    pub part1: String,
    pub part2: Option<String>,
    pub solution1: Option<String>,
    pub solution2: Option<String>,
}

/// The outcome contains results of solving a problem
pub struct Outcome {
    pub year: &'static str,
    pub day: &'static str,
    pub part: String,
    pub result: String,
    pub solution: String,
    pub correct: bool,
    pub elapsed_ms: i32,
}
