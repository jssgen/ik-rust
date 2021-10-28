extern crate time;

fn main() {
    let d = time::now();
    println!("Hoje é dia {}/{}/{}", d.tm_mday, d.tm_mon, d.tm_year + 1900);
    let (dia, mes, ano, hehe) = (d.tm_mday, d.tm_mon, d.tm_year + 1900, "hehe.");
    println!("Hoje é dia {}/{}/{}, {}", dia, mes, ano, hehe);
}
