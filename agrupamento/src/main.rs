mod matematica {
    fn is_zero(number: i32) -> bool {
        if number == 0 {
            return true;
        };
        false
    }
    pub fn soma(a: i32, b: i32) -> i32 {
        a + b
    }
    pub fn divide(a: i32, b: i32) -> i32 {
        a / b
    }
    pub fn subtrai(a: i32, b: i32) -> i32 {
        a - b
    }
    pub fn multiplica(a: i32, b: i32) -> i32 {
        a * b
    }
}
use matematica::divide as dividir;
fn main() {
    let a: i32 = 10;
    let b: i32 = 6;
    println!("{} + {} = {}", a, b, matematica::soma(a, b));
    println!("{} - {} = {}", a, b, matematica::subtrai(a, b));
    println!("{} / {} = {}", a, b, matematica::divide(a, b));
    println!("{} / {} = {}", a, b, dividir(a, b));
    println!("{} * {} = {}", a, b, matematica::multiplica(a, b));
}
