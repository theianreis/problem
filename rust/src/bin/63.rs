// Exercicio 63: Ordene com bubble sort e pare cedo quando nao houver trocas.
fn bubble_sort(valores: &mut [i32]) {
    for fim in (1..valores.len()).rev() {
        let mut trocou = false;
        for i in 0..fim {
            if valores[i] > valores[i + 1] {
                valores.swap(i, i + 1);
                trocou = true;
            }
        }
        if !trocou {
            break;
        }
    }
}
fn main() {
    let mut valores = [5, 1, 4, 2, 2];
    bubble_sort(&mut valores);
    assert_eq!(valores, [1, 2, 2, 4, 5]);
    println!("Bubble sort: {valores:?}");
}
