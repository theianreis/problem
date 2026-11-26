// Exercicio 141: Implemente union-find com compressao de caminhos e uniao por tamanho.
struct UnionFind {
    pais: Vec<usize>,
    tamanhos: Vec<usize>,
    componentes: usize,
}
impl UnionFind {
    fn novo(n: usize) -> Self {
        Self {
            pais: (0..n).collect(),
            tamanhos: vec![1; n],
            componentes: n,
        }
    }
    // Os indices passados as operacoes devem ser menores que n.
    fn encontrar(&mut self, x: usize) -> usize {
        if self.pais[x] != x {
            self.pais[x] = self.encontrar(self.pais[x]);
        }
        self.pais[x]
    }
    fn unir(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.encontrar(a), self.encontrar(b));
        if a == b {
            return false;
        }
        if self.tamanhos[a] < self.tamanhos[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.pais[b] = a;
        self.tamanhos[a] += self.tamanhos[b];
        self.componentes -= 1;
        true
    }
}
fn main() {
    let mut conjuntos = UnionFind::novo(5);
    assert!(conjuntos.unir(0, 1));
    assert!(conjuntos.unir(1, 2));
    assert!(!conjuntos.unir(0, 2));
    assert_eq!(conjuntos.encontrar(0), conjuntos.encontrar(2));
    assert_eq!(conjuntos.componentes, 3);
    println!("Componentes restantes: {}", conjuntos.componentes);
}
