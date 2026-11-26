// Exercicio 67: Implemente quicksort no proprio vetor com particao de Lomuto.
// Pivo final: O(n log n) medio, O(n^2) no pior caso e recursao O(n).
fn quicksort(valores: &mut [i32]) {
    if valores.len() <= 1 {
        return;
    }
    let ultimo = valores.len() - 1;
    let pivo = valores[ultimo];
    let mut posicao = 0;
    for i in 0..ultimo {
        if valores[i] < pivo {
            valores.swap(i, posicao);
            posicao += 1;
        }
    }
    valores.swap(posicao, ultimo);
    let (esquerda, direita) = valores.split_at_mut(posicao);
    quicksort(esquerda);
    quicksort(&mut direita[1..]);
}
fn main() {
    let mut valores = [5, 3, 5, -1, 0];
    quicksort(&mut valores);
    assert_eq!(valores, [-1, 0, 3, 5, 5]);
    println!("Quicksort: {valores:?}");
}
