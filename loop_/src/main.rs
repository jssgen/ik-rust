fn main() {
    let mut a = 0;
    loop {
        a += 1;
        println!("{}", a);
        if a >= 10 {
            break;
        }
    }
}
