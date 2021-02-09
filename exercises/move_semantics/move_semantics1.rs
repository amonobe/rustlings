// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    
    // essa linha é um pouco confusa, cria-se uma variável mutável de msm nome e liga-se ela
    // aos valores passados via parametro
    // a melhor prática seria mudar um pouco o nome da variável pra vec_linha ou para
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
