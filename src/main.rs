enum Key {
    RSA { key: String },
}

struct Keychain {
    keys: Vec<Key>,
}

enum ParsingError {
    UnknownAlgo { name: String },
    RSAKeyMissing,
}

impl std::str::FromStr for Keychain {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = s.split(':');
        let mut keys = Vec::new();
        while let Some(algo_name) = items.next() {
            match algo_name {
                "rsa" => {
                    let key = items.next().ok_or(ParsingError::RSAKeyMissing)?;
                    keys.push(Key::RSA { key: key.to_owned() });
                }
                _ => {
                    return Err(ParsingError::UnknownAlgo {
                        name: algo_name.to_owned(),
                    })
                }
            }
        }
        Ok(Self { keys })
    }
}
impl std::fmt::Display for Keychain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, key) in self.keys.iter().enumerate() {
            if index != 0 {
                f.write_str(":")?;
            }
            match key {
                Key::RSA { key } => {
                    f.write_str("rsa:")?;
                    f.write_str(key)?;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
