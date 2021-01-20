// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] 
// extern crate rocket;
// extern crate rocket_cors;

// //#[macro_use]
// //extern crate serde_derive;
// #[macro_use]
// extern crate rocket_contrib;

// use rocket_contrib::json::{Json, JsonValue};
// use rocket_cors::{
//     AllowedHeaders,
//     AllowedOrigins,
//     Cors,
//     CorsOptions,
//     Error };
    
// use rand::Rng;

// use rocket::response::content;
// use rocket::http::Method;
// use rocket::State;


// type ID = usize;

// #[derive(Debug, PartialEq, Eq)]
// struct Message {
//     id: ID,
//     content: String,
// }

// #[get("/")]
// fn hello() -> JsonValue { 
//     json!({
//         "hello" : "world"
//     })
// }

// #[get("/")]
// fn temperature()-> JsonValue {
//     let ran_num = genrate_random();
//     json!({
//         "Temperature" : ran_num
//     })
    
// }


// #[get("/")]
// fn light()-> JsonValue {
//     let ran_num = genrate_random();
//     json!({
//         "Light" : ran_num
//     })
// }

// #[derive(FromForm)]
// struct Log{
//     temperature: u32,
//     light: u32
// }

// #[get("/", data = "<Log>")]
// fn log_data(values: Json<Log>)-> String{
//     format!("log values: {} {}",values.light,values.temperature)
// }


// fn main() {
//     rocket::ignite()
//         .mount("/", routes![hello, temperature, light, log_data])
//         .attach(make_cors())
//         .launch();
// }

// fn make_cors() -> Cors {
//     let allowed_origins = AllowedOrigins::some_exact(&[
//         "http://localhost:8000/",
//     ]);

//     CorsOptions {
//         allowed_origins,
//         allowed_methods: vec![Method::Get, Method::Post]
//             .into_iter()
//             .map(From::from)
//             .collect(),
//         allowed_headers: AllowedHeaders::some(&[
//             "Authorization",
//             "Accept",
//             "Access-Control-Allow-Origin",
//         ]),
//         allow_credentials: true,
//         ..Default::default()
//     }
//     .to_cors()
//     .expect("error building cors")
// }

// fn genrate_random()->u32{
//     let mut rang = rand::thread_rng();
//     let num = rang.gen_range(0,100);
//     num
// }

// /******************* Rocket exersice *******************************/

// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] 
// extern crate rocket;
// #[macro_use]
// extern crate rocket_contrib;

// use rocket::response::content::Html;
// use rocket::request::Form;

// fn main(){
//     rocket::ignite().mount("/", routes![hello,html,add, form, user_post]).launch();

// }

// //returning string to browser
// #[get("/")]
// fn hello()->String{
//     "helow world".to_string()
// }

// //returning HTML to browser
// #[get("/home")]
// fn html()->Html<&'static str>{
//     Html(r#"
//     <html>
//         <head>
//             <title> Home </title>
//         </head>
//         <body>
//             <h1> Hello world </h1>
//         </body>
//     </html>"#
//     )
// }

// //getting a number from client return added value
// #[get("/add/<number>")]
// fn add(number: i32)->String{
//     let result = 32 + number;
//     result.to_string()
// }

// //creating an html form
// #[get("/form")]
// fn form()->Html<&'static str>{
//     Html(r#"
//     <html>
//         <head>
//             <title> Home </title>
//         </head>
//         <body>
//             <h1> POST Request </h1>
//             <h2> type a number that you want to add to 32 </h2>
//             <br>
//                 <form method="POST" action="/addUsingPost">
//                     <input type="number" placeholder = "Any Number" name = "number">
//                     <br></br>
//                     <input type = "submit" value = "send">
//                 </form>
//             </br>
//         </body>
//     </html>"#
//     )
// }

// #[derive(FromForm)]
// struct UserForm {
//     number: i32
// }

// //rerouting the form data /addUsingPost
// #[post("/addUsingPost", data = "<form>")]
// fn user_post(form: Form<UserForm>)->String{
//     let result = 34 + form.number;
//     format!("result = {}",result)
// }

//..................Rocket teaching web ....................//
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rand;
extern crate rocket_contrib;

mod routes;

use crate::routes::route;
use rand::Rng;
// use rocket_contrib::serve::StaticFiles;
// use rocket::response::NamedFile;
use rocket::response::content;

fn main() {
    rocket::ignite()
        .mount("/", routes![route::hello, route::prim_serise,
                        route::is_prime, route::genrate_worksheet,
                        route::less_greater_workout])
        .launch();
}