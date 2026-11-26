// Exercicio 48: Encontre o segundo maior valor distinto em uma unica passagem.
fn segundo_maior(valores: &[i32]) -> Option<i32> {
    let (mut maior, mut segundo) = (None, None);
    for &valor in valores {
        match maior {
            None => maior = Some(valor),
            Some(m) if valor > m => {
                segundo = maior;
                maior = Some(valor);
            }
            Some(m) if valor < m && segundo.is_none_or(|s| valor > s) => segundo = Some(valor),
            _ => {}
        }
    }
    segundo
}
fn main() {
    assert_eq!(segundo_maior(&[9, 9, 4, 7]), Some(7));
    assert_eq!(segundo_maior(&[2, 2]), None);
    println!("Segundo maior: {:?}", segundo_maior(&[9, 9, 4, 7]));
}
