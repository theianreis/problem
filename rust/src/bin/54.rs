// Exercicio 54: Separe pares e impares preservando a ordem original.
fn separar(valores: &[i32]) -> (Vec<i32>, Vec<i32>) {
    valores.iter().copied().partition(|n| n % 2 == 0)
}
fn main() {
    let (pares, impares) = separar(&[3, -2, 4, 1, 0]);
    assert_eq!(pares, vec![-2, 4, 0]);
    assert_eq!(impares, vec![3, 1]);
    println!("Pares: {pares:?}; impares: {impares:?}");
}
