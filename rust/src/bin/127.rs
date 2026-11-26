// Exercicio 127: Implemente um min-heap binario manual com insercao e extracao em O(log n).
#[derive(Default)]
struct MinHeap {
    dados: Vec<i32>,
}
impl MinHeap {
    fn inserir(&mut self, valor: i32) {
        self.dados.push(valor);
        let mut i = self.dados.len() - 1;
        while i > 0 {
            let pai = (i - 1) / 2;
            if self.dados[pai] <= self.dados[i] {
                break;
            }
            self.dados.swap(i, pai);
            i = pai;
        }
    }
    fn extrair(&mut self) -> Option<i32> {
        let ultimo = self.dados.pop()?;
        if self.dados.is_empty() {
            return Some(ultimo);
        }
        let menor = std::mem::replace(&mut self.dados[0], ultimo);
        let mut i = 0;
        loop {
            let esquerda = 2 * i + 1;
            if esquerda >= self.dados.len() {
                break;
            }
            let direita = esquerda + 1;
            let filho = if direita < self.dados.len() && self.dados[direita] < self.dados[esquerda]
            {
                direita
            } else {
                esquerda
            };
            if self.dados[i] <= self.dados[filho] {
                break;
            }
            self.dados.swap(i, filho);
            i = filho;
        }
        Some(menor)
    }
}
fn main() {
    let mut heap = MinHeap::default();
    for valor in [5, 1, 3, -2, 1] {
        heap.inserir(valor);
    }
    let mut ordenados = Vec::new();
    while let Some(valor) = heap.extrair() {
        ordenados.push(valor);
    }
    assert_eq!(ordenados, vec![-2, 1, 1, 3, 5]);
    println!("Extraidos: {ordenados:?}");
}
