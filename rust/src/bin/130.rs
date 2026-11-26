// Exercicio 130: Mantenha a mediana de um fluxo de inteiros usando dois heaps.
use std::{cmp::Reverse, collections::BinaryHeap};
#[derive(Default)]
struct Mediana {
    menores: BinaryHeap<i32>,
    maiores: BinaryHeap<Reverse<i32>>,
}
impl Mediana {
    fn inserir(&mut self, valor: i32) {
        if self.menores.peek().is_none_or(|&n| valor <= n) {
            self.menores.push(valor);
        } else {
            self.maiores.push(Reverse(valor));
        }
        if self.menores.len() > self.maiores.len() + 1 {
            self.maiores.push(Reverse(self.menores.pop().unwrap()));
        } else if self.maiores.len() > self.menores.len() {
            self.menores.push(self.maiores.pop().unwrap().0);
        }
    }
    fn obter(&self) -> Option<f64> {
        let menor = f64::from(*self.menores.peek()?);
        if self.menores.len() == self.maiores.len() {
            Some((menor + f64::from(self.maiores.peek()?.0)) / 2.0)
        } else {
            Some(menor)
        }
    }
}
fn main() {
    let mut fluxo = Mediana::default();
    assert_eq!(fluxo.obter(), None);
    for (valor, esperado) in [(5, 5.0), (2, 3.5), (9, 5.0), (1, 3.5)] {
        fluxo.inserir(valor);
        assert_eq!(fluxo.obter(), Some(esperado));
    }
    println!("Mediana: {:?}", fluxo.obter());
}
