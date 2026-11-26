// Exercicio 69: Ordene u32 com radix sort LSD, processando um byte por passagem.
fn radix_sort(valores: &mut [u32]) {
    let mut auxiliar = vec![0; valores.len()];
    for deslocamento in [0, 8, 16, 24] {
        let mut contagens = [0_usize; 256];
        for &valor in valores.iter() {
            contagens[((valor >> deslocamento) & 255) as usize] += 1;
        }
        let mut posicoes = [0_usize; 256];
        for i in 1..256 {
            posicoes[i] = posicoes[i - 1] + contagens[i - 1];
        }
        // A distribuicao estavel preserva a ordem das passagens anteriores.
        for &valor in valores.iter() {
            let digito = ((valor >> deslocamento) & 255) as usize;
            auxiliar[posicoes[digito]] = valor;
            posicoes[digito] += 1;
        }
        valores.copy_from_slice(&auxiliar);
    }
}
fn main() {
    let mut valores = [256, 0, u32::MAX, 9, 256, 1];
    radix_sort(&mut valores);
    assert_eq!(valores, [0, 1, 9, 256, 256, u32::MAX]);
    println!("Radix sort: {valores:?}");
}
