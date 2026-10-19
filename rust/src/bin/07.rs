fn main() {
    let neutrino: &str = "low";

   /*
    controle de fluxo e correspondência de padrões (pattern matching) 
    que permite comparar um valor contra uma série de padrões e 
    */
    
    match neutrino {
        "low" => println!("No detect"),
        "heavy" => println!("Great, detect"),
        _ => println!("Erro"),
    }
}