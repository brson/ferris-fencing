use std::fmt::{self, Formatter, Display};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Serialize, Deserialize)]
pub struct BotName(String);

impl BotName {
    pub fn new(name: String) -> BotName {
        BotName(sanitize(name))
    }
}

fn sanitize(name: String) -> String {
    let mut g = name.graphemes(true);
    let g = g.next().unwrap_or("?");
    g.to_string()
}

impl Display for BotName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
