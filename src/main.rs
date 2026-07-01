use dotenv::dotenv;
mod interface;
mod service;

fn main() {
    println!("Hello, world!");
    dotenv().ok();
    //interface::init_interface();

    service::establish_connection();
}