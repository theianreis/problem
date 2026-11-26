// Exercicio 144: Implemente uma arvore de segmentos iterativa para somas e atribuicoes pontuais.
struct Segmentos {
    n: usize,
    dados: Vec<i64>,
}
impl Segmentos {
    // Os valores e as somas intermediarias devem caber em i64.
    fn nova(valores: &[i64]) -> Self {
        let n = valores.len();
        let mut dados = vec![0; 2 * n];
        dados[n..].copy_from_slice(valores);
        for i in (1..n).rev() {
            dados[i] = dados[2 * i] + dados[2 * i + 1];
        }
        Self { n, dados }
    }
    fn atribuir(&mut self, indice: usize, valor: i64) -> bool {
        if indice >= self.n {
            return false;
        }
        let mut i = indice + self.n;
        self.dados[i] = valor;
        while i > 1 {
            i /= 2;
            self.dados[i] = self.dados[2 * i] + self.dados[2 * i + 1];
        }
        true
    }
    fn soma(&self, inicio: usize, fim: usize) -> Option<i64> {
        if inicio > fim || fim > self.n {
            return None;
        }
        let (mut a, mut b, mut soma) = (inicio + self.n, fim + self.n, 0);
        while a < b {
            if a % 2 == 1 {
                soma += self.dados[a];
                a += 1;
            }
            if b % 2 == 1 {
                b -= 1;
                soma += self.dados[b];
            }
            a /= 2;
            b /= 2;
        }
        Some(soma)
    }
}
fn main() {
    let mut arvore = Segmentos::nova(&[1, 2, 3, 4, 5]);
    assert_eq!(arvore.soma(1, 4), Some(9));
    assert!(arvore.atribuir(2, 10));
    assert_eq!(arvore.soma(1, 4), Some(16));
    assert_eq!(Segmentos::nova(&[]).soma(0, 0), Some(0));
    println!("Soma atualizada: {:?}", arvore.soma(1, 4));
}
