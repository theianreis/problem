// Exercicio 147: Resolva Sudoku 9x9 por backtracking, validando as pistas iniciais.
type Tabuleiro = [[u8; 9]; 9];
fn permitido(tabuleiro: &Tabuleiro, linha: usize, coluna: usize, numero: u8) -> bool {
    for i in 0..9 {
        if tabuleiro[linha][i] == numero || tabuleiro[i][coluna] == numero {
            return false;
        }
    }
    let (bloco_l, bloco_c) = (linha / 3 * 3, coluna / 3 * 3);
    (bloco_l..bloco_l + 3).all(|l| (bloco_c..bloco_c + 3).all(|c| tabuleiro[l][c] != numero))
}
fn explorar(tabuleiro: &mut Tabuleiro) -> bool {
    for l in 0..9 {
        for c in 0..9 {
            if tabuleiro[l][c] != 0 {
                continue;
            }
            for numero in 1..=9 {
                if permitido(tabuleiro, l, c, numero) {
                    tabuleiro[l][c] = numero;
                    if explorar(tabuleiro) {
                        return true;
                    }
                    tabuleiro[l][c] = 0;
                }
            }
            return false;
        }
    }
    true
}
fn resolver(tabuleiro: &mut Tabuleiro) -> bool {
    for l in 0..9 {
        for c in 0..9 {
            let numero = tabuleiro[l][c];
            if numero == 0 {
                continue;
            }
            if numero > 9 {
                return false;
            }
            tabuleiro[l][c] = 0;
            let valido = permitido(tabuleiro, l, c, numero);
            tabuleiro[l][c] = numero;
            if !valido {
                return false;
            }
        }
    }
    explorar(tabuleiro)
}
fn main() {
    let mut tabuleiro = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let pistas = tabuleiro;
    assert!(resolver(&mut tabuleiro));
    for l in 0..9 {
        for c in 0..9 {
            assert!((1..=9).contains(&tabuleiro[l][c]));
            if pistas[l][c] != 0 {
                assert_eq!(tabuleiro[l][c], pistas[l][c]);
            }
            let numero = tabuleiro[l][c];
            tabuleiro[l][c] = 0;
            assert!(permitido(&tabuleiro, l, c, numero));
            tabuleiro[l][c] = numero;
        }
    }
    let mut invalido = [[1; 9]; 9];
    assert!(!resolver(&mut invalido));
    for linha in tabuleiro {
        println!("{linha:?}");
    }
}
