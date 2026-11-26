// Exercicio 107: Construa uma lista simplesmente encadeada generica com Box e Option.
struct No<T> {
    valor: T,
    proximo: Option<Box<No<T>>>,
}
struct Lista<T> {
    inicio: Option<Box<No<T>>>,
}
impl<T> Lista<T> {
    fn nova() -> Self {
        Self { inicio: None }
    }
    fn inserir_inicio(&mut self, valor: T) {
        self.inicio = Some(Box::new(No {
            valor,
            proximo: self.inicio.take(),
        }));
    }
    fn remover_inicio(&mut self) -> Option<T> {
        self.inicio.take().map(|no| {
            self.inicio = no.proximo;
            no.valor
        })
    }
    fn iterar(&self) -> impl Iterator<Item = &T> {
        std::iter::successors(self.inicio.as_deref(), |no| no.proximo.as_deref())
            .map(|no| &no.valor)
    }
}
impl<T> Drop for Lista<T> {
    fn drop(&mut self) {
        // Destruicao iterativa evita uma cadeia de chamadas recursivas.
        while let Some(mut no) = self.inicio.take() {
            self.inicio = no.proximo.take();
        }
    }
}
fn main() {
    let mut lista = Lista::nova();
    lista.inserir_inicio(1);
    lista.inserir_inicio(2);
    assert_eq!(lista.iterar().copied().collect::<Vec<_>>(), vec![2, 1]);
    assert_eq!(lista.remover_inicio(), Some(2));
    println!("Lista: {:?}", lista.iterar().collect::<Vec<_>>());
}
