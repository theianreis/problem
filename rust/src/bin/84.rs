// Exercicio 84: Some as diagonais principal e secundaria de uma matriz quadrada.
fn diagonais(matriz: &[Vec<i32>]) -> Option<(i64, i64)> {
    let n = matriz.len();
    if matriz.iter().any(|linha| linha.len() != n) {
        return None;
    }
    let (mut principal, mut secundaria) = (0, 0);
    for (i, linha) in matriz.iter().enumerate() {
        principal += i64::from(linha[i]);
        secundaria += i64::from(linha[n - 1 - i]);
    }
    Some((principal, secundaria))
}
fn main() {
    let matriz = [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(diagonais(&matriz), Some((15, 15)));
    println!("Diagonais: {:?}", diagonais(&matriz));
}
