enum Algo {
    
}

struct Key {
    algo: String,
    value: String,
}

struct Keychain {
    keys: Vec<Key>,
}

impl Keychain {
    pub fn encrypt(message: Vec<u8>) {

    }
    pub fn decrypt(message: Vec<u8>) {

    }
}
impl std::str::FromStr for Keychain {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}
impl std::fmt::Display for Keychain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, key) in self.keys.iter().enumerate() {
            if index != 0 {
                f.write_str(":")?;
            }
            f.write_str(&key.algo)?;
            f.write_str("_")?;
            f.write_str(&key.value)?;
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
