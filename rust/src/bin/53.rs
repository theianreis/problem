// Exercicio 53: Rotacione um vetor k posicoes para a direita com tres inversoes.
fn rotacionar(valores: &mut [i32], k: usize) {
    if valores.is_empty() {
        return;
    }
    let k = k % valores.len();
    valores.reverse();
    valores[..k].reverse();
    valores[k..].reverse();
}
fn main() {
    let mut valores = vec![1, 2, 3, 4, 5];
    rotacionar(&mut valores, 7);
    assert_eq!(valores, vec![4, 5, 1, 2, 3]);
    rotacionar(&mut [], 3);
    println!("Rotacao: {valores:?}");
}
