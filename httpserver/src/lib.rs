// #![feature(custom_attribute)]
// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;

extern crate sink;

use sink::*;

pub struct HttpServer {}

pub struct HttpRequest {

}

pub type ETag = u32;

pub enum HttpErrors {
    NotFound,
}

impl ISink for HttpServer {
    type TInput = HttpRequest;
    type TResult = Result<ETag, HttpErrors>;

    fn handle(&self, input: HttpRequest) -> Result<ETag, HttpErrors> {
        // Do parsing, validation, etc here
        Ok (32)
    }
}

pub struct HttpSession {

}

// pub enum Commands {}

// impl<TSink> ISource for TSink//HttpServer
// where
//     TSink: ISink<TInput = Self::TOutput, TResult = Result<ETag, HttpErrors>>,
// {
//     type TOutput = Commands;
//     // type THandle = ();

//     // fn bind(self, sink: TSink) -> () {}
// }


// #[get("/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }

// fn main() {
//     rocket::ignite().mount("/hello", routes![hello]).launch();
// }