use dotenv::dotenv;
mod interface;

fn main() {
    println!("Hello, world!");
    dotenv().ok();
    interface::init_interface();
}