use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Type};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, sqlx::Decode, sqlx::Encode)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, sqlx::Type, sqlx::Decode, sqlx::Encode)]
pub struct Poll {
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, sqlx::Type, sqlx::Decode, sqlx::Encode)]
pub struct Vote {
    pub question_title: String,
    pub option_votes: Vec<i32>,
}

impl Poll {
    fn new(title: String, questions: Vec<Question>) -> Poll {
        Poll { title, questions }
    }
}

impl Question {
    pub(crate) fn new(text: String, options: Vec<String>) -> Question {
        Question { text, options }
    }
}

impl Vote {
    pub(crate) fn new(question_title: String, option_votes: Vec<i32>) -> Vote {
        Vote {
            question_title,
            option_votes,
        }
    }
}
