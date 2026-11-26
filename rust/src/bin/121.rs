// Exercicio 121: Implemente insercao e busca em uma arvore binaria de busca sem duplicatas.
#[derive(Default)]
struct Arvore {
    raiz: Option<Box<No>>,
}
struct No {
    valor: i32,
    esquerda: Arvore,
    direita: Arvore,
}
impl Arvore {
    fn inserir(&mut self, valor: i32) {
        match self.raiz.as_mut() {
            None => {
                self.raiz = Some(Box::new(No {
                    valor,
                    esquerda: Self::default(),
                    direita: Self::default(),
                }))
            }
            Some(no) if valor < no.valor => no.esquerda.inserir(valor),
            Some(no) if valor > no.valor => no.direita.inserir(valor),
            _ => {}
        }
    }
    fn contem(&self, valor: i32) -> bool {
        match self.raiz.as_ref() {
            None => false,
            Some(no) if valor == no.valor => true,
            Some(no) if valor < no.valor => no.esquerda.contem(valor),
            Some(no) => no.direita.contem(valor),
        }
    }
}
fn main() {
    // Sem balanceamento: altura e custo das operacoes podem chegar a O(n).
    let mut arvore = Arvore::default();
    for valor in [8, 3, 10, 1, 6, 3] {
        arvore.inserir(valor);
    }
    assert!(arvore.contem(6));
    assert!(!arvore.contem(7));
    println!("A arvore contem 6: {}", arvore.contem(6));
}
