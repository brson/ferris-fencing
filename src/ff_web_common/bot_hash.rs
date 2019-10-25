use crate::bot_exe::BotExe;
use std::fmt::{self, Formatter, Display};
use blake2_rfc::blake2b::blake2b;

#[derive(Serialize, Deserialize)]
pub struct BotHash(String);

impl BotHash {
    pub fn from_string(h: String) -> BotHash {
        assert!(hex::decode(&h).is_ok());
        assert_eq!(h.len(), 128);
        BotHash(h)
    }

    pub fn from_exe(exe: &BotExe) -> BotHash {
        let hash = blake2b(64, b"ferris", &exe.0);
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
    use crate::bot_exe::BotExe;

    #[test]
    fn hex() {
        for _ in 0..1000 {
            let exe = BotExe(b"this is a binary".to_vec());
            let hash = BotHash::from_exe(&exe);
            let hashstr = format!("{}", hash);
            assert!(hex::decode(&hashstr).is_ok());
            assert_eq!(hashstr.len(), 128);
        }
    }
}
