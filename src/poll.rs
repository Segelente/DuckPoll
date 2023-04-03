use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub title: String,
    pub id: u32,
    #[serde(skip)]
    questions: Vec<Question>,
}

impl Poll {
    fn new(title: String, id: u32, questions: Vec<Question>) -> Poll {
        Poll {
            title,
            id,
            questions,
        }
    }
}

impl Question {
    fn new(text: String, options: Vec<String>) -> Question {
        Question { text, options }
    }
}
