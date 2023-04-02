use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Question {
    text: String,
    options: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub title: String,
    pub id: u32,
    #[serde(skip)]
    question: Vec<Question>,
}

impl Poll {
    fn new(title: String, id: u32, question: Vec<Question>) -> Poll {
        Poll {
            title,
            id,
            question,
        }
    }
}

impl Question {
    fn new(text: String, options: Vec<String>) -> Question {
        Question { text, options }
    }
}
