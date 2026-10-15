fn main(){
    let mut count =1;

    let result =loop {
        println!("Hello");

        if count == 3{
            break count;
        }

        count +=1;
    };

    println!("The loop stopped at:{}", result);
}