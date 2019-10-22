use std::fmt::{self, Formatter, Display};
use blake2_rfc::blake2b::blake2b;

pub struct BotHash(String);

impl BotHash {
    fn from_string(h: String) -> BotHash {
        assert!(hex::decode(&h).is_ok());
        assert_eq!(h.len(), 128);
        BotHash(h)
    }

    fn from_hash(h: &[u8]) -> BotHash {
        assert_eq!(h.len(), 64);
        let hash = hex::encode(h);
        BotHash(hash)
    }

    fn from_bin(bin: &[u8]) -> BotHash {
        let hash = blake2b(64, b"ferris", bin);
        let hash = hash.as_bytes();
        let hash = hex::encode(hash);
        BotHash(hash)
    }
}

impl Display for BotHash {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::BotHash;

    #[test]
    fn hex() {
        for _ in 0..1000 {
            let hash = BotHash::from_bin(b"this is a binary");
            let hashstr = format!("{}", hash);
            assert!(hex::decode(&hashstr).is_ok());
            assert_eq!(hashstr.len(), 128);
        }
    }
}
