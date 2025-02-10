pub mod functions;
pub mod input;

use functions::ArrName;
use input::input;

fn main() {
    let nums: Vec<i32> = input();
    let mut out = ArrName::new(nums);
    out.forword();
    out.display();
    // println!("{:?}", nums);
}
