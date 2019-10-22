use crate::bot_id::BotId;
use crate::bot_name::BotName;
use crate::bot_hash::BotHash;

pub struct Bot {
    id: BotId,
    name: BotName,
    hash: BotHash,
}
