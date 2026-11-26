// Exercicio 89: Ache a maior soma de k elementos consecutivos com janela deslizante.
fn maior_soma(valores: &[i32], k: usize) -> Option<i64> {
    if k == 0 || k > valores.len() {
        return None;
    }
    let mut soma: i64 = valores[..k].iter().map(|&n| i64::from(n)).sum();
    let mut melhor = soma;
    for i in k..valores.len() {
        soma += i64::from(valores[i]) - i64::from(valores[i - k]);
        melhor = melhor.max(soma);
    }
    Some(melhor)
}
fn main() {
    assert_eq!(maior_soma(&[2, 1, 5, 1, 3, 2], 3), Some(9));
    assert_eq!(maior_soma(&[-4, -2], 1), Some(-2));
    assert_eq!(maior_soma(&[], 1), None);
    println!("Maior soma: {:?}", maior_soma(&[2, 1, 5, 1, 3, 2], 3));
}
