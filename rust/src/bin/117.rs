// Exercicio 117: Implemente um cache LRU com HashMap e VecDeque.
use std::collections::{HashMap, VecDeque};
// Versao didatica: promover uma chave custa O(capacidade), devido a retain.
struct Lru {
    capacidade: usize,
    dados: HashMap<i32, String>,
    ordem: VecDeque<i32>,
}
impl Lru {
    fn novo(capacidade: usize) -> Self {
        Self {
            capacidade,
            dados: HashMap::new(),
            ordem: VecDeque::new(),
        }
    }
    fn promover(&mut self, chave: i32) {
        self.ordem.retain(|&c| c != chave);
        self.ordem.push_back(chave);
    }
    fn obter(&mut self, chave: i32) -> Option<&str> {
        if !self.dados.contains_key(&chave) {
            return None;
        }
        self.promover(chave);
        self.dados.get(&chave).map(String::as_str)
    }
    fn inserir(&mut self, chave: i32, valor: &str) {
        if self.capacidade == 0 {
            return;
        }
        if !self.dados.contains_key(&chave) && self.dados.len() == self.capacidade {
            let antiga = self.ordem.pop_front().unwrap();
            self.dados.remove(&antiga);
        }
        self.dados.insert(chave, valor.into());
        self.promover(chave);
    }
}
fn main() {
    let mut cache = Lru::novo(2);
    cache.inserir(1, "A");
    cache.inserir(2, "B");
    assert_eq!(cache.obter(1), Some("A"));
    cache.inserir(3, "C");
    assert_eq!(cache.obter(2), None);
    cache.inserir(1, "atualizado");
    assert_eq!(cache.obter(1), Some("atualizado"));
    println!("LRU preservou a chave acessada e removeu a menos recente.");
}
