// Exercicio 109: Manipule uma lista duplamente encadeada com LinkedList da biblioteca padrao.
use std::collections::LinkedList;
fn main() {
    let mut lista: LinkedList<_> = [1, 2, 3, 4].into_iter().collect();
    let mut segunda = lista.split_off(2);
    assert_eq!(lista.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    segunda.push_front(0);
    assert_eq!(segunda.pop_back(), Some(4));
    lista.append(&mut segunda);
    assert!(segunda.is_empty());
    assert_eq!(lista.iter().copied().collect::<Vec<_>>(), vec![1, 2, 0, 3]);
    println!("Lista concatenada: {lista:?}");
}
