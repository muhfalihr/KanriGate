use argon2::{password_hash::{SaltString, PasswordHasher}, Argon2};
use rand::thread_rng;

fn main() {
    let password = "password";
    let salt = SaltString::generate(&mut thread_rng());
    let argon2 = Argon2::default();
    
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => println!("\nBERHASIL! Berikut adalah hash Argon2 Anda:\n\n{}\n", hash.to_string()),
        Err(e) => eprintln!("Gagal melakukan hashing: {}", e),
    }
}
