#![allow(unused)]

use b_error::{BResult, BError};
use crate::bot_id::BotId;
use crate::bot_name::BotName;
use crate::bot_hash::BotHash;
use crate::bot_exe::BotExe;
use crate::bot::Bot;
use crate::types::FullMatch;

pub const MAX_BIN_SIZE: usize = 1024 * 1000;

pub struct Store;

impl Store {
    pub fn store_bot(&self, name: BotName, exe: &BotExe) -> BResult<Bot> {
        if exe.0.len() > MAX_BIN_SIZE {
            return Err(BError::new(
                format!("exe size {}B exceeds max size {}B",
                        exe.0.len(), MAX_BIN_SIZE)
            ));
        }

        let id = BotId::new();
        let name = name;
        let hash = BotHash::from_exe(exe);

        panic!();

        Ok(Bot { id, name, hash })
    }

    pub fn load_bot_meta(&self, id: BotId) -> BResult<Option<Bot>> {
        panic!()
    }

    pub fn load_bot_exe(&self, id: BotId) -> BResult<Option<BotExe>> {
        panic!()
    }

    pub fn load_random_match(&self) -> BResult<FullMatch> {
        panic!()
    }
}

