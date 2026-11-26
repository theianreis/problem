// Exercicio 60: Ache o unico ausente em 0..=n, dados n numeros distintos nesse intervalo.
fn ausente(valores: &[usize]) -> usize {
    // XOR cancela pares iguais e dispensa uma soma que poderia transbordar.
    valores
        .iter()
        .enumerate()
        .fold(valores.len(), |acc, (i, &n)| acc ^ i ^ n)
}
fn main() {
    assert_eq!(ausente(&[]), 0);
    assert_eq!(ausente(&[3, 0, 1]), 2);
    println!("Numero ausente: {}", ausente(&[3, 0, 1]));
}
