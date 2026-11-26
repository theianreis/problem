// Exercicio 66: Implemente merge sort recursivo, com tempo O(n log n).
fn merge_sort(valores: &[i32]) -> Vec<i32> {
    if valores.len() <= 1 {
        return valores.to_vec();
    }
    let meio = valores.len() / 2;
    let a = merge_sort(&valores[..meio]);
    let b = merge_sort(&valores[meio..]);
    let (mut i, mut j) = (0, 0);
    let mut resultado = Vec::with_capacity(valores.len());
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            resultado.push(a[i]);
            i += 1;
        } else {
            resultado.push(b[j]);
            j += 1;
        }
    }
    resultado.extend_from_slice(&a[i..]);
    resultado.extend_from_slice(&b[j..]);
    resultado
}
fn main() {
    assert!(merge_sort(&[]).is_empty());
    let resultado = merge_sort(&[7, 2, 6, 1, 2]);
    assert_eq!(resultado, vec![1, 2, 2, 6, 7]);
    println!("Merge sort: {resultado:?}");
}
