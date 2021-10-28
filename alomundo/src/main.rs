use std::io;
fn main() {
    println!("Alô, mundo!");
    let nome = "Sigma";
    println!("Alô, {}!", nome);
    let ola = "Olá,";
    let mundo = "mundo!";
    println!("{} {}", ola, mundo);
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    println!("Olá, {}!", name)
}
