CREATE TABLE answer(
    id INT NOT NULL,
    answer VARCHAR(255) NOT NULL,
    question_id INT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (question_id) REFERENCES question(id)
)