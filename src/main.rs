#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::io::Read;


#[get("/")]
fn hello() -> String{
    let path = Path::new("api.json");
    let display = path.display();

    println!("{:?} {:?}",path,display);

    let mut file = match File::create(path){
        Ok(file) => file,
        Err(_) => panic!("file creating error"),  
    };

    match reqwest::get("https://api.openweathermap.org/data/2.5/weather?q=karachi&Apikey=4970e4f266675063af77ad454f45ebd6&units=metric"){
        Ok(mut response) => {
            match response.text(){
                Ok(text) => match file.write_all(text.as_bytes()){
                    Ok(_) => println!("file has been written"),
                    Err(e) => println!("the error is : {}",e),
                }
                Err(_) => println!("server is not responding"),
            }
        }
        Err(_) => println!("conection is not stablished"),
    }

    let mut file = match File::open(&path){
        Ok(file) => file,
        Err(e) => panic!("file open error : {}",e),
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let json = Json::from_str(&buffer).unwrap();

    let result = format!("the tempratue of karachi is : {}",json.find_path(&["main"]).unwrap());
    result

}

fn main(){
    rocket::ignite().mount("/",routes![hello]).launch();
}




















