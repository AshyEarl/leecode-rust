use crate::test::MyTest;

mod _00001_two_sum;
mod _00002_add_two_numbers;
mod test;

fn main() {
    // let test = crate::_00001_two_sum::Solution {};
    let test = crate::_00002_add_two_numbers::Solution {};
    test.test();
}