fn main(){
    let numero:i128 = 5;

    let mut count: i128 = 1;
    
    let mut n: i128 = numero;


    while n > 1  {
        count *= n;
        n -= 1;
    }

    println!("fatorial de {} = {}",numero , count );

}