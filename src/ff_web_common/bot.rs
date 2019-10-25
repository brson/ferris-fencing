use crate::bot_id::BotId;
use crate::bot_name::BotName;
use crate::bot_hash::BotHash;

#[derive(Serialize, Deserialize)]
pub struct Bot {
    pub id: BotId,
    pub name: BotName,
    pub hash: BotHash,
}
