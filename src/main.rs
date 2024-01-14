mod number_system;

use number_system::{NumberSystem, NumberSystemTrait};
fn main() {
    let temp = NumberSystem::Binary(&String::from("1011")).to_hexadecimal();
    println!("{:?}",temp);

    let temp2 = NumberSystem::Decimal(&String::from("22")).to_hexadecimal();
    println!("{:?}",temp2);

}
