// Exercicio 122: Percorra uma arvore binaria em pre-ordem, em-ordem e pos-ordem.
struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}
fn folha(valor: i32) -> Option<Box<No>> {
    Some(Box::new(No {
        valor,
        esquerda: None,
        direita: None,
    }))
}
fn percorrer(no: &Option<Box<No>>, pre: &mut Vec<i32>, em: &mut Vec<i32>, pos: &mut Vec<i32>) {
    if let Some(no) = no {
        pre.push(no.valor);
        percorrer(&no.esquerda, pre, em, pos);
        em.push(no.valor);
        percorrer(&no.direita, pre, em, pos);
        pos.push(no.valor);
    }
}
fn main() {
    let arvore = Some(Box::new(No {
        valor: 2,
        esquerda: folha(1),
        direita: folha(3),
    }));
    let (mut pre, mut em, mut pos) = (Vec::new(), Vec::new(), Vec::new());
    percorrer(&arvore, &mut pre, &mut em, &mut pos);
    assert_eq!(pre, vec![2, 1, 3]);
    assert_eq!(em, vec![1, 2, 3]);
    assert_eq!(pos, vec![1, 3, 2]);
    println!("Pre: {pre:?}; em: {em:?}; pos: {pos:?}");
}
