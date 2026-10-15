fn main() {
    let neutrino: &str = "low";

    match neutrino {
        "low" => println!("No detect"),
        "heavy" => println!("Great, detect"),
        _ => println!("Erro"),
    }
}