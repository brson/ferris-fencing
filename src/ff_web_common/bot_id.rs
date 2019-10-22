use std::fmt::{self, Formatter, Display};

pub struct BotId(u64);

impl BotId {
    pub fn new() -> BotId {
        BotId(rand::random())
    }
        
}

impl Display for BotId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::BotId;

    #[test]
    fn hex() {
        for _ in 0..1000 {
            let id = BotId::new();
            let idstr = format!("{}", id);
            assert!(hex::decode(&idstr).is_ok());
            assert_eq!(idstr.len(), 16);
        }
    }
}
