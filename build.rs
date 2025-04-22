use dotenvy::{dotenv, dotenv_iter};

fn main() {
    dotenv().ok();
    for item in dotenv_iter().unwrap() {
        let (key, val) = item.unwrap();
        println!("cargo:rustc-env={}={}", key, val);
    }
}
