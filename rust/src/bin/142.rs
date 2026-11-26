// Exercicio 142: Implemente uma trie Unicode com insercao, busca exata e consulta de prefixos.
use std::collections::BTreeMap;
#[derive(Default)]
struct Trie {
    filhos: BTreeMap<char, Trie>,
    fim: bool,
}
impl Trie {
    fn inserir(&mut self, palavra: &str) {
        let mut no = self;
        for letra in palavra.chars() {
            no = no.filhos.entry(letra).or_default();
        }
        no.fim = true;
    }
    fn localizar(&self, texto: &str) -> Option<&Trie> {
        let mut no = self;
        for letra in texto.chars() {
            no = no.filhos.get(&letra)?;
        }
        Some(no)
    }
    fn contem(&self, palavra: &str) -> bool {
        self.localizar(palavra).is_some_and(|no| no.fim)
    }
    fn tem_prefixo(&self, prefixo: &str) -> bool {
        self.localizar(prefixo).is_some()
    }
}
fn main() {
    let mut trie = Trie::default();
    for palavra in ["casa", "casaco", "café", "árvore"] {
        trie.inserir(palavra);
    }
    assert!(trie.contem("casa"));
    assert!(!trie.contem("cas"));
    assert!(trie.tem_prefixo("cas"));
    assert!(trie.contem("árvore"));
    assert!(!trie.tem_prefixo("ru"));
    println!("Prefixo cas encontrado: {}", trie.tem_prefixo("cas"));
}
