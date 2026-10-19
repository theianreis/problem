fn par_impar(n: i32)-> &'static str{
    match n%2{
        0 => "par",
        _ => "impar",
    }
}

fn main(){
    println!("{}", par_impar(5))
}