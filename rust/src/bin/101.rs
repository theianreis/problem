// Exercicio 101: Implemente uma pilha generica com Vec: empilhar, desempilhar e consultar topo.
struct Pilha<T> {
    itens: Vec<T>,
}
impl<T> Pilha<T> {
    fn nova() -> Self {
        Self { itens: Vec::new() }
    }
    fn empilhar(&mut self, item: T) {
        self.itens.push(item);
    }
    fn desempilhar(&mut self) -> Option<T> {
        self.itens.pop()
    }
    fn topo(&self) -> Option<&T> {
        self.itens.last()
    }
}
fn main() {
    let mut pilha = Pilha::nova();
    assert_eq!(pilha.desempilhar(), None);
    pilha.empilhar(10);
    pilha.empilhar(20);
    assert_eq!(pilha.topo(), Some(&20));
    assert_eq!(pilha.desempilhar(), Some(20));
    assert_eq!(pilha.desempilhar(), Some(10));
    println!("Pilha LIFO validada.");
}
