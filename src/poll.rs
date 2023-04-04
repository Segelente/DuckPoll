use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Poll {
    pub title: String,
    pub questions: Vec<Question>,
}

impl Poll {
    fn new(title: String, questions: Vec<Question>) -> Poll {
        Poll {
            title,
            questions,
        }
    }
}

impl Question {
    pub(crate) fn new(text: String, options: Vec<String>) -> Question {
        Question { text, options }
    }
}
