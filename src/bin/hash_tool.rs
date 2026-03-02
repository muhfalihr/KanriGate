use argon2::{password_hash::{SaltString, PasswordHasher}, Argon2};
use rand::thread_rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let password = if args.len() > 1 {
        &args[1]
    } else {
        println!("Usage: cargo run --bin hash_tool <password>");
        println!("Using default password 'admin' for demo purposes...\n");
        "admin"
    };

    let salt = SaltString::generate(&mut thread_rng());
    let argon2 = Argon2::default();
    
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => {
            println!(
                "SUCCESS! Here is the Argon2 hash for password '{}':\n",
                password
            );
            println!("{}\n", hash.to_string());
            println!(
                "Please copy the hash above into your .env file as APP_ADMIN_PASSWORD_HASH"
            );
        },
        Err(e) => eprintln!("Failed to hash password: {}", e),
    }
}