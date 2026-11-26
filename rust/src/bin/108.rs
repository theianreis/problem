// Exercicio 108: Inverta uma lista encadeada transferindo os nos sem clonar valores.
struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}
fn inverter(mut inicio: Option<Box<No>>) -> Option<Box<No>> {
    let mut anterior = None;
    while let Some(mut no) = inicio {
        inicio = no.proximo.take();
        no.proximo = anterior;
        anterior = Some(no);
    }
    anterior
}
fn main() {
    let mut lista = None;
    for valor in (1..=4).rev() {
        lista = Some(Box::new(No {
            valor,
            proximo: lista,
        }));
    }
    lista = inverter(lista);
    let mut valores = Vec::new();
    while let Some(no) = lista {
        valores.push(no.valor);
        lista = no.proximo;
    }
    assert_eq!(valores, vec![4, 3, 2, 1]);
    assert!(inverter(None).is_none());
    println!("Lista invertida: {valores:?}");
}
