use crate::bot::Bot;
use ff_rt::game::Match;

pub struct FullMatch {
    pub p1: Bot,
    pub p2: Bot,
    pub match_: Match,
}
