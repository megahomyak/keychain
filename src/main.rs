use base64::Engine;
use std::io::{Read, Write};

struct Keychains {
    secret: String,
    public: String,
}

fn join_keys(chain: Vec<Vec<String>>) -> String {
    chain.into_iter().flatten().collect::<Vec<_>>().join(":")
}

fn make_keychains() -> Keychains {
    let mut secret: Vec<Vec<String>> = Vec::new();
    let mut public: Vec<Vec<String>> = Vec::new();
    // Do all the key insertion thing
    public.reverse();
    Keychains {
        secret: join_keys(secret),
        public: join_keys(public),
    }
}

fn enc(
    mut input: Vec<u8>,
    self_secret_keychain: String,
    recipient_public_keychain: String,
) -> Vec<u8> {
}

fn dec(
    mut input: Vec<u8>,
    self_secret_keychain_path: String,
    recipient_public_keychain_path: String,
) -> Vec<u8> {
}

fn main() {
    fn read(fpath: &str) -> String {
        std::fs::read_to_string(fpath).unwrap()
    }
    let mut args = std::env::args();
    let command = args.next().unwrap();
    if command == "enc" {
        let self_secret_keychain_path = args.next().unwrap();
        let recipient_public_keychain_path = args.next().unwrap();
        let mut input = Vec::new();
        std::io::stdin().read_to_end(&mut input).unwrap();
        let output = enc(
            input,
            read(&self_secret_keychain_path),
            read(&recipient_public_keychain_path),
        );
        let output = base64::engine::general_purpose::STANDARD.encode(output);
        println!("{}", output);
    } else if command == "dec" {
        let self_secret_keychain_path = args.next().unwrap();
        let recipient_public_keychain_path = args.next().unwrap();
        let mut input = Vec::new();
        std::io::stdin().read_to_end(&mut input).unwrap();
        let input = String::from_utf8(input).unwrap();
        let input = input.trim();
        let input = base64::engine::general_purpose::STANDARD
            .decode(input)
            .unwrap();
        let output = dec(
            input,
            read(&self_secret_keychain_path),
            read(&recipient_public_keychain_path),
        );
        std::io::stdout().write_all(&output).unwrap();
    } else if command == "make_keychains" {
        let self_secret_keychain_path = args.next().unwrap();
        let self_public_keychain_path = args.next().unwrap();
        let keychains = make_keychains();
        std::fs::write(self_secret_keychain_path, keychains.secret).unwrap();
        std::fs::write(self_public_keychain_path, keychains.public).unwrap();
    } else {
        eprintln!("Unknown command");
    }
}
