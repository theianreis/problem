// Exercicio 114: Agrupe palavras anagramas usando a sequencia ordenada de caracteres como chave.
use std::collections::BTreeMap;
fn agrupar(palavras: &[&str]) -> Vec<Vec<String>> {
    let mut grupos: BTreeMap<Vec<char>, Vec<String>> = BTreeMap::new();
    for &palavra in palavras {
        let mut chave: Vec<char> = palavra.chars().collect();
        chave.sort_unstable();
        grupos.entry(chave).or_default().push(palavra.into());
    }
    grupos.into_values().collect()
}
fn main() {
    // Comparacao exata, sensivel a caixa e acentos.
    let grupos = agrupar(&["amor", "roma", "rust", "ramo"]);
    assert_eq!(grupos.len(), 2);
    assert!(grupos.contains(&vec!["amor".into(), "roma".into(), "ramo".into()]));
    println!("Grupos: {grupos:?}");
}
