// Exercicio 85: Construa a matriz identidade de ordem n.
fn identidade(n: usize) -> Vec<Vec<u8>> {
    let mut matriz = vec![vec![0; n]; n];
    for (i, linha) in matriz.iter_mut().enumerate() {
        linha[i] = 1;
    }
    matriz
}
fn main() {
    assert_eq!(identidade(2), vec![vec![1, 0], vec![0, 1]]);
    assert!(identidade(0).is_empty());
    println!("Identidade: {:?}", identidade(4));
}
