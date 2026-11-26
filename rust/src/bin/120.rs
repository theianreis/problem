// Exercicio 120: Consulte um intervalo de chaves em um mapa ordenado BTreeMap.
use std::collections::BTreeMap;
fn main() {
    let estoque = BTreeMap::from([
        (100, "arroz"),
        (120, "feijao"),
        (150, "leite"),
        (180, "cafe"),
    ]);
    let intervalo: Vec<_> = estoque
        .range(110..=160)
        .map(|(&codigo, &nome)| (codigo, nome))
        .collect();
    assert_eq!(intervalo, vec![(120, "feijao"), (150, "leite")]);
    assert_eq!(estoque.range(200..).count(), 0);
    println!("Produtos entre 110 e 160: {intervalo:?}");
}
