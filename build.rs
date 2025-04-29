use dotenvy::{Error, dotenv, dotenv_iter};

fn env_error(e: Error) {
    eprintln!("環境変数の読み込みに失敗しました: {}", e);
    std::process::exit(1);
}

fn main() {
    match dotenv() {
        Ok(_) => {}
        Err(e) => {
            env_error(e);
        }
    }

    for item in dotenv_iter().unwrap_or_else(|e| {
        env_error(e);
        unreachable!();
    }) {
        match item {
            Ok((key, val)) => {
                println!("cargo:rustc-env={}={}", key, val);
            }
            Err(e) => {
                env_error(e);
            }
        }
    }
}
