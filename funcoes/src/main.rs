extern crate time;
fn dia_atual() {
    let d = time::now();
    const O_1900: i32 = 1900;
    println!(
        "Hoje é dia {}/{}/{}",
        d.tm_mday,
        d.tm_mon,
        d.tm_year + O_1900
    );
}
fn soma(x: i32, y: i32) -> i32 {
    x + y
}
fn executar_func(function: fn()) {
    function();
}
fn printar() {
    println!("Olá!");
}
fn main() {
    dia_atual();
    let x = 5;
    let y = 8;
    let a = 3;
    let b = 9;
    println!("{} + {} = {}", x, y, soma(x, y));
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("{} + {} = {}", a, b, sum(a, b));
    let func_ponteiro: fn() = printar;
    executar_func(func_ponteiro);
}
