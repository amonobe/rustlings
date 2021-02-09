// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {

    //esse problema pode ser resolvido tirando o ponto-e-vírgula
    // OU adicionando a keyword 'return' no inicio
    num * num
}
