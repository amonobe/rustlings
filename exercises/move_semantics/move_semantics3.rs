// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
// IMPORTANTE: a variavel vec0 originalmente não é mutável
// quando os seus valores são passados para função fill_vec elas são recebidas como um parâmetro mutável
// isso não é um problema pois os valores foram movidos e agora pertencem à função fill_vec
// por isso é possível que o valor originalmente de uma variável imutável seja atribuído a um parâmetro mutável
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
