// Exercicio 80: Encontre todas as ocorrencias de uma substring com KMP em O(n + m).
fn kmp(texto: &str, padrao: &str) -> Vec<usize> {
    let (texto, padrao) = (texto.as_bytes(), padrao.as_bytes());
    // Convencao: padrao vazio corresponde a cada fronteira de caractere UTF-8.
    if padrao.is_empty() {
        return (0..=texto.len())
            .filter(|&i| i == texto.len() || texto[i] & 0xC0 != 0x80)
            .collect();
    }
    let mut prefixos = vec![0; padrao.len()];
    let mut j = 0;
    for i in 1..padrao.len() {
        while j > 0 && padrao[i] != padrao[j] {
            j = prefixos[j - 1];
        }
        if padrao[i] == padrao[j] {
            j += 1;
        }
        prefixos[i] = j;
    }
    let mut encontrados = Vec::new();
    j = 0;
    for (i, &byte) in texto.iter().enumerate() {
        while j > 0 && byte != padrao[j] {
            j = prefixos[j - 1];
        }
        if byte == padrao[j] {
            j += 1;
        }
        if j == padrao.len() {
            encontrados.push(i + 1 - j);
            j = prefixos[j - 1];
        }
    }
    encontrados
}
fn main() {
    assert_eq!(kmp("aaaa", "aa"), vec![0, 1, 2]);
    assert_eq!(kmp("á", ""), vec![0, 2]);
    println!("Ocorrencias em bytes: {:?}", kmp("banana", "ana"));
}
