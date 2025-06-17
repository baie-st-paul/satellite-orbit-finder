use reqwest::{self};
use std::sync::Arc;
use dotenv::dotenv;
mod interface;

fn main(){
    println!("Hello, world!");
    dotenv().ok();
   // send_request();
    interface::init_interface();
}
 
fn send_request() {
    //cookie
    let cookie_store = Arc::new(reqwest::cookie::Jar::default());
    let client = reqwest::blocking::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build().unwrap();

    let login_data = [
        ("identity", std::env::var("identity").ok().unwrap()),
        ("password", std::env::var("password").ok().unwrap())
    ];

    let res1 = client.post("https://www.space-track.org/ajaxauth/login").form(&login_data).send();
    println!("{}", res1.as_ref().unwrap().status());
    println!("{}", res1.unwrap().text().unwrap());

    //let url = "https://www.space-track.org/basicspacedata/query/class/tle_latest/ORDINAL/1/limit/1000/OBJECT_TYPE/DEBRIS/format/json";

    //let res2 = client.get(url).send().unwrap();
    //println!("{}", res2.text().unwrap())   
}