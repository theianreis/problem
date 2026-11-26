// Exercicio 27: Encontre o menor e o maior valor de uma fatia, incluindo o caso vazio.
fn extremos(valores: &[i32]) -> Option<(i32, i32)> {
    let &primeiro = valores.first()?;
    let (mut menor, mut maior) = (primeiro, primeiro);
    for &valor in &valores[1..] {
        menor = menor.min(valor);
        maior = maior.max(valor);
    }
    Some((menor, maior))
}
fn main() {
    assert_eq!(extremos(&[]), None);
    assert_eq!(extremos(&[4, -2, 9, 1]), Some((-2, 9)));
    println!("Extremos: {:?}", extremos(&[4, -2, 9, 1]));
}
