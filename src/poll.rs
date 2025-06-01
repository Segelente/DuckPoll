use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Type};

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub text: String,
    pub answers: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub(crate) fn new(text: String, answers: Vec<String>) -> Question {
        Question { text, answers }
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
