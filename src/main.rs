use trust::add;
use trust::divide;
use trust::multiply;
use trust::sub;

fn main() {
    println!("add(1, 2) = {}", add(1, 2));
    println!("sub(1, 2) = {}", sub(1, 2));
    println!("multiply(1, 2) = {}", multiply(1, 2));
    println!("divide(1, 2) = {:?}", divide(1, 2));
}
