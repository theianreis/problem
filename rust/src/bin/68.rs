// Exercicio 68: Ordene bytes com counting sort, usando 256 contadores.
fn counting_sort(valores: &[u8]) -> Vec<u8> {
    let mut contagens = [0_usize; 256];
    for &valor in valores {
        contagens[usize::from(valor)] += 1;
    }
    let mut resultado = Vec::with_capacity(valores.len());
    for (valor, &quantidade) in contagens.iter().enumerate() {
        resultado.extend(std::iter::repeat_n(valor as u8, quantidade));
    }
    resultado
}
fn main() {
    let resultado = counting_sort(&[4, 0, 255, 4, 2]);
    assert_eq!(resultado, vec![0, 2, 4, 4, 255]);
    println!("Counting sort: {resultado:?}");
}
