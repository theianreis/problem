// Exercicio 128: Encontre os k maiores valores com um min-heap de tamanho k.
use std::{cmp::Reverse, collections::BinaryHeap};
fn maiores(valores: &[i32], k: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for &valor in valores {
        heap.push(Reverse(valor));
        if heap.len() > k {
            heap.pop();
        }
    }
    let mut resultado: Vec<_> = heap.into_iter().map(|Reverse(n)| n).collect();
    resultado.sort_unstable_by(|a, b| b.cmp(a));
    resultado
}
fn main() {
    assert_eq!(maiores(&[3, 1, 8, 8, 5], 3), vec![8, 8, 5]);
    assert!(maiores(&[1, 2], 0).is_empty());
    println!("Tres maiores: {:?}", maiores(&[3, 1, 8, 8, 5], 3));
}
