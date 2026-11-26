// Exercicio 124: Valide uma BST usando limites globais, sem permitir valores duplicados.
struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}
fn validar(no: &Option<Box<No>>, minimo: Option<i32>, maximo: Option<i32>) -> bool {
    let Some(no) = no else {
        return true;
    };
    if minimo.is_some_and(|m| no.valor <= m) || maximo.is_some_and(|m| no.valor >= m) {
        return false;
    }
    validar(&no.esquerda, minimo, Some(no.valor)) && validar(&no.direita, Some(no.valor), maximo)
}
fn main() {
    // 7 e menor que 10, mas esta na subarvore direita: invalida.
    let arvore = Some(Box::new(No {
        valor: 10,
        esquerda: None,
        direita: Some(Box::new(No {
            valor: 15,
            esquerda: Some(Box::new(No {
                valor: 7,
                esquerda: None,
                direita: None,
            })),
            direita: None,
        })),
    }));
    assert!(!validar(&arvore, None, None));
    assert!(validar(&None, None, None));
    println!("BST valida: {}", validar(&arvore, None, None));
}
