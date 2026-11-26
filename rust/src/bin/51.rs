// Exercicio 51: Calcule soma e media de um vetor, tratando o vetor vazio.
fn estatisticas(valores: &[i32]) -> Option<(i64, f64)> {
    if valores.is_empty() {
        return None;
    }
    let soma: i64 = valores.iter().map(|&n| i64::from(n)).sum();
    Some((soma, soma as f64 / valores.len() as f64))
}
fn main() {
    assert_eq!(estatisticas(&[]), None);
    assert_eq!(estatisticas(&[2, 4, 6]), Some((12, 4.0)));
    println!("Soma e media: {:?}", estatisticas(&[2, 4, 6]));
}
