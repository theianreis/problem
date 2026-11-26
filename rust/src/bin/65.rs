// Exercicio 65: Ordene com insertion sort deslocando os elementos maiores.
fn insertion_sort(valores: &mut [i32]) {
    for i in 1..valores.len() {
        let atual = valores[i];
        let mut j = i;
        while j > 0 && valores[j - 1] > atual {
            valores[j] = valores[j - 1];
            j -= 1;
        }
        valores[j] = atual;
    }
}
fn main() {
    let mut valores = [5, 2, 4, 1];
    insertion_sort(&mut valores);
    assert_eq!(valores, [1, 2, 4, 5]);
    println!("Insertion sort: {valores:?}");
}
