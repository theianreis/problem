fn main(){
    let score1:f64 = 3.0;
    let score2: f64 = 4.5;
    let score3: f64 = 8.0;
    let score4: f64 = 10.0;

    let sum_score: f64 = score1+score2+score3+score4;

    let median = sum_score/4.0;

    if median <= 8.0{
        print!("Reproved");
    }else {
        print!("Aproved");
    }
}