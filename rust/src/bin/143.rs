// Exercicio 143: Implemente uma arvore de Fenwick com atualizacao e soma prefixada em O(log n).
struct Fenwick {
    dados: Vec<i64>,
}
impl Fenwick {
    fn nova(n: usize) -> Self {
        Self {
            dados: vec![0; n + 1],
        }
    }
    // Indices externos comecam em zero. Os totais devem caber em i64.
    fn adicionar(&mut self, indice: usize, delta: i64) -> bool {
        if indice >= self.dados.len() - 1 {
            return false;
        }
        let mut i = indice + 1;
        while i < self.dados.len() {
            self.dados[i] += delta;
            i += i & i.wrapping_neg();
        }
        true
    }
    fn prefixo(&self, fim: usize) -> Option<i64> {
        if fim >= self.dados.len() {
            return None;
        }
        let (mut soma, mut i) = (0, fim);
        while i > 0 {
            soma += self.dados[i];
            i &= i - 1;
        }
        Some(soma)
    }
    fn intervalo(&self, inicio: usize, fim: usize) -> Option<i64> {
        if inicio > fim {
            return None;
        }
        Some(self.prefixo(fim)? - self.prefixo(inicio)?)
    }
}
fn main() {
    let mut arvore = Fenwick::nova(5);
    for (i, valor) in [1, 2, 3, 4, 5].into_iter().enumerate() {
        assert!(arvore.adicionar(i, valor));
    }
    assert_eq!(arvore.intervalo(1, 4), Some(9));
    arvore.adicionar(2, 7);
    assert_eq!(arvore.intervalo(1, 4), Some(16));
    assert!(!arvore.adicionar(5, 1));
    println!("Soma atualizada [1, 4): {:?}", arvore.intervalo(1, 4));
}
