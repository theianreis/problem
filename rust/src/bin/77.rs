// Exercicio 77: Descomprima grupos RLE com limite de caracteres para a saida.
fn descomprimir(grupos: &[(char, usize)], limite: usize) -> Option<String> {
    let mut total = 0_usize;
    for &(_, quantidade) in grupos {
        if quantidade == 0 {
            return None;
        }
        total = total.checked_add(quantidade)?;
        if total > limite {
            return None;
        }
    }
    Some(
        grupos
            .iter()
            .flat_map(|&(c, n)| std::iter::repeat_n(c, n))
            .collect(),
    )
}
fn main() {
    let grupos = [('a', 3), ('á', 2)];
    assert_eq!(descomprimir(&grupos, 5), Some("aaaáá".into()));
    assert_eq!(descomprimir(&grupos, 4), None);
    println!("Texto: {:?}", descomprimir(&grupos, 10));
}
