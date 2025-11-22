pub mod year2019;

pub type SolveFn = fn(&str) -> Result<String, String>;

pub struct Day {
    pub name: &'static str,
    pub part1: SolveFn,
    pub part2: SolveFn,
}

pub static YEARS: &[(&str, &[Day])] = &[("2019", year2019::DAYS)];

#[macro_export]
macro_rules! register_days {
    ($($day:ident),* $(,)?) => {
        $(mod $day;)*

        pub static DAYS: &[$crate::solvers::Day] = &[
            $(
                $crate::solvers::Day {
                    name: stringify!($day),
                    part1: $day::part1,
                    part2: $day::part2,
                },
            )*
        ];
    };
}

#[macro_export]
macro_rules! part1_todo {
    () => {
        pub fn part1(input: &str) -> Result<String, String> {
            return Ok("Not implemented".to_string());
        }
    };
}

#[macro_export]
macro_rules! part2_todo {
    () => {
        pub fn part2(input: &str) -> Result<String, String> {
            return Ok("Not implemented".to_string());
        }
    };
}
