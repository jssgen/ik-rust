fn resultado(nota1: f32, nota2: f32, nota3: f32, nota4: f32) {
    let media_nota: f32 = nota1 + nota2 + nota3 + nota4;
    const MAX: f32 = 100.0;
    const MIN: f32 = 60.0;
    if media_nota >= MIN && media_nota < MAX {
        println!(
            "Parabéns! aprovado com a média de notas: {} pontos.",
            media_nota
        );
    } else if media_nota < MIN {
        println!(
            "Você precisa fazer a prova de recuperação final, está com: {} pontos.",
            media_nota
        );
    } else if media_nota == MAX {
        println!("Parabéns!!! Aprovado com 100 pontos, passou com excelência.");
    } else if media_nota > MAX || media_nota < 0.0 {
        println!("Há algo de errado na conta, pode digita os valores novamente?");
    }
}

fn main() {
    let x = if 10 + 5 == 15 {
        "10 + 5 é 15"
    } else {
        "10 + 5 não é 15"
    };
    println!("{}", x);
    resultado(18.0, 18.0, 18.0, 18.0);
}
