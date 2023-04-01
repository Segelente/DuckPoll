use serde::{Deserialize, Serialize};
struct Question {
    id: u32,
    text: String,
    options: Vec<String>,
}
#[derive(Serialize, Deserialize)]
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
    fn new(id: u32, text: String, options: Vec<String>) -> Question {
        Question { id, text, options }
    }
}
