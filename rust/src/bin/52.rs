// Exercicio 52: Inverta um vetor no proprio lugar usando dois indices.
fn inverter(valores: &mut [i32]) {
    for esquerda in 0..valores.len() / 2 {
        valores.swap(esquerda, valores.len() - 1 - esquerda);
    }
}
fn main() {
    let mut valores = vec![1, 2, 3, 4, 5];
    inverter(&mut valores);
    assert_eq!(valores, vec![5, 4, 3, 2, 1]);
    inverter(&mut []);
    println!("Invertido: {valores:?}");
}
