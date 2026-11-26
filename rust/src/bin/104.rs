// Exercicio 104: Implemente uma fila generica FIFO com VecDeque.
use std::collections::VecDeque;
struct Fila<T> {
    itens: VecDeque<T>,
}
impl<T> Fila<T> {
    fn nova() -> Self {
        Self {
            itens: VecDeque::new(),
        }
    }
    fn enfileirar(&mut self, item: T) {
        self.itens.push_back(item);
    }
    fn desenfileirar(&mut self) -> Option<T> {
        self.itens.pop_front()
    }
    fn frente(&self) -> Option<&T> {
        self.itens.front()
    }
}
fn main() {
    let mut fila = Fila::nova();
    fila.enfileirar("Ana");
    fila.enfileirar("Bia");
    assert_eq!(fila.frente(), Some(&"Ana"));
    assert_eq!(fila.desenfileirar(), Some("Ana"));
    assert_eq!(fila.desenfileirar(), Some("Bia"));
    assert_eq!(fila.desenfileirar(), None);
    println!("Fila FIFO validada.");
}
