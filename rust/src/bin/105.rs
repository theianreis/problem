// Exercicio 105: Implemente uma fila circular com capacidade fixa e reaproveitamento de espaco.
struct FilaCircular<T> {
    dados: Vec<Option<T>>,
    inicio: usize,
    tamanho: usize,
}
impl<T> FilaCircular<T> {
    fn nova(capacidade: usize) -> Self {
        Self {
            dados: (0..capacidade).map(|_| None).collect(),
            inicio: 0,
            tamanho: 0,
        }
    }
    fn inserir(&mut self, valor: T) -> Result<(), T> {
        if self.tamanho == self.dados.len() {
            return Err(valor);
        }
        let fim = (self.inicio + self.tamanho) % self.dados.len();
        self.dados[fim] = Some(valor);
        self.tamanho += 1;
        Ok(())
    }
    fn remover(&mut self) -> Option<T> {
        if self.tamanho == 0 {
            return None;
        }
        let valor = self.dados[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.dados.len();
        self.tamanho -= 1;
        valor
    }
}
fn main() {
    let mut fila = FilaCircular::nova(2);
    assert_eq!(fila.inserir(1), Ok(()));
    assert_eq!(fila.inserir(2), Ok(()));
    assert_eq!(fila.inserir(3), Err(3));
    assert_eq!(fila.remover(), Some(1));
    fila.inserir(3).unwrap();
    assert_eq!(fila.remover(), Some(2));
    assert_eq!(fila.remover(), Some(3));
    assert_eq!(fila.remover(), None);
    assert_eq!(FilaCircular::nova(0).inserir(1), Err(1));
    println!("Fila circular validada, incluindo a volta ao inicio.");
}
