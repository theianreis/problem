// Exercicio 90: Encontre dois valores que somam o alvo em um vetor ordenado com dois ponteiros.
fn dois_ponteiros(valores: &[i32], alvo: i64) -> Option<(usize, usize)> {
    if valores.len() < 2 {
        return None;
    }
    let (mut esquerda, mut direita) = (0, valores.len() - 1);
    while esquerda < direita {
        let soma = i64::from(valores[esquerda]) + i64::from(valores[direita]);
        match soma.cmp(&alvo) {
            std::cmp::Ordering::Equal => return Some((esquerda, direita)),
            std::cmp::Ordering::Less => esquerda += 1,
            std::cmp::Ordering::Greater => direita -= 1,
        }
    }
    None
}
fn main() {
    assert_eq!(dois_ponteiros(&[1, 2, 4, 7], 9), Some((1, 3)));
    assert_eq!(dois_ponteiros(&[2], 4), None);
    println!("Indices: {:?}", dois_ponteiros(&[1, 2, 4, 7], 9));
}
