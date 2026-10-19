fn fatorial(valor: u128) -> u128{
    match valor{
        0 => 1,
        1.. => (1..=valor).product(),
    }
}

fn main(){
    println!("{}", fatorial(5));
}