// Exercicio 140: Encontre uma arvore geradora minima de um grafo nao direcionado com Kruskal.
struct Dsu {
    pais: Vec<usize>,
    tamanhos: Vec<usize>,
}
impl Dsu {
    fn novo(n: usize) -> Self {
        Self {
            pais: (0..n).collect(),
            tamanhos: vec![1; n],
        }
    }
    fn raiz(&mut self, x: usize) -> usize {
        if self.pais[x] != x {
            self.pais[x] = self.raiz(self.pais[x]);
        }
        self.pais[x]
    }
    fn unir(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.raiz(a), self.raiz(b));
        if a == b {
            return false;
        }
        if self.tamanhos[a] < self.tamanhos[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.pais[b] = a;
        self.tamanhos[a] += self.tamanhos[b];
        true
    }
}
fn kruskal(n: usize, arestas: &[(usize, usize, u32)]) -> Option<(u64, Vec<(usize, usize)>)> {
    if arestas.iter().any(|&(a, b, _)| a >= n || b >= n) {
        return None;
    }
    let mut arestas = arestas.to_vec();
    arestas.sort_unstable_by_key(|&(_, _, peso)| peso);
    let mut dsu = Dsu::novo(n);
    let (mut custo, mut escolhidas) = (0, Vec::new());
    for (a, b, peso) in arestas {
        if dsu.unir(a, b) {
            custo += u64::from(peso);
            escolhidas.push((a, b));
        }
    }
    if escolhidas.len() == n.saturating_sub(1) {
        Some((custo, escolhidas))
    } else {
        None
    }
}
fn main() {
    let resultado = kruskal(4, &[(0, 1, 1), (1, 2, 2), (2, 3, 3), (0, 3, 10), (0, 2, 5)]).unwrap();
    assert_eq!(resultado.0, 6);
    assert_eq!(resultado.1.len(), 3);
    assert!(kruskal(3, &[(0, 1, 1)]).is_none());
    println!("Custo e arestas da AGM: {resultado:?}");
}
