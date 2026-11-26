// Exercicio 150: Integre mapa, heap e arvore para codificar e decodificar texto com Huffman.
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};
struct No {
    caractere: Option<char>,
    filhos: Option<(usize, usize)>,
}
struct Huffman {
    nos: Vec<No>,
    raiz: Option<usize>,
    codigos: BTreeMap<char, Vec<bool>>,
}
impl Huffman {
    fn novo(texto: &str) -> Self {
        let mut frequencias = BTreeMap::new();
        for c in texto.chars() {
            *frequencias.entry(c).or_insert(0_usize) += 1;
        }
        let mut nos = Vec::new();
        let mut heap = BinaryHeap::new();
        for (c, frequencia) in frequencias {
            let indice = nos.len();
            nos.push(No {
                caractere: Some(c),
                filhos: None,
            });
            heap.push(Reverse((frequencia, indice)));
        }
        while heap.len() > 1 {
            let Reverse((fa, a)) = heap.pop().unwrap();
            let Reverse((fb, b)) = heap.pop().unwrap();
            let indice = nos.len();
            nos.push(No {
                caractere: None,
                filhos: Some((a, b)),
            });
            heap.push(Reverse((fa + fb, indice)));
        }
        let raiz = heap.pop().map(|Reverse((_, i))| i);
        let mut resultado = Self {
            nos,
            raiz,
            codigos: BTreeMap::new(),
        };
        if let Some(raiz) = raiz {
            resultado.gerar_codigos(raiz, &mut Vec::new());
        }
        resultado
    }
    fn gerar_codigos(&mut self, indice: usize, prefixo: &mut Vec<bool>) {
        if let Some(c) = self.nos[indice].caractere {
            // Alfabeto com um simbolo recebe o codigo 0, preservando a quantidade.
            let codigo = if prefixo.is_empty() {
                vec![false]
            } else {
                prefixo.clone()
            };
            self.codigos.insert(c, codigo);
        } else if let Some((a, b)) = self.nos[indice].filhos {
            prefixo.push(false);
            self.gerar_codigos(a, prefixo);
            prefixo.pop();
            prefixo.push(true);
            self.gerar_codigos(b, prefixo);
            prefixo.pop();
        }
    }
    fn codificar(&self, texto: &str) -> Option<Vec<bool>> {
        let mut bits = Vec::new();
        for c in texto.chars() {
            bits.extend_from_slice(self.codigos.get(&c)?);
        }
        Some(bits)
    }
    fn decodificar(&self, bits: &[bool]) -> Option<String> {
        let Some(raiz) = self.raiz else {
            return if bits.is_empty() {
                Some(String::new())
            } else {
                None
            };
        };
        if let Some(c) = self.nos[raiz].caractere {
            return if bits.iter().any(|&b| b) {
                None
            } else {
                Some(std::iter::repeat_n(c, bits.len()).collect())
            };
        }
        let mut texto = String::new();
        let mut atual = raiz;
        for &bit in bits {
            let (a, b) = self.nos[atual].filhos?;
            atual = if bit { b } else { a };
            if let Some(c) = self.nos[atual].caractere {
                texto.push(c);
                atual = raiz;
            }
        }
        if atual == raiz {
            Some(texto)
        } else {
            None
        }
    }
}
fn main() {
    for texto in ["", "aaaa", "banana com café"] {
        let huffman = Huffman::novo(texto);
        let bits = huffman.codificar(texto).unwrap();
        assert_eq!(huffman.decodificar(&bits).as_deref(), Some(texto));
        let codigos: Vec<_> = huffman.codigos.values().collect();
        for (i, a) in codigos.iter().enumerate() {
            for (j, b) in codigos.iter().enumerate() {
                if i != j {
                    assert!(!b.starts_with(a));
                }
            }
        }
        assert!(huffman.codificar("☃").is_none());
        println!(
            "{texto:?}: {} bits de dados (UTF-8 original: {}).",
            bits.len(),
            texto.len() * 8
        );
    }
    // Representacao didatica com bool; tamanho logico exclui arvore e metadados.
    // Um compressor de arquivos tambem precisa empacotar os bits e salvar a arvore.
}
