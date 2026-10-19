fn tipo_letra(c: char)-> &'static str{
    match c {
        'a'| 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => "vogal",
        _ => "consoante",
    }
}

fn main(){
    println!("{}", tipo_letra('b'))
}