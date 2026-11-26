// Exercicio 125: Remova um valor de uma BST tratando nos com zero, um ou dois filhos.
#[derive(Debug)]
struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}
fn inserir(raiz: &mut Option<Box<No>>, valor: i32) {
    match raiz {
        None => {
            *raiz = Some(Box::new(No {
                valor,
                esquerda: None,
                direita: None,
            }))
        }
        Some(no) if valor < no.valor => inserir(&mut no.esquerda, valor),
        Some(no) if valor > no.valor => inserir(&mut no.direita, valor),
        _ => {}
    }
}
fn remover(raiz: Option<Box<No>>, valor: i32) -> Option<Box<No>> {
    let mut no = raiz?;
    if valor < no.valor {
        no.esquerda = remover(no.esquerda, valor);
    } else if valor > no.valor {
        no.direita = remover(no.direita, valor);
    } else {
        match (no.esquerda.take(), no.direita.take()) {
            (None, direita) => return direita,
            (esquerda, None) => return esquerda,
            (esquerda, Some(direita)) => {
                let mut sucessor = &*direita;
                while let Some(ref proximo) = sucessor.esquerda {
                    sucessor = proximo;
                }
                no.valor = sucessor.valor;
                no.esquerda = esquerda;
                no.direita = remover(Some(direita), no.valor);
            }
        }
    }
    Some(no)
}
fn em_ordem(raiz: &Option<Box<No>>, saida: &mut Vec<i32>) {
    if let Some(no) = raiz {
        em_ordem(&no.esquerda, saida);
        saida.push(no.valor);
        em_ordem(&no.direita, saida);
    }
}
fn main() {
    let mut raiz = None;
    for valor in [5, 3, 7, 6, 8] {
        inserir(&mut raiz, valor);
    }
    for valor in [5, 3, 7, 99] {
        raiz = remover(raiz, valor);
    }
    let mut valores = Vec::new();
    em_ordem(&raiz, &mut valores);
    assert_eq!(valores, vec![6, 8]);
    println!("BST apos remocoes: {valores:?}");
}
