fn main(){
    let temperature_celciuls: i128 = 120; // grau celcius
    let conversion_celciuls_for_kelvin = temperature_celciuls + 273;
    let temperature_kelvin = conversion_celciuls_for_kelvin;

    println!("The temperature in celcius is {} ,as for the temperature in temperature_kelvin is {}",temperature_celciuls, temperature_kelvin);
}
