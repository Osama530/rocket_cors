#[get("/")]
pub fn hello()->String{
 format!("hello web")
}

#[get("/primes")]
pub fn prim_serise() -> String {
    let mut series: Vec<i32> = Vec::new(); //empty vector initialization
    let mut count = 0;

    for num in 2..100 {
        count = 0;
        for i in 2..num {
            if num % i == 0 {
                count += 1;
                break;
            }
        }
        if count == 0 && num != 1 {
            series.push(num);
        }
    }
    format!("{:?}",series)
}

#[post("/<num>")]
pub fn is_prime (num: i32)-> String{
    let mut flag = 1;
    if num <= 1 {
        format!("the number {} you enter is not a prime number",num)
    }
    else {
        for i in 2..num/2+1 {
            if num%i == 0 {
                flag = 0;
                break;
            }
        }
        if flag==0 {
            format!("the number {} you enter is not a prime number",num)
        }
        else {
            format!("the number {} you enter is a prime number",num)
        }
    }
}

use rand::Rng;

#[get("/worksheet")]
pub fn genrate_worksheet()-> String {

    let mut series: Vec<String> = Vec::new(); //empty vector initialization
   
    for i in 0..100 {

        let mut rang = rand::thread_rng();
        let num_1 = rang.gen_range(0,100).to_string();
        let num_2 = rang.gen_range(0,100).to_string();
        let qustion = num_1 + "__" + &num_2;
        series.push(qustion);
    };
    format!("{:#?}",series)
}

// use rocket_contrib::serve::StaticFiles;
// use rocket::response::NamedFile;

use rocket::response::content;
#[post("/workout?<answer>")]
pub fn less_greater_workout(answer: String){
    let response = content::Html("<h1>Hello, world!</h1>");
}