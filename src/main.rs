use crate::test::MyTest;

mod _0001_two_sum;
mod _0002_add_two_numbers;
mod _0003_longest_substring_without_repeating_characters;
mod test;

fn main() {
    // let test = crate::_00001_two_sum::Solution {};
    // let test = crate::_0002_add_two_numbers::Solution {};
    let test = crate::_0003_longest_substring_without_repeating_characters::Solution {};
    test.test();
}