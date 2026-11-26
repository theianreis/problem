// Exercicio 57: Intercale dois vetores previamente ordenados em O(n + m).
fn intercalar(a: &[i32], b: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut resultado = Vec::with_capacity(a.len() + b.len());
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
    assert_eq!(intercalar(&[], &[1]), vec![1]);
    let resultado = intercalar(&[1, 3, 5], &[2, 3, 6]);
    assert_eq!(resultado, vec![1, 2, 3, 3, 5, 6]);
    println!("Intercalado: {resultado:?}");
}
