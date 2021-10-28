#![feature(exclusive_range_pattern)]
#[allow(unused_variables)]
fn checar_nota(nota: f32) -> () {
    match nota {
        0.0..4.8 => println!("Sujeito reprovado."),
        4.9..5.9 => println!("Sujeito de recuperação."),
        6.0..10.0 => println!("Sujeito aprovado."),
        _ => println!("Nota inválida!"),
    }
}
fn primo(numero: i32) -> () {
    match numero {
        0 => println!("Zero."),
        2 | 3 | 5 | 7 | 11 => println!("Primo!"),
        _ => println!("Qualquer número."),
    }
}
enum Genero {
    Masculino,
    Feminino,
}

fn checar_tupla(f: (i32, i32)) -> () {
    match f {
        (_x, 0) => println!("O segundo número da tupla é 0."),
        (0, _x) => println!("O primeiro número da tupla é 0."),
        _ => println!("Não há zeros na tupla."),
    }
}

fn main() {
    checar_nota(6.8);
    primo(5);
    let genero = Genero::Feminino;
    match genero {
        Genero::Masculino => println!("Masculino!"),
        Genero::Feminino => println!("Feminino!"),
    }
    checar_tupla((0, 10));
    checar_tupla((10, 0));
    checar_tupla((20, 1));

    fn vogal_ou_consoante(c: char) -> char {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 'v',
            'A' | 'E' | 'I' | 'O' | 'U' => 'v',
            _ => 'c',
        }
    }

    println!("{}", vogal_ou_consoante('a'));
    println!("{}", vogal_ou_consoante('b'));

    let nome: &'static str = "Lucas";
    let mut consoantes = 0;
    let mut vogais = 0;
    for i in nome.chars() {
        match vogal_ou_consoante(i) {
            'v' => vogais += 1,
            'c' => consoantes += 1,
            _ => ()
        }
    }
    println!("{} tem {} vogais e {} consoantes.", nome, vogais, consoantes);
}
