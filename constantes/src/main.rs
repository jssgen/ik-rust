extern crate time;
fn main() {
    const Y: i32 = 3;
    println!("{}", Y);
    const O_1900: i32 = 1900;
    let d = time::now();
    println!(
        "Hoje é dia {}/{}/{}",
        d.tm_mday,
        d.tm_mon,
        d.tm_year + O_1900
    );
}
