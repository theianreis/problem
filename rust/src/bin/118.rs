// Exercicio 118: Implemente tabela hash com colisoes resolvidas por encadeamento separado.
struct Tabela {
    baldes: Vec<Vec<(String, i32)>>,
}
impl Tabela {
    fn nova(capacidade: usize) -> Self {
        Self {
            baldes: vec![Vec::new(); capacidade.max(1)],
        }
    }
    fn indice(&self, chave: &str) -> usize {
        // Hash simples para estudo; nao e um hash criptografico.
        chave.bytes().fold(0_usize, |h, b| {
            h.wrapping_mul(31).wrapping_add(usize::from(b))
        }) % self.baldes.len()
    }
    fn inserir(&mut self, chave: &str, valor: i32) {
        let indice = self.indice(chave);
        let balde = &mut self.baldes[indice];
        if let Some(par) = balde.iter_mut().find(|(c, _)| c == chave) {
            par.1 = valor;
        } else {
            balde.push((chave.into(), valor));
        }
    }
    fn obter(&self, chave: &str) -> Option<i32> {
        self.baldes[self.indice(chave)]
            .iter()
            .find(|(c, _)| c == chave)
            .map(|(_, v)| *v)
    }
    fn remover(&mut self, chave: &str) -> Option<i32> {
        let indice = self.indice(chave);
        let balde = &mut self.baldes[indice];
        let posicao = balde.iter().position(|(c, _)| c == chave)?;
        Some(balde.swap_remove(posicao).1)
    }
}
fn main() {
    // Um balde forca colisoes para demonstrar o encadeamento.
    let mut tabela = Tabela::nova(1);
    tabela.inserir("a", 1);
    tabela.inserir("b", 2);
    tabela.inserir("a", 3);
    assert_eq!(tabela.obter("a"), Some(3));
    assert_eq!(tabela.remover("b"), Some(2));
    assert_eq!(tabela.obter("b"), None);
    println!("Tabela hash validada com colisoes.");
}
