fn main() {
    const PI: f64 = 3.14159265359; 
    let r: f64 = 4.0; 
    let volume = (4.0 / 3.0) * PI * r.powi(3);

    println!("Volume da esfera é dado por V = {:.2}", volume);
}