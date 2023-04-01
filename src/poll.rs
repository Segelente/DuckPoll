struct Question {
    id: u32,
    text: String,
    options: Vec<String>,
}

struct Poll {
    title: String,
    id: u32,
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
