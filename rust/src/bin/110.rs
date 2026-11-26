// Exercicio 110: Ache o proximo elemento maior a direita usando pilha monotonica em O(n).
fn proximos_maiores(valores: &[i32]) -> Vec<Option<i32>> {
    let mut resultado = vec![None; valores.len()];
    let mut pilha: Vec<usize> = Vec::new();
    for (i, &valor) in valores.iter().enumerate() {
        while pilha.last().is_some_and(|&j| valores[j] < valor) {
            resultado[pilha.pop().unwrap()] = Some(valor);
        }
        pilha.push(i);
    }
    resultado
}
fn main() {
    let resultado = proximos_maiores(&[2, 1, 2, 4, 3]);
    assert_eq!(resultado, vec![Some(4), Some(2), Some(4), None, None]);
    println!("Proximos maiores: {resultado:?}");
}
