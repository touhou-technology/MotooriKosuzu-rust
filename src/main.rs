use tokio;
use dotenv;
use std::collections::HashMap;

pub mod notebook;

#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();


    notebook::Book::init();
}
