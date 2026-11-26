// Exercicio 123: Calcule altura em numero de nos e quantidade de folhas de uma arvore.
struct No {
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}
fn medidas(no: &Option<Box<No>>) -> (usize, usize) {
    match no {
        None => (0, 0),
        Some(no) => {
            let (ae, fe) = medidas(&no.esquerda);
            let (ad, fd) = medidas(&no.direita);
            (1 + ae.max(ad), if ae == 0 && ad == 0 { 1 } else { fe + fd })
        }
    }
}
fn main() {
    let folha = || {
        Some(Box::new(No {
            esquerda: None,
            direita: None,
        }))
    };
    let arvore = Some(Box::new(No {
        esquerda: folha(),
        direita: folha(),
    }));
    assert_eq!(medidas(&None), (0, 0));
    assert_eq!(medidas(&arvore), (2, 2));
    println!("Altura e folhas: {:?}", medidas(&arvore));
}
