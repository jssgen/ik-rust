fn main() {
    println!("Char:\n");
    let _x = 'x';
    let a = '\u{2764}';
    println!("{}", a);
    let z: char = '9';
    println!("O {} é dígito?", z.is_digit(10));
    println!("O {} é binário?", z.is_digit(2));
    let b: char = '→';
    let c: String = a.escape_unicode().collect();
    println!("{} em unicode: {}", b, c);
    println!("{}", 'a'.is_alphabetic());
    println!("Maiuscula -> {}", 'a'.is_uppercase());
    println!("Minuscula -> {}", 'a'.is_lowercase());
    println!("Whitespace -> {}", 'a'.is_whitespace());

    println!("\nBooleanos:");
    let _y: bool = true;
    let _p: bool = false;
    if _y {
        println!("Y é verdadeiro!");
    } else {
        println!("Y é falso!");
    }
    if _p {
        println!("P é verdadeiro!");
    } else {
        println!("P é falso!");
    }
    println!("True E Falso: {}", true && false);
    println!("True ou Falso: {}", true || false);
    println!("Não true: {}", !true);

    println!("\nValores numéricos:");
    // let a: i32 = 4294967295; overflow literals! :D, resolva trocando i32 por u32
    println!("i8 = {} a {}", i8::min_value(), i8::max_value());
    println!("i16= {} a {}", i16::min_value(), i16::max_value());
    println!("i32= {} a {}", i32::min_value(), i32::max_value());
    println!("i64= {} a {}", i64::min_value(), i64::max_value());
    println!("i128 = {} a {}", i128::min_value(), i128::max_value());
    println!("u8 = {} a {}", u8::min_value(), u8::max_value());
    println!("u16= {} a {}", u16::min_value(), u16::max_value());
    println!("u32= {} a {}", u32::min_value(), u32::max_value());
    println!("u64= {} a {}", u64::min_value(), u64::max_value());
    println!("u128= {} a {}", u128::min_value(), u128::max_value());

    println!("\nFloats:");
    let a: f32 = 3.549236;
    println!("Floor: {}", a.floor());
    println!("Ceil: {}", a.ceil());
    println!("Round: {}", a.round());
    println!("Truncate: {}", a.trunc());
    println!("Fractional: {}", a.fract());
    println!("Is Finite?: {}", a.is_finite());
    println!("Is Infinite?: {}", a.is_infinite());
    println!("Is NaN?: {}", a.is_nan());

    println!("\nArrays e Slices:");
    let mut array = ["Lucas", "Rust", "Roberto"];
    println!(
        "{} {} está programando em {}.",
        array[0], array[2], array[1]
    );
    array[0] = "João";
    println!(
        "{} {} está programando em {}.",
        array[0], array[2], array[1]
    );
    let u = [1; 4]; // tem essa sintaxe pra definir arrays também, [T; N], onde T é o tipo dos elementos do array e N o seu tamanho, veja como u fica de maneira literal, na variável k.
    println!("{}", u.len());
    let _k = [0, 0, 0, 0, 0];
    let mut n = [""; 5];
    n[0] = "Cargo";
    n[1] = "Apple";
    n[2] = "Robert";
    println!("{}, {}, {}, {}", n[0], n[1], n[2], n[3]);

    let i = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let t = &i[..];
    let r = &i[1..5];
    println!("{}, {}, {}", i.len(), t.len(), r.len());
    for s in i.iter() {
        println!("{}", s);
    }
    println!("");
    for s in r.iter() {
        println!("{}", s);
    }

    println!("\nTuplas:");
    // chave e valor da chave
    let v = ('a', "char");
    let q = (32, "inteiro");
    let q1 = q.0;
    println!("{}, {}", q1, q.1);
    let (v1, v2) = v;
    println!("{}, {}", v1, v2);

    println!("\nEnums:");
    enum Genero {
        Masculino,
        Feminino,
    }
    struct Pessoa {
        nome: &'static str,
        genero: Genero,
    }
    let nelson = Pessoa {
        nome: "Nelson Mandela",
        genero: Genero::Masculino,
    };
}
