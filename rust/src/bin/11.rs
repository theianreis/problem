fn main(){
    let mut num = 1;

    while num <= 10 {
    if num == 6 {
        break;
    }
    println!("Number: {}", num);
    num += 1;
    }
}