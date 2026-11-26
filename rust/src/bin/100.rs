// Exercicio 100: Calcule o tamanho da maior subsequencia estritamente crescente em O(n log n).
fn lis(valores: &[i32]) -> usize {
    let mut finais = Vec::new();
    for &valor in valores {
        let posicao = finais.partition_point(|&ultimo| ultimo < valor);
        if posicao == finais.len() {
            finais.push(valor);
        } else {
            finais[posicao] = valor;
        }
    }
    finais.len()
}
fn main() {
    assert_eq!(lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(lis(&[2, 2, 2]), 1);
    assert_eq!(lis(&[]), 0);
    println!("Tamanho da LIS: {}", lis(&[10, 9, 2, 5, 3, 7, 101, 18]));
}
