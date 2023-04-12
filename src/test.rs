use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog, FileFilter, Label, SpinButton};
use pbkdf2::pbkdf2;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, Write};
use std::process;

const SALT_LEN: usize = 16;
const IV_LEN: usize = 16;
const KEY_LEN: usize = 16;
const ITERATIONS: u32 = 100_000;
const TAGLINE: &str = "Secure your files with HexaCrypt!";

fn encrypt_file(password: &[u8], salt: &[u8], iv: &[u8], input_path: &str, output_path: &str) -> Result<(), String> {
    // Open input file and read contents
    let mut input_file = match File::open(input_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error opening input file: {}", err))
    };
    let mut input_data = Vec::new();
    if let Err(err) = input_file.read_to_end(&mut input_data) {
        return Err(format!("Error reading input file: {}", err));
    }

    // Generate encryption key
    let mut key = [0u8; KEY_LEN];
    pbkdf2::<Hmac<Sha256>>(password, salt, ITERATIONS, &mut key);

    // Encrypt data with AES-128 in CBC mode
    let cipher = Cbc::<Aes128, Pkcs7>::new_var(&key, iv).map_err(|err| format!("Error creating AES cipher: {}", err))?;
    let ciphertext = cipher.encrypt_vec(&input_data);

    // Write encrypted data to output file
    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating output file: {}", err))
    };
    if let Err(err) = output_file.write_all(&ciphertext) {
        return Err(format!("Error writing to output file: {}", err));
    }

    Ok(())
}

fn decrypt_file(password: &[u8], salt: &[u8], iv: &[u8], input_path: &str, output_path: &str) -> Result<(), String> {
    // Open input file and read contents
    let mut input_file = match File::open(input_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error opening input file: {}", err))
    };
    let mut input_data = Vec::new();
    if let Err(err) = input_file.read_to_end(&mut input_data) {
        return Err(format!("Error reading input file: {}", err));
    }

    // Generate decryption key
    let mut key = [0u8; KEY_LEN];
    pbkdf2::<Hmac<Sha256>>(password, salt, ITERATIONS, &mut key);

    // Decrypt data with AES-128 in CBC mode
    let cipher = Cbc::<Aes128, Pkcs7>::new_var(&key, iv).map_err(|err| format!("Error creating AES cipher: {}", err))?;
    let plaintext = cipher.decrypt_vec(&input_data).map_err(|err| format!("Error decrypting data: {}", err))?;

    // Write decrypted data to output file
    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating output file: {}", err))
    };
    if let Err(err) = output_file.write_all(&plaintext) {
        return Err(format!("Error writing to output file: {}", err));
    }

    Ok(())
}

